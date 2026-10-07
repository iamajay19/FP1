"""Native guards, generated config and Rust partial-output cleanup; isolated linkage."""

import json
import os
import shlex
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CLIENT = ROOT / "trusted-client"


def run(args, env):
    print("$ " + shlex.join(map(str, args)), flush=True)
    subprocess.run(args, env=env, check=True)


def build_artifacts(env):
    command = ["cargo", "build", "--offline", "--locked", "--message-format=json",
               "--manifest-path", CLIENT / "Cargo.toml"]
    print("$ " + shlex.join(map(str, command)), flush=True)
    result = subprocess.run(command, env=env, capture_output=True, text=True, check=True)
    print(result.stderr, end="", flush=True)
    messages = [json.loads(line) for line in result.stdout.splitlines()]
    # Select artifacts from this invocation, including Cargo's fresh results;
    # never choose an unrelated old native build by directory modification time.
    package_ids = {m["package_id"] for m in messages if m["reason"] == "compiler-artifact"
                   and m["target"]["name"] == "cadp_pqc"}
    outputs = [Path(m["out_dir"]) for m in messages if m["reason"] == "build-script-executed"
               and m["package_id"] in package_ids]
    libraries = [Path(f) for m in messages if m["reason"] == "compiler-artifact"
                 and m["target"]["name"] == "cadp_pqc" for f in m["filenames"]
                 if f.endswith(".rlib")]
    if len(outputs) != 1 or len(libraries) != 1:
        raise RuntimeError("expected this invocation's cadp-pqc native build and rlib")
    return outputs[0], libraries[0]


def configuration_regression(work, prefix, env):
    header = prefix / "include/oqs/oqsconfig.h"
    disabled = "/* #undef OQS_USE_SHA3_AVX512VL */"
    lines = [line.strip() for line in header.read_text().splitlines()
             if "OQS_USE_SHA3_AVX512VL" in line]
    if lines != [disabled]:
        raise RuntimeError("PQC-BUILD-002: installed SHA3 AVX-512 configuration is forbidden")
    print(f"PQC-BUILD-002 generated configuration: {disabled}", flush=True)
    binary = work / "configuration-tests"
    run(["rustc", "--edition=2024", "--test", "-D", "warnings", CLIENT / "build.rs",
         "-o", binary], env)
    run([binary, "--nocapture"], env)
    # Change only a disposable installed-header copy. The real source, cache and
    # installed headers remain untouched. Any macro definition is forbidden,
    # including a value of zero, because upstream uses #if defined(...).
    fake = work / "forbidden/oqs"
    fake.mkdir(parents=True)
    for value in ("1", "0"):
        (fake / "oqsconfig.h").write_text(
            header.read_text() + f"\n#define OQS_USE_SHA3_AVX512VL {value}\n")
        command = ["cc", "-std=c11", "-fsyntax-only", "-I", fake.parent,
                   "-I", prefix / "include", CLIENT / "native/oqs_shim.c"]
        print("$ " + shlex.join(map(str, command)), flush=True)
        result = subprocess.run(command, env=env, capture_output=True, text=True, check=False)
        print(result.stderr, end="", flush=True)
        if result.returncode == 0 or "Phase 1 forbids OQS_USE_SHA3_AVX512VL" not in result.stderr:
            raise RuntimeError("PQC-BUILD-002: shim did not reject forbidden configuration")
        print(f"Expected compile rejection for macro={value}: exit={result.returncode}", flush=True)
    print("PQC-BUILD-002 generated-header and forbidden-configuration regression PASS", flush=True)


def partial_output_regression(work, library, prefix, env):
    source = CLIENT / "tests/support"
    obj = work / "partial_output.o"
    binary = work / "partial-output"
    run(["rustfmt", "--edition", "2024", "--check", source / "partial_output.rs"], env)
    run(["cc", "-std=c11", "-O2", "-fPIC", "-Wall", "-Wextra", "-Wpedantic", "-Werror",
         "-I", prefix / "include", "-c", source / "partial_output.c", "-o", obj], env)
    command = ["rustc", "--edition=2024", "-D", "warnings", "-C", "opt-level=2",
               source / "partial_output.rs", "--extern", f"cadp_pqc={library}",
               "-L", f"dependency={library.parent / 'deps'}",
               "-C", f"link-arg={obj}"]
    for symbol in ("OQS_KEM_keypair", "OQS_SIG_keypair", "OQS_KEM_encaps",
                   "OQS_KEM_decaps", "OQS_MEM_cleanse", "free"):
        command += ["-C", f"link-arg=-Wl,--wrap={symbol}"]
    command += ["-o", binary]
    run(command, env)
    for mode in ("kem-keygen", "sig-keygen", "encaps", "decaps"):
        print(f"PQC-SECRET-003 mode={mode}; synthetic output only", flush=True)
        run([binary], dict(env, CADP_TEST_PARTIAL_OUTPUT=mode))
    print("PQC-SECRET-003 PASS: 4 Rust error/Drop/cleanse/deallocation paths", flush=True)


def main():
    env = os.environ.copy()
    env["CARGO_HOME"] = str(ROOT / ".deps/cargo-home")
    env.setdefault("CARGO_TARGET_DIR", str(CLIENT / "target"))
    out, library = build_artifacts(env)
    prefix = out / "liboqs-install"
    wrapped = (
        "OQS_KEM_new OQS_SIG_new OQS_KEM_free OQS_SIG_free OQS_version "
        "OQS_KEM_keypair OQS_KEM_encaps OQS_KEM_decaps "
        "OQS_SIG_keypair OQS_SIG_sign OQS_SIG_verify"
    ).split()
    with tempfile.TemporaryDirectory(prefix="fp1-native-test-") as temporary:
        work = Path(temporary)
        configuration_regression(work, prefix, env)
        binary = work / "native-contract"
        # Fedora's visible GCC sanitizer runtimes are missing; Clang's bundled
        # compiler-rt is available. This affects tests only, not the pinned build.
        command = ["clang", "-std=c11", "-O1", "-g", "-fPIC", "-Wall", "-Wextra",
                   "-Wpedantic", "-Werror", "-fsanitize=address,undefined",
                   "-fno-omit-frame-pointer", "-I", CLIENT / "native", "-I",
                   prefix / "include", CLIENT / "native/oqs_shim.c",
                   CLIENT / "tests/native_contract.c", prefix / "lib/liboqs.a", "-lpthread"]
        command += [f"-Wl,--wrap={symbol}" for symbol in wrapped]
        command += ["-o", binary]
        run(command, env)
        leaks = env.get("CADP_TEST_LEAKS", "0")
        if leaks not in ("0", "1"):
            raise RuntimeError("CADP_TEST_LEAKS must be 0 or 1")
        print(f"LeakSanitizer detect_leaks={leaks}; set CADP_TEST_LEAKS=1 on an "
              "unrestricted host (this execution surface rejects its ptrace use)", flush=True)
        env["ASAN_OPTIONS"] = f"detect_leaks={leaks}:halt_on_error=1"
        env["UBSAN_OPTIONS"] = "halt_on_error=1"
        run([binary], env)
        partial_output_regression(work, library, prefix, env)
    print("4 native contracts, config regression and 4 Rust cleanup cases passed; "
          "temporary fixtures removed", flush=True)


if __name__ == "__main__":
    main()
