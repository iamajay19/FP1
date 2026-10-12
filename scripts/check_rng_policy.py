"""P2A-BUILD-003/004: actual offline Cargo/rustdoc rejections and cfg defense.

Only disposable public fixtures. Never load response files in production guards.
The compile-time controls compile the exact root guard without unrelated modules.
"""
import os
import json
import re
import shlex
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
APPROVED = '--cfg=getrandom_backend="linux_getrandom"'
BACKENDS = {"custom", "linux_getrandom", "linux_raw", "rdrand", "rndr",
            "efi_rng", "windows_legacy", "unsupported", "extern_impl"}
CUSTOM = '--cfg=getrandom_backend="custom"'
ALTERNATE = '--cfg=getrandom_backend="linux_raw"'


def config_flags(key, flags):
    # TOML accepts this JSON string-array representation; passed as argv, no shell.
    return ["--config", key + "=" + json.dumps(flags)]


def rejected(command, env, name):
    for key in ("RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "RUSTDOCFLAGS",
                "CARGO_ENCODED_RUSTDOCFLAGS", "RUSTDOC"):
        if key in env:
            print(f"{key}={env[key]!r}", flush=True)
    print(f"$ {shlex.join(map(str, command))}", flush=True)
    result = subprocess.run(command, cwd=ROOT, env=env, capture_output=True, text=True)
    # A missing fixture/compiler/tool is not evidence of a policy rejection.
    diagnostics = result.stdout + result.stderr
    policy = ("Phase 2A" in diagnostics or "could not compile `getrandom`" in diagnostics
              or "multiple input filenames" in diagnostics)
    assert result.returncode != 0 and policy, (name, result.returncode, diagnostics)
    lines = [line for line in diagnostics.splitlines()
             if "Phase 2A" in line or "could not compile `getrandom`" in line
             or "multiple input filenames" in line]
    print(f"{name}: rejected exit={result.returncode}; " + " / ".join(lines), flush=True)


def compile_guard(work, env):
    backend_source = (ROOT / ".deps/phase2-vendor/getrandom-0.4.3/src/backends.rs").read_text()
    recognized = set(re.findall(r'getrandom_backend\s*=\s*"([^"]+)"', backend_source))
    assert recognized == BACKENDS, recognized
    lib = (ROOT / "trusted-client/src/lib.rs").read_text()
    guard = lib.split("pub mod foundation;", 1)[0]
    forbidden = set(re.findall(r'getrandom_backend\s*=\s*"([^"]+)"', guard))
    assert forbidden == BACKENDS, forbidden
    build = (ROOT / "trusted-client/build.rs").read_text()
    check_cfg = next(line for line in build.splitlines() if "cargo:rustc-check-cfg" in line)
    assert set(re.findall(r'\\"([^"\\]+)\\"', check_cfg)) == BACKENDS
    source = work / "guard.rs"
    source.write_text(guard)
    command = ["rustc", "--edition=2024", "--crate-type=lib", "--emit=metadata",
               source, "-o", work / "guard.rmeta", APPROVED]
    subprocess.run(command, cwd=ROOT, env=env, check=True)
    for backend in sorted(BACKENDS - {"linux_getrandom"}):
        rejected([*command, f'--cfg=getrandom_backend="{backend}"'], env,
                 f"P2A-BUILD-004 effective cfg {backend}")
    rsp = work / "compile-hidden.rsp"
    rsp.write_text('--cfg=getrandom_backend="custom"\n')
    rejected([*command, "@" + str(rsp)], env, "P2A-BUILD-004 expanded hidden CUSTOM")
    doc = ["rustdoc", "--edition=2024", "--test", source, APPROVED]
    subprocess.run(doc, cwd=ROOT, env=env, check=True)
    for backend in sorted(BACKENDS - {"linux_getrandom"}):
        rejected([*doc, f'--cfg=getrandom_backend="{backend}"'], env,
                 f"P2A-BUILD-004 rustdoc effective cfg {backend}")
    rejected([*doc, "@" + str(rsp)], env, "P2A-BUILD-004 rustdoc expanded hidden CUSTOM")
    # Sensitivity: removing only the independent exclusivity guard must permit
    # this compilation. This never builds/links a runtime/custom RNG fixture.
    source.write_text(guard.split("// Independent of build.rs", 1)[0])
    subprocess.run([*command, "@" + str(rsp)], cwd=ROOT, env=env, check=True)
    subprocess.run([*doc, "@" + str(rsp)], cwd=ROOT, env=env, check=True)
    print("P2A-BUILD-004 PASS: rustc AND rustdoc each reject 8 alternate cfgs "
          "+ hidden CUSTOM; both omitted-exclusivity negative controls accepted", flush=True)


def controls(env):
    with tempfile.TemporaryDirectory(prefix="fp1-rng-policy-") as directory:
        work = Path(directory)
        compile_guard(work, env)
        files = {}
        for name, contents in {
            "custom": '--cfg=getrandom_backend="custom"\n',
            "alternate": '--cfg=getrandom_backend="linux_raw"\n',
            "duplicate": APPROVED + "\n",
            "conflict": '--cfg=getrandom_backend="unsupported"\n',
        }.items():
            files[name] = work / (name + ".rsp")
            files[name].write_text(contents)
        files["nested"] = work / "outer.rsp"
        files["nested"].write_text("@" + str(files["custom"]) + "\n")
        # Hold approved dependencies/target separate from adversarial flags.
        base = dict(env, CARGO_TARGET_DIR=str(work / "target"))
        for key in ["RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "RUSTDOCFLAGS",
                    "CARGO_ENCODED_RUSTDOCFLAGS"]:
            base.pop(key, None)
        check = ["cargo", "check", "--offline", "--locked", "--manifest-path",
                 ROOT / "trusted-client/Cargo.toml"]
        cases = []
        for name, path in files.items():
            cases.append(("response-" + name, {"RUSTFLAGS": APPROVED + " @" + str(path)}, []))
        cases += [
            ("encoded-response", {"CARGO_ENCODED_RUSTFLAGS":
                                  APPROVED + "\x1f@" + str(files["custom"])}, []),
            ("custom", {"RUSTFLAGS": CUSTOM}, []),
            ("approved-custom", {"RUSTFLAGS": APPROVED + " " + CUSTOM}, []),
            ("approved-alternate", {"RUSTFLAGS": APPROVED + " " + ALTERNATE}, []),
            ("missing", {"RUSTFLAGS": ""}, []),
            ("duplicate", {"RUSTFLAGS": APPROVED + " " + APPROVED}, []),
            ("ordering", {"RUSTFLAGS": '@' + str(files["custom"]) + " " + APPROVED}, []),
            ("whitespace", {"CARGO_ENCODED_RUSTFLAGS":
                            '--cfg\x1fgetrandom_backend = "linux_getrandom"'}, []),
            ("encoded-conflict", {"CARGO_ENCODED_RUSTFLAGS": APPROVED + "\x1f" + CUSTOM}, []),
            ("cli-response", {}, config_flags("build.rustflags",
                                              [APPROVED, "@" + str(files["custom"])])),
            ("target-response", {}, ["--target", "x86_64-unknown-linux-gnu",
                                     *config_flags("target.x86_64-unknown-linux-gnu.rustflags",
                                                   [APPROVED, "@" + str(files["custom"])])]),
            ("launcher-override", {"RUSTDOC": "rustdoc"}, []),
            ("unknown", {"RUSTFLAGS": APPROVED + ' --cfg=getrandom_backend="not_a_backend"'}, []),
            ("split-custom", {"CARGO_ENCODED_RUSTFLAGS":
                              '--cfg\x1fgetrandom_backend="custom"\x1f--cfg\x1f'
                              'getrandom_backend="linux_getrandom"'}, []),
            ("quoted", {"CARGO_ENCODED_RUSTFLAGS":
                        APPROVED + '\x1f--cfg\x1fgetrandom_backend = "custom"'}, []),
            ("target-env-response", {"CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS":
                                     APPROVED + " @" + str(files["custom"])},
             ["--target", "x86_64-unknown-linux-gnu"]),
            ("encoded-wins-conflict", {"RUSTFLAGS": APPROVED,
                                       "CARGO_ENCODED_RUSTFLAGS": APPROVED +
                                       '\x1f--cfg=getrandom_backend="custom"'}, []),
        ]
        for name, updates, extra in cases:
            rejected([*check, *extra], dict(base, **updates), "P2A-BUILD-003 " + name)
        # Recreate the original attack's build operation too, not only check.
        build = ["cargo", "build", *check[2:]]
        rejected(build, dict(base, RUSTFLAGS=APPROVED + " @" + str(files["custom"])),
                 "P2A-BUILD-003 original Astra CUSTOM cargo build")
        # Rustdoc operates on the approved already-built graph. Configuration
        # supplied flags are tested separately from environment-visible flags.
        doc = ["cargo", "test", *check[2:], "--doc"]
        doc_cases = [
            ("doc-hidden-custom", {"RUSTDOCFLAGS": APPROVED + " @" + str(files["custom"])}, []),
            ("doc-hidden-duplicate", {"RUSTDOCFLAGS":
                                     APPROVED + " @" + str(files["duplicate"])}, []),
            ("doc-hidden-alternate", {"RUSTDOCFLAGS":
                                     APPROVED + " @" + str(files["alternate"])}, []),
            ("doc-hidden-conflict", {"RUSTDOCFLAGS": APPROVED + " @" + str(files["conflict"])}, []),
            ("doc-nested", {"RUSTDOCFLAGS": APPROVED + " @" + str(files["nested"])}, []),
            ("doc-custom", {"RUSTDOCFLAGS": APPROVED + ' --cfg=getrandom_backend="custom"'}, []),
            ("doc-missing", {"RUSTDOCFLAGS": ""}, []),
            ("doc-encoded-response", {"CARGO_ENCODED_RUSTDOCFLAGS":
                                      APPROVED + "\x1f@" + str(files["custom"])}, []),
            ("doc-cli-response", {}, config_flags("build.rustdocflags",
                                                  ["@" + str(files["custom"])])),
            ("doc-cli-duplicate-response", {}, config_flags("build.rustdocflags",
                                                            ["@" + str(files["duplicate"])])),
            ("doc-cli-conflict", {}, config_flags("build.rustdocflags", [CUSTOM])),
            ("doc-conflicting-sources", {"CARGO_ENCODED_RUSTDOCFLAGS": APPROVED,
                                         "RUSTDOCFLAGS": '--cfg=getrandom_backend="custom"'}, []),
            ("doc-cli-launcher-override", {}, ["--config", 'build.rustdoc="rustdoc"']),
        ]
        for name, updates, extra in doc_cases:
            rejected([*doc, *extra], dict(env, **updates), "P2A-BUILD-003 " + name)
        print(f"P2A-BUILD-003 PASS: {len(cases) + 1 + len(doc_cases)} actual Cargo "
              "rejections; disposable response files removed", flush=True)


def main():
    env = os.environ.copy()
    env.setdefault("CARGO_HOME", str(ROOT / ".deps/cargo-home"))
    controls(env)


if __name__ == "__main__":
    main()
