"""P2A-DEPS-001/P2A-RNG-003/P2A-BUILD-002: vendor, features, failure controls.

No network. Fixtures are public, disposable and never linked into production.
This is consistency verification, not upstream authenticity or an advisory audit.
"""
import hashlib
import json
import os
import subprocess
import shutil
import tempfile
import tomllib
from pathlib import Path

from check_pqc_native import build_artifacts, run
from check_rng_policy import controls as rng_policy_controls

ROOT = Path(__file__).resolve().parents[1]
PINS = {
    "aes-gcm": ("0.11.1", ["aes", "zeroize"]), "argon2": ("0.6.0", ["zeroize"]),
    "sha2": ("0.11.0", ["zeroize"]), "blake2": ("0.11.0", ["zeroize"]),
    "getrandom": ("0.4.3", []), "rpassword": ("7.5.4", []),
    "zeroize": ("1.9.1", ["alloc"]), "minicbor": ("2.3.0", []),
    "hmac": ("0.13.0", ["zeroize"]), "hkdf": ("0.13.0", []),
}


def integrity():
    vendor = ROOT / ".deps/phase2-vendor"
    assert not vendor.is_symlink()
    lock = tomllib.loads((ROOT / ".deps/phase2-bootstrap-Cargo.lock").read_text())
    packages = {(v["name"], v["version"]): v for v in lock["package"] if "source" in v}
    owner_meta = json.loads((ROOT / ".deps/phase2-bootstrap-metadata.json").read_text())
    owner_set = {(v["name"], v["version"]) for v in owner_meta["packages"] if v["source"]}
    assert owner_set == set(packages)
    project_lock = tomllib.loads((ROOT / "trusted-client/Cargo.lock").read_text())
    locked = {(v["name"], v["version"]): v for v in project_lock["package"] if "source" in v}
    assert locked == packages
    actual, seen, count = set(), set(), 0
    for directory in vendor.iterdir():
        assert directory.is_dir() and not directory.is_symlink()
        for path in directory.rglob("*"):
            assert not path.is_symlink() and (path.is_dir() or path.is_file())
        manifest = tomllib.loads((directory / "Cargo.toml").read_text())["package"]
        key = (manifest["name"], manifest["version"])
        assert key in packages and key not in seen
        seen.add(key)
        assert directory.name == f"{key[0]}-{key[1]}"
        checks = json.loads((directory / ".cargo-checksum.json").read_text())
        assert checks["package"] == packages[key]["checksum"]
        files = {str(p.relative_to(directory)) for p in directory.rglob("*") if p.is_file()}
        assert files == set(checks["files"]) | {".cargo-checksum.json"}
        for name, checksum in checks["files"].items():
            p = Path(name)
            assert not p.is_absolute() and ".." not in p.parts
            assert hashlib.sha256((directory / p).read_bytes()).hexdigest() == checksum
        count += len(checks["files"])
        actual |= {str((directory / f).relative_to(ROOT)) for f in files}
    assert seen == set(packages)
    inventory = {}
    for line in (ROOT / ".deps/phase2-vendor-SHA256SUMS.txt").read_text().splitlines():
        checksum, name = line.split("  ", 1)
        assert name not in inventory
        inventory[name] = checksum
    assert set(inventory) == actual
    for name, checksum in inventory.items():
        assert hashlib.sha256((ROOT / name).read_bytes()).hexdigest() == checksum
    print(f"P2A-DEPS-001 integrity PASS: {len(packages)} packages / {count} Cargo files / "
          f"{len(inventory)} owner hashes, exact sets, no symlinks", flush=True)


def features(env):
    manifest = tomllib.loads((ROOT / "trusted-client/Cargo.toml").read_text())
    assert set(manifest["dependencies"]) == set(PINS)
    for name, (version, feature_set) in PINS.items():
        direct = manifest["dependencies"][name]
        assert direct["version"] == "=" + version and direct["default-features"] is False
        assert sorted(direct.get("features", [])) == sorted(feature_set)
    data = json.loads(subprocess.check_output(
        ["cargo", "metadata", "--offline", "--locked", "--format-version", "1",
         "--filter-platform", "x86_64-unknown-linux-gnu", "--manifest-path",
         ROOT / "trusted-client/Cargo.toml"], env=env, cwd=ROOT))
    nodes = {n["id"]: n for n in data["resolve"]["nodes"]}
    registry = [p for p in data["packages"] if p["source"] and p["id"] in nodes]
    assert len(registry) == 31
    for p in registry:
        if p["name"] in PINS:
            version, feature_set = PINS[p["name"]]
            assert p["version"] == version
            assert sorted(nodes[p["id"]]["features"]) == sorted(feature_set)
        assert not any("proc-macro" in t["kind"] for t in p["targets"])
    print("P2A-DEPS-001 features PASS: exact ten pins / 31 Linux registry crates", flush=True)


def controls(env):
    _, library = build_artifacts(env)
    with tempfile.TemporaryDirectory(prefix="fp1-p2a-controls-") as name:
        work = Path(name)
        c = work / "deny.c"
        c.write_text('#include <errno.h>\n#include <sys/types.h>\n#include <stdlib.h>\n'
                     'ssize_t getrandom(void *p,size_t n,unsigned flags) {'
                     '(void)p;(void)n;(void)flags;'
                     'errno=getenv("P2A_TEST_EPERM")?EPERM:ENOSYS;return -1;}\n')
        native = work / "deny.so"
        run(["cc", "-shared", "-fPIC", "-Wall", "-Wextra", "-Werror", c, "-o", native], env)
        rust = work / "rng.rs"
        rust.write_text('#![deny(unsafe_code)]\nuse cadp_pqc::foundation::{RandomId,Error};\n'
                        'fn main(){assert_eq!(RandomId::generate(),'
                        'Err(Error::RandomnessUnavailable));}\n')
        binary = work / "rng"
        run(["rustc", "--edition=2024", "-D", "warnings", rust, "--extern",
             f"cadp_pqc={library}", "-L", f"dependency={library.parent / 'deps'}",
             "-o", binary], env)
        for error in ("ENOSYS", "EPERM"):
            fixture_env = dict(env, LD_PRELOAD=str(native))
            fixture_env.pop("P2A_TEST_EPERM", None)
            if error == "EPERM":
                fixture_env["P2A_TEST_EPERM"] = "1"
            run([binary], fixture_env)
            print(f"P2A-RNG-003 production syscall failure {error}: PASS", flush=True)
        for flags in ("", '--cfg=getrandom_backend="linux_raw"',
                      '--cfg=getrandom_backend="linux_getrandom" '
                      '--cfg=getrandom_backend="linux_raw"'):
            case_env = dict(env, RUSTFLAGS=flags, CARGO_TARGET_DIR=str(work / "target"))
            case_env.pop("CARGO_ENCODED_RUSTFLAGS", None)
            command = ["cargo", "check", "--offline", "--locked", "--manifest-path",
                       ROOT / "trusted-client/Cargo.toml"]
            result = subprocess.run(command, env=case_env, cwd=ROOT, capture_output=True, text=True)
            assert result.returncode == 101, result.stderr
            guard = "requires Cargo-wide getrandom_backend" in result.stderr
            # Conflicting cfgs can make getrandom reject its own dependency setup
            # before Cargo runs our guard. That is also fail-closed, not proof the
            # guard ran; its exact predicate is separately tested via build.rs.
            assert guard or "could not compile `getrandom`" in result.stderr, result.stderr
            reason = "project guard" if guard else "dependency configuration rejection"
            print(f"Rejection source: {reason}", flush=True)
            print(f"P2A-BUILD-002 override rejected: {flags!r}; exit=101", flush=True)
        # Meaningful sensitivity check: remove ONLY wiping in a disposable source
        # copy. The actual Drop observer must reject still-live nonzero memory.
        copy = work / "without-cleanse"
        shutil.copytree(ROOT / "trusted-client", copy,
                        ignore=shutil.ignore_patterns("target"))
        # The copied crate preserves the production relative launcher check.
        # Only this disposable fixture links back to the real checked-in script.
        (work / "scripts").symlink_to(ROOT / "scripts", target_is_directory=True)
        secret = copy / "src/foundation/secret.rs"
        contents = secret.read_text()
        marker = "        self.bytes.as_mut().zeroize();"
        assert contents.count(marker) == 1
        secret.write_text(contents.replace(marker, "        // Negative control: omit wipe."))
        control_env = dict(env, CADP_LIBOQS_SOURCE=str(ROOT / ".deps/liboqs-0.16.0"),
                           CARGO_TARGET_DIR=str(work / "negative-target"))
        result = subprocess.run(
            ["cargo", "test", "--offline", "--locked", "--manifest-path", copy / "Cargo.toml",
             "p2a_secret_001"], cwd=ROOT, env=control_env, capture_output=True, text=True)
        assert result.returncode == 101
        assert "p2a_secret_001_redaction_and_live_drop ... FAILED" in result.stdout
        assert "assertion `left == right` failed" in result.stdout
        print("P2A-SECRET-002 missing-Drop-wipe negative control rejected: exit=101", flush=True)
    print("Temporary fixtures removed; no production hook or alternate RNG", flush=True)


def main():
    integrity()
    env = os.environ.copy()
    env.setdefault("CARGO_HOME", str(ROOT / ".deps/cargo-home"))
    features(env)
    controls(env)
    rng_policy_controls(env)


if __name__ == "__main__":
    main()
