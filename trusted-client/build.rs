//! Offline, native Linux build of the approved source; never probe system liboqs.

use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

const COMMIT: &str = "5a1a854b0dc9f2141bdc771c555ee60c37950183";

fn rng_flags_valid(flags: &str) -> bool {
    let args: Vec<_> = flags.split('\u{1f}').collect();
    let mut count = 0;
    for (i, arg) in args.iter().enumerate() {
        // rustc expands @files after Cargo's literal flag handling. Never read
        // them here: even harmless/duplicate/nested response files are forbidden.
        if arg.starts_with('@') {
            return false;
        }
        if arg.contains("getrandom_backend") {
            let split_form =
                *arg == "getrandom_backend=\"linux_getrandom\"" && i > 0 && args[i - 1] == "--cfg";
            let joined_form = *arg == "--cfg=getrandom_backend=\"linux_getrandom\"";
            if !split_form && !joined_form {
                return false;
            }
            count += 1;
        }
    }
    count == 1
}

fn run(command: &mut Command, log: &mut File) -> Result<String, Box<dyn Error>> {
    writeln!(log, "$ {command:?}")?;
    let output = command.output()?;
    log.write_all(&output.stdout)?;
    log.write_all(&output.stderr)?;
    writeln!(log, "exit={}\n", output.status)?;
    if !output.status.success() {
        return Err(format!("command failed: {command:?}; see build-evidence.txt").into());
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_owned())
}

fn verify_source(source: &Path, log: &mut File) -> Result<(), Box<dyn Error>> {
    let git = |args: &[&str]| {
        let mut command = Command::new("git");
        command.arg("-C").arg(source).args(args);
        command
    };
    if run(&mut git(&["rev-parse", "HEAD"]), log)? != COMMIT
        || run(&mut git(&["rev-parse", "refs/tags/0.16.0^{commit}"]), log)? != COMMIT
    {
        return Err("liboqs must be tag 0.16.0 at the approved exact commit".into());
    }
    // Also reject ignored source-side artifacts: a clean tracked tree alone is
    // insufficient if CMake could consume an untracked/ignored source file.
    let status = run(
        &mut git(&[
            "status",
            "--porcelain",
            "--untracked-files=all",
            "--ignored=matching",
        ]),
        log,
    )?;
    if !status.is_empty() {
        return Err("liboqs source must be clean, with no untracked or ignored artifacts".into());
    }
    // Git's assume-unchanged/skip-worktree flags can conceal edits from status.
    let files = run(&mut git(&["ls-files", "-v"]), log)?;
    if files.lines().any(|line| {
        line.starts_with('S') || line.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
    }) {
        return Err("liboqs source index must not hide tracked files".into());
    }
    Ok(())
}

fn sha3_avx512_disabled(header: &str) -> bool {
    // The pinned CMake template emits exactly this undef marker when disabled.
    // Reject definitions (even 0: upstream uses #if defined), missing markers,
    // duplicates and unfamiliar forms rather than guess preprocessor semantics.
    let lines: Vec<_> = header
        .lines()
        .filter(|line| line.contains("OQS_USE_SHA3_AVX512VL"))
        .map(str::trim)
        .collect();
    lines == ["/* #undef OQS_USE_SHA3_AVX512VL */"]
}

fn verify_configuration(header: &Path, log: &mut File) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(header)?;
    if !sha3_avx512_disabled(&contents) {
        return Err("generated liboqs configuration must disable OQS_USE_SHA3_AVX512VL".into());
    }
    writeln!(
        log,
        "Verified {}: /* #undef OQS_USE_SHA3_AVX512VL */",
        header.display()
    )?;
    Ok(())
}

fn build() -> Result<(), Box<dyn Error>> {
    // Complete opt-in backend set from pinned getrandom 0.4.3 backends.rs.
    println!(
        "cargo:rustc-check-cfg=cfg(getrandom_backend, values(\"custom\", \"efi_rng\", \"rdrand\", \"rndr\", \"linux_getrandom\", \"linux_raw\", \"windows_legacy\", \"unsupported\", \"extern_impl\"))"
    );
    println!("cargo:rerun-if-env-changed=CARGO_ENCODED_RUSTFLAGS");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_GETRANDOM_BACKEND");
    if !rng_flags_valid(&std::env::var("CARGO_ENCODED_RUSTFLAGS")?)
        || std::env::var("CARGO_CFG_GETRANDOM_BACKEND")?.as_str() != "linux_getrandom"
    {
        return Err(
            "Phase 2A requires Cargo-wide getrandom_backend=\"linux_getrandom\" exactly once"
                .into(),
        );
    }
    println!("cargo:rerun-if-env-changed=CADP_LIBOQS_SOURCE");
    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    // Per-crate `cargo rustc -- ...` flags are invisible here. Require the
    // actual-argv wrapper even when Cargo configuration/environment overrides it.
    let rustc_guard = manifest.join("../scripts/guard_rustc.py").canonicalize()?;
    println!("cargo:rerun-if-env-changed=RUSTC_WRAPPER");
    println!("cargo:rerun-if-changed={}", rustc_guard.display());
    if PathBuf::from(std::env::var_os("RUSTC_WRAPPER").ok_or("missing guarded RUSTC_WRAPPER")?)
        .canonicalize()
        .map_err(|_| "Phase 2A requires the guarded rustc wrapper")?
        != rustc_guard
    {
        return Err("Phase 2A requires the guarded rustc wrapper".into());
    }
    // Cargo does not expose configuration-supplied effective rustdocflags to a
    // build script. Require the argument-checking launcher for that separate path.
    let rustdoc_guard = manifest
        .join("../scripts/guard_rustdoc.py")
        .canonicalize()?;
    println!("cargo:rerun-if-env-changed=RUSTDOC");
    println!("cargo:rerun-if-changed={}", rustdoc_guard.display());
    if PathBuf::from(std::env::var_os("RUSTDOC").ok_or("missing guarded RUSTDOC")?)
        .canonicalize()
        .map_err(|_| "Phase 2A requires the guarded rustdoc launcher")?
        != rustdoc_guard
    {
        return Err("Phase 2A requires the guarded rustdoc launcher".into());
    }
    // Environment flag sources may coexist. Validate each provided source too;
    // the encoded Rust flags above remain authoritative for effective compilation.
    for name in ["RUSTFLAGS", "RUSTDOCFLAGS", "CARGO_ENCODED_RUSTDOCFLAGS"] {
        println!("cargo:rerun-if-env-changed={name}");
        if let Some(value) = std::env::var_os(name) {
            let value = value.to_str().ok_or("non-UTF-8 compiler flags rejected")?;
            let encoded = if name.starts_with("CARGO_ENCODED_") {
                value.to_owned()
            } else {
                value.split_whitespace().collect::<Vec<_>>().join("\u{1f}")
            };
            if !rng_flags_valid(&encoded) {
                return Err("Phase 2A rejects conflicting flag sources or response files".into());
            }
        }
    }
    let source = match std::env::var_os("CADP_LIBOQS_SOURCE") {
        Some(path) => PathBuf::from(path),
        None => manifest.join("../.deps/liboqs-0.16.0"),
    }
    .canonicalize()
    .map_err(|_| "approved local liboqs source missing; Cargo never downloads it")?;
    let target = std::env::var("TARGET")?;
    if target != std::env::var("HOST")? || !target.contains("linux") {
        return Err("Phase 1 supports native Linux builds only; cross-compilation untested".into());
    }
    println!("cargo:rerun-if-changed={}", source.display());
    println!("cargo:rerun-if-changed=native/oqs_shim.c");
    println!("cargo:rerun-if-changed=native/oqs_shim.h");
    let out = PathBuf::from(std::env::var("OUT_DIR")?);
    let mut log = File::create(out.join("build-evidence.txt"))?;
    verify_source(&source, &mut log)?;
    let prefix = out.join("liboqs-install");
    let build_dir = out.join("liboqs-build");
    run(
        Command::new("cmake")
            .arg("-S")
            .arg(&source)
            .arg("-B")
            .arg(&build_dir)
            .arg("-DCMAKE_BUILD_TYPE=Release")
            .arg("-DBUILD_SHARED_LIBS=OFF")
            .arg("-DOQS_BUILD_ONLY_LIB=ON")
            .arg("-DOQS_DIST_BUILD=ON")
            .arg("-DOQS_USE_OPENSSL=OFF")
            .arg("-DOQS_USE_SHA3_AVX512VL=OFF")
            .arg("-DOQS_MINIMAL_BUILD=KEM_ml_kem_768;SIG_ml_dsa_65")
            .arg("-DCMAKE_INSTALL_LIBDIR=lib")
            .arg("-DCMAKE_C_COMPILER=cc")
            .arg(format!("-DCMAKE_INSTALL_PREFIX={}", prefix.display())),
        &mut log,
    )?;
    // Check CMake's actual generated header before any backend compilation.
    // Explicit -D above also replaces an ON value in an existing CMake cache.
    verify_configuration(&build_dir.join("include/oqs/oqsconfig.h"), &mut log)?;
    run(
        Command::new("cmake")
            .arg("--build")
            .arg(&build_dir)
            .args(["--parallel", "2"]),
        &mut log,
    )?;
    run(
        Command::new("cmake").arg("--install").arg(&build_dir),
        &mut log,
    )?;
    verify_configuration(&prefix.join("include/oqs/oqsconfig.h"), &mut log)?;
    run(
        Command::new("cc")
            .args([
                "-std=c11",
                "-O2",
                "-fPIC",
                "-Wall",
                "-Wextra",
                "-Wpedantic",
                "-Werror",
            ])
            .arg("-I")
            .arg(prefix.join("include"))
            .arg("-c")
            .arg(manifest.join("native/oqs_shim.c"))
            .arg("-o")
            .arg(out.join("oqs_shim.o")),
        &mut log,
    )?;
    // Recreate the archive rather than leave stale members after a source change.
    let archive = out.join("libcadp_oqs_shim.a");
    if archive.exists() {
        fs::remove_file(&archive)?;
    }
    run(
        Command::new("ar")
            .arg("rcs")
            .arg(&archive)
            .arg(out.join("oqs_shim.o")),
        &mut log,
    )?;
    verify_source(&source, &mut log)?;
    run(
        Command::new("sha256sum").arg(prefix.join("lib/liboqs.a")),
        &mut log,
    )?;
    println!("cargo:rustc-link-search=native={}", out.display());
    println!(
        "cargo:rustc-link-search=native={}",
        prefix.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=cadp_oqs_shim");
    println!("cargo:rustc-link-lib=static=oqs");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-env=CADP_LIBOQS_COMMIT={COMMIT}");
    Ok(())
}

fn main() {
    if let Err(error) = build() {
        eprintln!("Phase 1 pinned liboqs build rejected: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::{rng_flags_valid, sha3_avx512_disabled};

    #[test]
    fn p2a_build_001_rng_configuration_fails_closed() {
        assert!(rng_flags_valid(
            "--cfg\u{1f}getrandom_backend=\"linux_getrandom\""
        ));
        assert!(rng_flags_valid(
            "--cfg=getrandom_backend=\"linux_getrandom\""
        ));
        for flags in [
            "",
            "getrandom_backend=\"linux_getrandom\"",
            "--cfg\u{1f}getrandom_backend=\"linux_raw\"",
            "--cfg\u{1f}getrandom_backend=\"linux_getrandom\"\u{1f}--cfg\u{1f}getrandom_backend=\"custom\"",
            "--cfg=getrandom_backend=\"linux_getrandom\"\u{1f}--cfg=getrandom_backend=\"linux_getrandom\"",
            "--cfg=getrandom_backend=\"linux_getrandom\"\u{1f}@anything",
            "@outer\u{1f}--cfg=getrandom_backend=\"linux_getrandom\"",
            "--cfg\u{1f}getrandom_backend = \"linux_getrandom\"",
        ] {
            assert!(!rng_flags_valid(flags));
        }
    }

    #[test]
    fn pqc_build_002_generated_configuration_is_fail_closed() {
        let disabled = "/* #undef OQS_USE_SHA3_AVX512VL */\n";
        assert!(sha3_avx512_disabled(disabled));
        for header in [
            "#define OQS_USE_SHA3_AVX512VL 1\n",
            "#define OQS_USE_SHA3_AVX512VL 0\n",
            "# define OQS_USE_SHA3_AVX512VL 1\n",
            "",
            "#undef OQS_USE_SHA3_AVX512VL\n",
            &format!("{disabled}#define OQS_USE_SHA3_AVX512VL 1\n"),
            &format!("{disabled}{disabled}"),
        ] {
            assert!(!sha3_avx512_disabled(header));
        }
    }
}
