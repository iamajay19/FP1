"""Q-01 metadata/linkage probe only: no keys, cryptographic operations or wrappers.

Retained to reproduce readiness offline. All compiled artifacts are temporary.
"""

import hashlib
import os
import shlex
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / ".deps/liboqs-0.16.0"
COMMIT = "5a1a854b0dc9f2141bdc771c555ee60c37950183"

# Use installed C headers for struct access instead of duplicating their Rust layout.
C_SOURCE = r"""
#include <oqs/oqs.h>
int q01_metadata(const OQS_KEM *k, const OQS_SIG *s, size_t out[8]) {
    if (!k || !s || !out || !s->sign || !s->verify ||
        !s->sign_with_ctx_str || !s->verify_with_ctx_str) return 0;
    out[0] = k->length_public_key;
    out[1] = k->length_secret_key;
    out[2] = k->length_ciphertext;
    out[3] = k->length_shared_secret;
    out[4] = s->length_public_key;
    out[5] = s->length_secret_key;
    out[6] = s->length_signature;
    out[7] = s->sig_with_ctx_support;
    return 1;
}
"""

RUST_SOURCE = r"""
use std::ffi::{c_char, c_int, c_void, CStr};

unsafe extern "C" {
    fn OQS_init();
    fn OQS_destroy();
    fn OQS_version() -> *const c_char;
    fn OQS_KEM_alg_is_enabled(name: *const c_char) -> c_int;
    fn OQS_SIG_alg_is_enabled(name: *const c_char) -> c_int;
    fn OQS_KEM_new(name: *const c_char) -> *mut c_void;
    fn OQS_SIG_new(name: *const c_char) -> *mut c_void;
    fn OQS_KEM_free(kem: *mut c_void);
    fn OQS_SIG_free(sig: *mut c_void);
    fn q01_metadata(kem: *const c_void, sig: *const c_void, out: *mut usize) -> c_int;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Safety: fixed NUL-terminated identifiers; opaque handles only go back to
    // their owning library. C reads structs using matching installed headers.
    // No keys or secret buffers are created. Cleanup runs before any error return.
    let result = unsafe {
        OQS_init();
        let kem = OQS_KEM_new(c"ML-KEM-768".as_ptr());
        let sig = OQS_SIG_new(c"ML-DSA-65".as_ptr());
        let check = || -> Result<(), Box<dyn std::error::Error>> {
            let version_ptr = OQS_version();
            if version_ptr.is_null() || kem.is_null() || sig.is_null() {
                return Err("null library version or algorithm handle".into());
            }
            let version = CStr::from_ptr(version_ptr).to_str()?;
            if version != "0.16.0"
                || OQS_KEM_alg_is_enabled(c"ML-KEM-768".as_ptr()) != 1
                || OQS_SIG_alg_is_enabled(c"ML-DSA-65".as_ptr()) != 1
            {
                return Err("wrong version or missing required algorithm".into());
            }
            let mut sizes = [0usize; 8];
            if q01_metadata(kem, sig, sizes.as_mut_ptr()) != 1
                || sizes != [1184, 2400, 1088, 32, 1952, 4032, 3309, 1]
            {
                return Err("unexpected sizes or context API support".into());
            }
            println!("liboqs={version}");
            println!("ML-KEM-768 enabled=1 pk={} sk={} ct={} ss={}",
                sizes[0], sizes[1], sizes[2], sizes[3]);
            println!("ML-DSA-65 enabled=1 pk={} sk={} sig={} context_support={}",
                sizes[4], sizes[5], sizes[6], sizes[7]);
            println!("METADATA/LINKAGE PASS; no cryptographic operations executed");
            Ok(())
        };
        let result = check();
        OQS_KEM_free(kem);
        OQS_SIG_free(sig);
        OQS_destroy();
        result
    };
    result
}
"""


def run(args, *, env=None, expected=0):
    """Print exact commands and actual output; fail on unexpected exit status."""
    print("$ " + shlex.join(map(str, args)), flush=True)
    result = subprocess.run(args, env=env, text=True, capture_output=True, check=False)
    print(result.stdout, end="", flush=True)
    print(result.stderr, end="", flush=True)
    print(f"exit={result.returncode}", flush=True)
    if result.returncode != expected:
        raise RuntimeError(f"unexpected command exit: {result.returncode}")
    return result.stdout.strip()


def main():
    if run(["git", "-C", SOURCE, "rev-parse", "HEAD"]) != COMMIT:
        raise RuntimeError("source is not the pinned liboqs release commit")
    if run(["git", "-C", SOURCE, "status", "--porcelain", "--untracked-files=all"]):
        raise RuntimeError("source tree is dirty")
    if run(["git", "-C", SOURCE, "describe", "--tags", "--exact-match"]) != "0.16.0":
        raise RuntimeError("source tag is not 0.16.0")
    for command in (["rustc", "-Vv"], ["cargo", "-Vv"], ["gcc", "--version"],
                    ["g++", "--version"], ["cmake", "--version"],
                    ["pkg-config", "--version"], ["git", "--version"]):
        run(command)
    with tempfile.TemporaryDirectory(prefix="fp1-q01-") as temporary:
        work = Path(temporary)
        prefix = work / "prefix"
        run(["cmake", "-S", SOURCE, "-B", work / "build",
             "-DCMAKE_BUILD_TYPE=Release", "-DBUILD_SHARED_LIBS=OFF",
             "-DOQS_BUILD_ONLY_LIB=ON", "-DOQS_DIST_BUILD=ON",
             "-DOQS_USE_OPENSSL=OFF", "-DOQS_MINIMAL_BUILD=KEM_ml_kem_768;SIG_ml_dsa_65",
             f"-DCMAKE_INSTALL_PREFIX={prefix}"])
        run(["cmake", "--build", work / "build", "--parallel", "2"])
        run(["cmake", "--install", work / "build"])
        libdir = prefix / "lib64"
        if not libdir.exists():
            libdir = prefix / "lib"
        env = os.environ.copy()
        env["PKG_CONFIG_LIBDIR"] = str(libdir / "pkgconfig")
        env["PKG_CONFIG_PATH"] = str(libdir / "pkgconfig")
        env["CARGO_HOME"] = str(work / "cargo-home")
        env["CARGO_TARGET_DIR"] = str(work / "target")
        print(f"environment overrides: {env['PKG_CONFIG_LIBDIR']=}, "
              f"{env['CARGO_HOME']=}, {env['CARGO_TARGET_DIR']=}", flush=True)
        run(["pkg-config", "--modversion", "liboqs"], env=env)
        # This is the exact external-library range used by the inspected oqs-sys.
        run(["pkg-config", "--exists", "liboqs >= 0.16.0-rc1", "liboqs < 0.17.0"],
            env=env, expected=1)
        (work / "metadata.c").write_text(C_SOURCE)
        run(["gcc", "-std=c11", "-Wall", "-Wextra", "-Werror", "-I",
             prefix / "include", "-c", work / "metadata.c", "-o", work / "metadata.o"])
        run(["ar", "rcs", work / "libq01metadata.a", work / "metadata.o"])
        project = work / "probe"
        (project / "src").mkdir(parents=True)
        (project / "Cargo.toml").write_text(
            '[package]\nname="q01-readiness"\nversion="0.0.0"\nedition="2024"\n'
        )
        (project / "src/main.rs").write_text(RUST_SOURCE)
        run(["cargo", "fmt", "--manifest-path", project / "Cargo.toml"], env=env)
        run(["cargo", "fmt", "--check", "--manifest-path", project / "Cargo.toml"], env=env)
        run(["cargo", "rustc", "--offline", "--manifest-path", project / "Cargo.toml",
             "--", "-L", f"native={work}", "-L", f"native={libdir}",
             "-l", "static=q01metadata", "-l", "static=oqs"], env=env)
        binary = work / "target/debug/q01-readiness"
        run([binary])
        dynamic = run(["readelf", "-d", binary])
        if "liboqs.so" in dynamic:
            raise RuntimeError("unexpected dynamic liboqs linkage")
        for artifact in (libdir / "liboqs.a", binary):
            digest = hashlib.sha256(artifact.read_bytes()).hexdigest()
            print(f"SHA256 {artifact}: {digest}", flush=True)
    print("Temporary build/install/probe artifacts removed; Q-01 probe PASS", flush=True)


if __name__ == "__main__":
    main()
