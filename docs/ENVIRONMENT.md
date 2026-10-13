# Development Environment Inspection

Current status: Phase 1 environment preparation **READY**, Q-01 **RESOLVED** by the
local-source build/linkage evaluation below. Phase 1 primitive implementation is now
authorized and its tests have executed; see the final implementation section.
Earlier inspections and readiness-only test statuses are historical.
The 2026-10-07 corrective section records the current enforced SHA3 configuration;
the original implementation/readiness build descriptions below remain historical.

Inspection date: 2026-10-06, from `/home/iamajay/Projects/FP1` in the agent execution
surface. Host inspection is read-only; the final evaluation also builds into temporary
directories. No system installation or host configuration changes were made.
Availability means visible to this session, not proof absent everywhere.

| Probe | Observed result |
|---|---|
| OS / CPU | Fedora Linux 44 KDE; Linux 7.2.8-200.fc44.x86_64; x86_64 |
| Rust / toolchain / Cargo | rustc, rustup, cargo not on PATH |
| Python | 3.14.7; meets specification 3.12+; future dependencies untested |
| Global pip | No pip executable/module |
| venv | Module available; actual temporary `python3 -m venv` creation succeeded and was removed |
| Go | Not on PATH |
| Docker | CLI 29.8.2, build 7fc2dff |
| Docker Compose | v5.6.0 |
| Docker daemon | `docker info` failed: permission denied on local socket; no server version verified |
| Git | 2.55.0; `git status --short` exit 128: not a git repository |
| C compiler | GCC 16.2.1 (20260819), invoked as cc |
| C++ compiler / CMake | c++ and cmake not on PATH |
| pkg-config | 2.5.1 |
| liboqs | pkg-config cannot find liboqs; no linker-cache oqs entry; no Python oqs module |
| Make | GNU Make 4.4.1 |
| Ruff / Markdown lint / Mermaid CLI | ruff, markdownlint, mmdc not on PATH |
| Memory lock / core limits | 8192 KiB / unlimited (soft shell limits) |
| Swap | /dev/zram0 enabled, about 8 GiB; no plaintext protection inferred |

The exposed `.git` directory is read-only and is not usable Git metadata in this
surface. Do not initialize over it or claim a Git diff/commit was inspected. Initial
files were AGENTS.md, the two authoritative specification/plan files, a 10-row test
matrix and 13 empty supporting documents. Phase 0 modifies only the requested tree.

## Reproduce

```sh
python3 scripts/inspect_environment.py
make environment
```

The probe exits normally after collecting statuses; individual FAILED/MISSING
results are readiness blockers, not hidden successes. Temporary venv creation is
isolated outside the repository and cleaned up. No images are pulled and no crypto
library is invoked. Environment results can change; use a fresh probe before Phase 1.

Phase 1 needs an approved working Rust toolchain, compatible maintained liboqs and
bindings, native build prerequisites and exact version records. Phase 6 additionally
needs Go and accessible local Docker/Fabric tooling. No installation was attempted
because it is outside the Phase 0 implementation scope and write permissions.

## Phase 1 environment preparation — 2026-10-06

Scope approved: prerequisite inspection/preparation only. Phase 1 implementation
remains unapproved. No dependencies were installed, system settings changed, keys
generated or confidential files processed. No production Rust module was created.
The existing environment inspector was rerun successfully; its individual failures
remain failures. Focused command output is preserved in
[PHASE_1_ENVIRONMENT_VALIDATION.txt](PHASE_1_ENVIRONMENT_VALIDATION.txt).

### Fresh local results

| Component | Exact observed version / state |
|---|---|
| Fedora / architecture | Fedora 44 KDE, x86_64; kernel 7.2.8-200.fc44.x86_64 |
| Rust / Cargo / rustup | MISSING on PATH and absent from queried RPM packages; no ~/.cargo or ~/.rustup directories |
| GCC | 16.2.1 20260819 (Red Hat 16.2.1-2); gcc-16.2.1-2.fc44.x86_64 |
| C++ | MISSING; gcc-c++ not installed |
| CMake | MISSING; cmake not installed |
| pkg-config | 2.5.1; pkgconf-pkg-config-2.5.1-1.fc44.x86_64 |
| Make | 4.4.1; make-4.4.1-12.fc44.x86_64 |
| Git | 2.55.0; git-2.55.0-1.fc44.x86_64; repository access fails |
| Clang / libclang | clang, clang-libs and clang-devel not installed; no libclang linker-cache match |
| OpenSSL development files | openssl-devel-3.5.9-1.fc44.x86_64 installed |
| liboqs | No installed liboqs/liboqs-devel RPM, pkg-config record or linker-cache match |
| oqs / oqs-sys | Not built or installed; no project Cargo manifest/lockfile |

No relevant Rust/liboqs/build-prerequisite archives were found in the inspected
readable /tmp, /opt and /var/cache paths. This is not a claim about every host path.
The /usr path is on a read-only mount in this session, and write permission is
limited to the project and /tmp. Escalation for system installation is unavailable.
`curl -I --max-time 15 https://crates.io` exits 6: could not resolve host.
Consequently neither Fedora-native installation nor a downloaded local toolchain
can be completed in this execution surface. No installation command was attempted.
Web research tools can read upstream documentation; that does not provide Cargo
with network access or make build artifacts available to the shell.

Go, Docker/Fabric, pip, GUI, PDF and ECC dependencies are **not Phase 1 blockers**.
The memory-hardening deployment gate remains Phase 5. rustup is optional if a
suitable Fedora Rust/Cargo toolchain is used; Rust/Cargo themselves are required.

### Binding compatibility research (not local execution)

Upstream's published release pair is `oqs 0.11.0` and
`oqs-sys 0.11.0+liboqs-0.13.0`, bundling liboqs 0.13.0. This is an exact
source-documented compatibility candidate, not an installed or approved project pin.
See the [binding release](https://github.com/open-quantum-safe/liboqs-rust/releases/tag/v0.11.0)
and [published manifest](https://docs.rs/crate/oqs/0.11.0/source/Cargo.toml).

The [published build script](https://docs.rs/crate/oqs-sys/0.11.0%2Bliboqs-0.13.0/source/build.rs)
accepts system liboqs >=0.13.0 and <0.14.0, otherwise falls back to its vendored
source unless `LIBOQS_NO_VENDOR=1` forbids fallback. `vendored` explicitly forces
the bundled build. It invokes bindgen, requiring libclang, and CMake for vendoring.
Upstream liboqs now lists [0.16.0](https://github.com/open-quantum-safe/liboqs/releases/tag/0.16.0).
Do not silently pair that newer library with the published 0.11.0 bindings or
mistake the bundled 0.13.0 for the latest liboqs. Current maintenance/security
suitability of the older pair, or a reviewed newer binding commit, remains part of
Q-01; no verified maintained combination has been established on this machine.

Candidate build configuration, **not applied**: exact oqs/oqs-sys 0.11.0 dependency
requirements, default features disabled, `std`, `ml_kem`, `ml_dsa`, `vendored` on
oqs. The published build script then selects Release, OQS_BUILD_ONLY_LIB=Yes,
OQS_DIST_BUILD=Yes, OQS_USE_OPENSSL=No and static liboqs linkage on Linux.
ML-KEM/ML-DSA families are enabled; legacy Kyber/Dilithium features are not selected.
These family flags include other parameter sets; only the two required parameter
sets would be instantiated by the readiness probe. No non_portable feature.
Record effective build logs and Cargo.lock checksums before claiming a pin;
Cargo version requirements alone do not lock transitive dependencies. No Rust,
Cargo or CMake version is invented while those executables are absent.

### Algorithm availability and linking status

Source/API evidence identifies
[`oqs::kem::Algorithm::MlKem768`](https://docs.rs/oqs/0.11.0/oqs/kem/enum.Algorithm.html)
and [`oqs::sig::Algorithm::MlDsa65`](https://docs.rs/oqs/0.11.0/oqs/sig/enum.Algorithm.html).
Required runtime names are exactly `ML-KEM-768` and `ML-DSA-65`. The latter is also
explicit in the [liboqs 0.13.0 header](https://raw.githubusercontent.com/open-quantum-safe/liboqs/0.13.0/src/sig/sig.h).
Enumeration in documentation is not proof of an enabled algorithm in a local build.

Rust compilation, oqs-sys build, static linkage, and both runtime algorithm checks
are **BLOCKED / NOT EXECUTED**. No temporary Rust project was created because Cargo
cannot start. There is no resulting binary or algorithm-support output to report.
All PQC-KEM-001..004 and PQC-SIG-001..005 remain PLANNED in the unchanged test matrix.

When prerequisites and a maintained version pair are available, create a disposable
Cargo binary outside production component directories. Its entire operation must
be oqs::init(), checking each required enum's name() and is_enabled(), and constructing
Kem::new(MlKem768) and Sig::new(MlDsa65), with controlled nonzero failure. Print
the actual names and resolved versions. Do not call keypair, encapsulation,
decapsulation, signing or verification. Retain the lockfile/build log as evidence,
then remove the disposable source/build directory. Use cargo build --locked -vv
and cargo run --locked after generating and inspecting that isolated lockfile.
A successful static build proves linkage without requiring a shared liboqs loader.

### Git investigation

`findmnt -T .git` identifies an empty **read-only tmpfs mount**, not normal Git
metadata. `ls -la .git` shows only dot entries; git status exits 128. The execution
surface may hide underlying host metadata, so this does not prove the host project
lacks history. No git init, metadata repair, unmount, deletion or chmod was attempted.
The owner should inspect the project in a normal host terminal first. If an existing
repository is found, expose/use that checkout. Only if host-side inspection confirms
there is no repository or history to preserve should the owner choose git init.
Q-09 remains open; Git is not a compiler dependency but version control is not ready.

### Exact manual checks and recovery

Read-only checks (expected missing-tool failures in this session):

```sh
cd /home/iamajay/Projects/FP1
python3 scripts/inspect_environment.py
rustc --version --verbose
cargo --version --verbose
rustup show
gcc --version
c++ --version
cmake --version
pkg-config --version
pkg-config --modversion liboqs
rpm -q rust cargo gcc gcc-c++ cmake clang clang-libs clang-devel
git --version
findmnt -T .git
ls -la .git
git status --short
curl -I --max-time 15 https://crates.io
```

Suggested host-side prerequisite installation, **NOT EXECUTED here**. Run only in
a normal writable Fedora host environment with package access; review DNF's proposed
transaction. This supplies ordinary build tools, not a verified Rust/liboqs pair:

```sh
sudo dnf install rust cargo rustfmt clippy gcc gcc-c++ cmake make pkgconf-pkg-config clang clang-devel
```

Then record exact installed versions, make them visible to the execution surface,
resolve the binding/liboqs maintenance gate, supply registry access or verified
offline sources, and execute the isolated no-key linkage probe. Do not install Go,
Fabric, GUI or watermark dependencies as part of this preparation.

Q-01 remains **BLOCKED**. Environment preparation approval does not approve Phase 1
implementation. Phase 0 evidence remains historical and unchanged.

## Post-installation reinspection — 2026-10-06

This section supersedes earlier readiness conclusions. The user installed host
prerequisites; this session re-ran `python3 scripts/inspect_environment.py` before
further preparation. Phase 1 implementation is still unapproved. No keys generated,
no production wrapper written, no old liboqs installed. Recorded results are in
[PHASE_1_ENVIRONMENT_RECHECK.txt](PHASE_1_ENVIRONMENT_RECHECK.txt).

### Current exact versions

| Component | Observed result |
|---|---|
| Fedora / architecture / kernel | 44 KDE / x86_64 / 7.2.8-200.fc44.x86_64 |
| Rust | 1.98.1; commit 48a229ceaefd4985c50990b14116b6d856af0985; Fedora 1.98.1-1.fc44 |
| Cargo | 1.98.1; commit 797e8a9bca276c1c9f9f738d2a20f484fa4eea9d; Fedora 1.98.1-1.fc44 |
| Rust LLVM | 22.1.8 |
| GCC / C++ | 16.2.1 20260819; Fedora 16.2.1-2.fc44 |
| CMake | 4.3.0; Fedora 4.3.0-1.fc44 |
| Clang / libclang | 22.1.8; clang, clang-libs and clang-devel 22.1.8-4.fc44 installed |
| pkg-config / Make / Git | 2.5.1 / 4.4.1 / 2.55.0 |
| rustup | Absent; not required with the installed Fedora toolchain |
| liboqs | Still absent from pkg-config and linker cache |
| oqs / oqs-sys | Not fetched, built or pinned |

A disposable std-only Rust program compiled with `rustc --edition=2024` and ran
successfully, printing `Rust compiler readiness only; no liboqs linked`. Source and
binary were removed automatically. This proves local Rust compilation/execution,
not Cargo dependency resolution, C library linkage or algorithm availability.

### Supported liboqs and ordered integration evaluation

The [upstream security policy](https://github.com/open-quantum-safe/liboqs/security/policy)
explicitly supports only the most recent release: 0.16.0; versions below 0.16 are
unsupported. The [0.16.0 release](https://github.com/open-quantum-safe/liboqs/releases/tag/0.16.0)
is the preferred target. The earlier 0.13.0 candidate is withdrawn from consideration
without explicit user approval. No such approval was requested or granted.

1. **Current liboqs-rust main, pinned commit:** unresolved. The retrieved
   [main manifest](https://raw.githubusercontent.com/open-quantum-safe/liboqs-rust/main/oqs-sys/Cargo.toml)
   still reports 0.11.0+liboqs-0.13.0. Retrieved main files have different web crawl
   dates, so they cannot establish a coherent current Git revision or submodule pin.
   `git ls-remote` for main failed with exit 128 (DNS). Do not claim current main
   is compatible or definitively incompatible until its exact tree can be inspected.
2. **Configure oqs-sys with local liboqs 0.16.0:** the published 0.11.0 build script
   accepts only >=0.13.0,<0.14.0. The retrieved
   [main build script](https://raw.githubusercontent.com/open-quantum-safe/liboqs-rust/main/oqs-sys/build.rs)
   also derives a same-minor range from package metadata. PKG_CONFIG_PATH alone
   cannot override that range. LIBOQS_NO_VENDOR=1 prevents fallback; it does not
   enable compatibility. Do not falsify liboqs.pc or suppress the version check.
   A reviewed binding patch could be evaluated, but is not configuration-only:
   current header/ABI, generated bindings and higher-level API assumptions must be
   checked and built. No such patch or build has been performed.
3. **Minimal audited FFI to supported liboqs:** preferable in principle to an
   unsupported library if options 1/2 prove unsuitable, but not yet selected or
   audited. A readiness-only boundary can use opaque OQS_KEM/OQS_SIG pointers,
   OQS_init/version, enabled checks, new/free, and OQS_destroy without key operations.
   It must use exact declarations checked against the pinned headers, no copied
   struct layouts, checked null returns and balanced cleanup. C accessors compiled
   against those headers can expose metadata without Rust layout assumptions.
   This reduces binding surface but transfers ABI/unsafe maintenance to the project;
   it is not a custom cryptographic primitive. Missing source prevents an actual
   liboqs build/probe, so this option has not been demonstrated safe or ready.
4. **Unsupported release:** not used. No downgrade is authorized.

The current version's public headers define the exact requested identifiers:
[OQS_KEM_alg_ml_kem_768 = ML-KEM-768](https://github.com/open-quantum-safe/liboqs/blob/0.16.0/src/kem/kem.h)
and [OQS_SIG_alg_ml_dsa_65 = ML-DSA-65](https://github.com/open-quantum-safe/liboqs/blob/0.16.0/src/sig/sig.h).
Header evidence does not establish local enablement. Neither algorithm was accessed
through a Rust/liboqs binary. Linkage (static/dynamic): NOT ESTABLISHED. Effective
liboqs build flags: NONE, because no build occurred. Exact source commit, archive
SHA-256, crate revision and lockfile: NOT ESTABLISHED; release name alone is not a
verified source pin. All Phase 1 PQC tests remain PLANNED.

### Remaining access blocker and recovery

Both GitHub and crates.io still fail DNS resolution from the shell. No liboqs source
or crate bundle was found among inspected paths in the project, /tmp, Downloads or
~/.cargo; /usr/share/cargo/registry is empty and ~/.cargo does not exist. Web pages
are available for research but are not a complete verified build-source bundle.
No attempt was made to evade the restricted execution surface or change networking.

Provide this session with normal permitted source access, or place verified liboqs
0.16.0 sources and an exact liboqs-rust snapshot (including its referenced submodule)
in an accessible workspace directory. For the binding route, transitive Cargo
sources must also be available. Re-check main first, then evaluate the local-library
route and only then the minimal FFI fallback. Build into a workspace-local prefix
or temporary directory; no system installation is needed. Record hashes, effective
CMake flags, Cargo revisions if used, library version and runtime enablement before
resolving Q-01. The fallback without third-party Rust crates still needs liboqs source.

Read-only recovery checks:

```sh
cd /home/iamajay/Projects/FP1
python3 scripts/inspect_environment.py
rustc --version --verbose
cargo --version --verbose
clang --version
cmake --version
git ls-remote https://github.com/open-quantum-safe/liboqs-rust.git refs/heads/main
git ls-remote https://github.com/open-quantum-safe/liboqs.git refs/tags/0.16.0 'refs/tags/0.16.0^{}'
curl -I --max-time 15 https://crates.io
pkg-config --modversion liboqs
git status --short
make check
```

### Git readiness update

Git reads now succeed: HEAD is `062a898` (Phase 0 project foundation), and initial
status showed only the user's existing `.gitignore` modification. The previous
empty-mount diagnosis is historical. No initialization/repair was needed or attempted;
the user's modification is preserved. Q-09's read-access blocker is resolved.
Git metadata remains outside authorized write scope; no commit/write operation was
attempted or validated. Q-01 remains BLOCKED; compiler availability does not close it.

## Q-01 final local-source evaluation — 2026-10-06

This section supersedes every earlier BLOCKED readiness conclusion in this document.
The supplied local source trees removed the source-access blocker. Actual successful
build and runtime metadata/linkage evidence is in
[PHASE_1_Q01_READINESS.txt](PHASE_1_Q01_READINESS.txt). Historical evidence files
are retained unchanged. Q-01 is RESOLVED for environment readiness only.

### Host and exact source provenance

Fresh inspection: Fedora 44, x86_64; Rust 1.98.1
(`48a229ceaefd4985c50990b14116b6d856af0985`), Cargo 1.98.1
(`797e8a9bca276c1c9f9f738d2a20f484fa4eea9d`), GCC/C++ 16.2.1
(Fedora 16.2.1-2.fc44), CMake 4.3.0, pkg-config 2.5.1, Git 2.55.0,
Clang/libclang 22.1.8, GNU Make 4.4.1. rustup is absent and unnecessary for the
selected system Rust toolchain. No further host packages were installed.

| Source | Exact revision / version | Working tree |
|---|---|---|
| `.deps/liboqs-0.16.0` | tag `0.16.0`, `5a1a854b0dc9f2141bdc771c555ee60c37950183`; CMake version 0.16.0, no prerelease suffix | CLEAN |
| `.deps/liboqs-rust-main` | `c6fd1a68179255b2461800e7da61231d7d3c61da`; `oqs 0.11.0`, `oqs-sys 0.11.0+liboqs-0.16.0-rc1` | CLEAN |
| Rust checkout's `oqs-sys/liboqs` submodule | `2966c35b3b7691eda295d465b500673377250810`; describes as `0.16.0-11-g2966c35b3`; CMake version 0.16.0 | CLEAN, matches recorded submodule revision |

SHA-256 of `git archive HEAD` for the release source:
`2a469ff48e73c9cd78c2f7752465ca7a066582fbaa9185f52b4725783218d39d`.
Rust repository archive:
`de354d6348c8e34db63439d9ae6e69dfe8b464e15d0d0f49d411ea79000ea1f8`.
Its liboqs submodule archive:
`25270f09ae5a1e353fc4ce62321ddeeba8e0b35a7d3a05645d427a62ab43ad33`.
The Rust repository archive itself does not include submodule contents.
These are locally calculated integrity fingerprints, not independently verified
release-signature attestations.

The release's local `SECURITY.md` and the current
[upstream security policy](https://github.com/open-quantum-safe/liboqs/security/policy)
list 0.16.0 as supported and releases below 0.16 as unsupported. The
[release page](https://github.com/open-quantum-safe/liboqs/releases/tag/0.16.0)
identifies the matching release commit. Recheck support before future deployment.

### Rust binding evaluation in the required order

A. **Current checkout with pinned vendored liboqs:** source inspection shows an
updated ML-KEM/ML-DSA-capable wrapper and 0.16-series submodule, rather than the
older cached 0.13 views. The vendored revision is eleven commits after the release;
it is not the exact supported release tag. `oqs-sys/build.rs` builds that submodule,
links it statically on Linux and runs bindgen over its installed `common.h`, `rand.h`,
`kem.h` and `sig.h` (STFL excluded). The proper restricted features would be
`default-features=false`, `std,ml_kem,ml_dsa,vendored`; defaults enable extra
algorithms and OpenSSL. No complete vendored-crate build is claimed: an isolated
`cargo check --offline` failed at dependency resolution, exit 101, missing
`cstr_core` in an empty isolated Cargo home. This is not a demonstrated source/API
incompatibility. The untagged submodule and additional unavailable offline Rust
build dependencies make A unsuitable for this readiness selection.

B. **Current checkout with local release 0.16.0:** its build script derives the
range `>=0.16.0-rc1,<0.17.0` from the crate metadata. Actual pkg-config 2.5.1
reports the built library as `0.16.0`, but exits 1 for that range. Therefore the
unmodified build script cannot select this release here; without
`LIBOQS_NO_VENDOR=1` it would silently fall back to the different submodule, and
with that variable it would fail. Bindgen follows whichever headers are selected;
the C interfaces are generated at build time, with no release-binding build proven. No complete
external-crate build is claimed. No third-party source, submodule or `.pc` version
was altered to force compatibility.

The local high-level wrappers expose standardized `MlKem768` and generated
`MlDsa65` algorithm variants, public size accessors, and ML-DSA sign/verify context
methods. Empty Rust contexts become NULL with length zero. Context support is
queried from the signature object. This source compatibility evidence does not
replace the failed external-library selection or an actual crate build.

C. **Selected route:** a small manually declared Rust FFI boundary to the exact
liboqs 0.16.0 C API, with opaque KEM/signature handles and a header-compiled C
accessor for metadata. This avoids duplicated C struct layouts, bindgen/libclang
and transitive Cargo packages. No `oqs` or `oqs-sys` crate is used by the successful
probe. Production operations, ownership/error handling and zeroization still need
implementation and review during an explicitly approved Phase 1. The probe is
only evidence that this dependency route works.

### Independent release build and runtime availability

Commands (the executed build paths and full outputs are in the evidence file):

```sh
cmake -S .deps/liboqs-0.16.0 -B "$probe_root/build" \
  -DCMAKE_BUILD_TYPE=Release -DBUILD_SHARED_LIBS=OFF \
  -DOQS_BUILD_ONLY_LIB=ON -DOQS_DIST_BUILD=ON -DOQS_USE_OPENSSL=OFF \
  '-DOQS_MINIMAL_BUILD=KEM_ml_kem_768;SIG_ml_dsa_65' \
  -DCMAKE_INSTALL_PREFIX="$probe_root/prefix"
cmake --build "$probe_root/build" --parallel 2
cmake --install "$probe_root/build"
```

Configure, build and install all exited 0. Two isolated builds succeeded: initial
inspection build and the retained probe's complete reproduction. Minimal-build
filtering overrides the broader defaults shown in CMakeCache; the generated
`oqsconfig.h` enables only the ML-KEM-768 and ML-DSA-65 variants, their family flags
and x86_64 optimized backends. `OQS_DIST_BUILD=ON` keeps runtime CPU dispatch and
portable fallbacks. Internal SHA3 and OS randomness are used without OpenSSL;
no random operation was requested by this probe. Static liboqs archive from the
recorded reproduction has SHA-256
`00adeed4b3e7cea298da5b9dca810dffb93e26456febadd7947dd409295eb0f9`.
Artifact hashes can change with toolchains, paths and build options; they are
fingerprints of this run, not a bit-for-bit reproducibility claim.

`OQS_version`, `OQS_KEM_alg_is_enabled`, `OQS_SIG_alg_is_enabled`, `OQS_KEM_new`
and `OQS_SIG_new` were called successfully from Rust. Struct sizes were read by C
compiled against the installed headers and checked by Rust:

| Identifier | Public key | Secret key | Ciphertext / signature | Shared secret |
|---|---:|---:|---:|---:|
| `ML-KEM-768` (`OQS_KEM_alg_ml_kem_768`) | 1184 | 2400 | 1088 ciphertext | 32 |
| `ML-DSA-65` (`OQS_SIG_alg_ml_dsa_65`) | 1952 | 4032 | 3309 signature | N/A |

Both enabled checks returned 1, both constructors returned non-null, and runtime
version was exactly `0.16.0`. Signature context support and ordinary/context
function-pointer availability were verified through the C struct; support = 1.
No keys, signatures, ciphertexts, shared secrets or project identities were generated.

### FIPS API semantics

Source inspected: `src/kem/ml_kem/kem_ml_kem_768.c`,
`src/sig/ml_dsa/sig_ml_dsa_65.c`, and the mldsa-native backend `src/sign.c`.
ML-KEM constructor identifies FIPS203 and calls mlkem-native, not Kyber.
ML-DSA constructor identifies FIPS204; its ordinary sign/verify functions pass
NULL,0 to the pure ML-DSA backend on the portable/x86_64/aarch64 dispatch branches.
The backend prepares the pure domain prefix with `MLD_PREHASH_NONE`, yielding
`0x00 || 0x00` for empty context. Separate external-mu and prehash paths are not
selected. Explicit `OQS_SIG_sign_with_ctx_str`/verify with NULL,0 is also available.
This agrees with CRYPTO_PROTOCOL.md's pure ML-DSA empty-context requirement;
the protocol was not changed. Actual sign/verify correctness is still PLANNED.

### Rust linkage proof, cleanup and reproduction

The retained [probe script](../scripts/probe_liboqs_readiness.py) is justified as a
small repeatable offline environment check, isolated from all production modules.
It pins and checks the clean release source, builds in a TemporaryDirectory,
compiles a metadata-only C accessor with `-Wall -Wextra -Werror`, creates a
zero-dependency temporary Cargo project, runs `cargo fmt --check`, compiles with
`cargo rustc --offline` and explicit `-l static=oqs`, checks runtime metadata, and
runs `readelf -d`. Every expected command succeeded; the binding-range rejection
is explicitly expected exit 1. Cargo build took 0.17 seconds in this run.
`readelf` shows only libgcc_s, libc and the ELF loader as dynamic dependencies;
liboqs is static, not a system shared library. Binary SHA-256 for this run:
`bed3ea6f9ec352245485018101ef27a87a46671aea9c67f3ba735282ec91a8a8`.
Both initial and reproduction temporary build/install/probe trees were removed.
Only the script and text evidence are retained; no liboqs system installation exists.

Exact manual reproduction from the project directory:

```sh
python3 scripts/probe_liboqs_readiness.py
# Expected final line: Temporary build/install/probe artifacts removed; Q-01 probe PASS
# Expected runtime version 0.16.0 and the size table above.
git -C .deps/liboqs-0.16.0 status --short
git -C .deps/liboqs-rust-main status --short
git check-ignore .deps/liboqs-0.16.0 .deps/liboqs-rust-main
git diff --check
```

The script prints all expanded CMake/GCC/Cargo/readelf commands and actual exits.
It operates offline with an isolated empty Cargo home and no crate dependencies.
Keep the pinned ignored source trees for reproducibility; retaining them is not yet
Phase 13 offline-distribution packaging. `.deps/` remains ignored, Git read commands
work at project HEAD `200bc4ab96a8d8fdf3bc5ffa846a668de7f375c8`; Git writes were not
attempted because metadata is outside the writable execution scope.

### Remaining scope and maintenance

No Phase 1 environment blocker remains for the selected route. MEDIUM follow-up:
maintain and audit the small project-owned FFI boundary as liboqs changes; this probe
does not validate cryptographic operations, secret lifetimes, malformed inputs,
constant-time behavior or FIPS module certification. Static linking requires rebuilds
for upstream security updates. The common C API is also a plausible Phase 6 Go/cgo
boundary; Go/Fabric compilation, deterministic verification and chaincode deployment
remain Q-04 and were not tested. No later-phase dependency was installed.
All `PQC-KEM-001..004` and `PQC-SIG-001..005` remain PLANNED. The specification,
cryptographic protocol, test matrix, phase gates and production scaffold are unchanged.
Phase 1 implementation requires explicit user approval.

## Authorized Phase 1 implementation — 2026-10-06

Supersedes the readiness-only scope above. User explicitly authorized primitive
integration only. `trusted-client/Cargo.toml` is a dependency-free Rust 2024 crate;
Cargo.lock contains only `cadp-pqc 0.1.0`. No oqs/oqs-sys crate is used. The approved
release remains tag `0.16.0`, commit
`5a1a854b0dc9f2141bdc771c555ee60c37950183`; no upstream files were modified.
Tool versions remain Rust/Cargo 1.98.1, GCC/C++ 16.2.1, CMake 4.3.0, Clang 22.1.8,
pkg-config 2.5.1 and Git 2.55.0 on Fedora 44 x86_64. System packages/configuration
were not changed. `.deps/` and Cargo target trees remain ignored.

`build.rs` checks HEAD, tag, clean status including untracked/ignored artifacts,
and absence of assume-unchanged/skip-worktree flags. It checks again after build.
Source defaults to `.deps/liboqs-0.16.0`; `CADP_LIBOQS_SOURCE` can select another
checkout only if it passes the same pin checks. Missing/invalid source fails;
there is no automatic download, vendored fallback or system-library discovery.
Source-pin tests mutate only a disposable local clone, including an empty commit
because the supplied original repository is shallow. The actual source stays clean.

The approved liboqs configuration is unchanged:

```text
CMAKE_BUILD_TYPE=Release
BUILD_SHARED_LIBS=OFF
OQS_BUILD_ONLY_LIB=ON
OQS_DIST_BUILD=ON
OQS_USE_OPENSSL=OFF
OQS_MINIMAL_BUILD=KEM_ml_kem_768;SIG_ml_dsa_65
```

Additional build plumbing: `CMAKE_INSTALL_LIBDIR=lib`, `CMAKE_C_COMPILER=cc`, and
an install prefix beneath Cargo OUT_DIR. The C shim is compiled with
`-std=c11 -O2 -fPIC -Wall -Wextra -Wpedantic -Werror`, then archived with `ar rcs`.
Rust links static `cadp_oqs_shim` and `oqs` plus system pthread. `-fPIC` corrected
an actual initial Fedora PIE link failure; no liboqs source/options were changed.
Concurrent use requires the generated `OQS_USE_PTHREADS` macro; the shim refuses
an OpenSSL-enabled or pthread-disabled configuration. Runtime initialization checks
version `0.16.0`, exactly one enabled KEM and signature, FIPS203/FIPS204 metadata,
function availability and all seven standardized sizes before operations.

The backend is initialized once by Rust OnceLock, with cached fail-closed errors.
Native handles are local to C and freed before each return. A same-thread cleanup
guard invokes `OQS_thread_stop` after each native operation. There is deliberately
no `OQS_destroy` call or global shutdown API: initialization lives until process
exit, and destroy is a no-op in this OpenSSL-free release configuration. Tests
observe one initialization and matching per-operation thread cleanup; fresh-process
concurrent startup is checked separately.

Only KEM/signature/shared-secret buffers are secret owners. Fallible fixed-size heap
allocations stay in one private non-cloneable buffer and are cleansed with
`OQS_MEM_cleanse` before deallocation. Backend copies, fatal exits and Linux
memory-hardening limits are documented; Phase 5 controls are not implemented here.
No secret or project identity is written/logged. All test keys are ephemeral.

The C contract harness uses Clang `-fsanitize=address,undefined`, strict warnings
and disposable linker wrappers for native failure injection. GCC instrumentation
was attempted but failed because visible libasan/libubsan runtimes are missing.
Clang's compiler-rt works. LeakSanitizer was attempted and failed due to ptrace
restrictions; default tests explicitly use `detect_leaks=0`, with native live-handle
counts asserting cleanup. Enable `CADP_TEST_LEAKS=1` only on a suitable unrestricted
host. No successful LeakSanitizer run is claimed. The normal backend archive and
its assembly are not sanitizer-instrumented by this check.

Manual commands from the repository root:

```sh
export CARGO_HOME="$PWD/.deps/cargo-home"
cargo fmt --manifest-path trusted-client/Cargo.toml --check
cargo clippy --offline --locked --manifest-path trusted-client/Cargo.toml --all-targets -- -D warnings
cargo test --offline --locked --manifest-path trusted-client/Cargo.toml
cargo test --offline --locked --release --manifest-path trusted-client/Cargo.toml
python3 scripts/check_pqc_native.py
python3 scripts/check_pqc_source_pin.py
make check
git diff --check
```

Expanded configure/compiler commands and archive hash are recorded by each build
in `trusted-client/target/{debug,release}/build/cadp-pqc-*/out/build-evidence.txt`.
Phase evidence is recorded in [PHASE_1_VALIDATION.txt](PHASE_1_VALIDATION.txt); the
review and unsafe inventory are [PHASE_1_REVIEW.md](PHASE_1_REVIEW.md).
Historical Phase 0 checks/evidence are preserved: current documentation checks
reuse applicable tests and add Phase 1 scope checks rather than pretend the tree
is still scaffold-only. Later-phase commands fail clearly. Go/Fabric, GUI, PDF,
ECC, AES-GCM, HKDF, PKI and key-bundle dependencies remain outside this phase.

## Authorized Phase 1 security corrections — 2026-10-07

Independent Astra review identified H-01 in the original AVX-512 SHA3 configuration
and L-01 in persistent cleanup coverage. User approved these corrections only.
The exact release remains liboqs 0.16.0, commit
`5a1a854b0dc9f2141bdc771c555ee60c37950183`; its source remains clean and unpatched.
No crate, compiler, system package, host configuration or protocol was changed.
Rust/Cargo 1.98.1, GCC 16.2.1, Clang 22.1.8 and CMake 4.3.0 remain the tested tools.

Current required liboqs options (D-20 adds the explicit SHA3 exclusion):

```text
CMAKE_BUILD_TYPE=Release
BUILD_SHARED_LIBS=OFF
OQS_BUILD_ONLY_LIB=ON
OQS_DIST_BUILD=ON
OQS_USE_OPENSSL=OFF
OQS_USE_SHA3_AVX512VL=OFF
OQS_MINIMAL_BUILD=KEM_ml_kem_768;SIG_ml_dsa_65
```

`build.rs` validates `include/oqs/oqsconfig.h` immediately after configuration,
before compilation, and again in the installed prefix. The pinned template must
emit exactly `/* #undef OQS_USE_SHA3_AVX512VL */`; missing, defined or unfamiliar
forms fail closed. The C shim independently rejects any definition of this macro,
including a zero value. Its existing pthread/OpenSSL compile guard remains active.
Runtime metadata still checks version, exactly the two algorithms and all seven sizes.

Validation used an initially nonexistent temporary CARGO_TARGET_DIR, so debug,
release and native artifacts were freshly compiled. Generated and installed headers
show SHA3 AVX512VL and OpenSSL undefined, with OQS_USE_PTHREADS defined as 1.
The resulting archives contain no avx512vl symbols. A separate CMake cache seeded
with SHA3 AVX512VL ON was reconfigured OFF and rebuilt by the actual build script.
Exact paths, commands, generated configuration, archive fingerprints and results
are in [PHASE_1_CORRECTIONS_VALIDATION.txt](PHASE_1_CORRECTIONS_VALIDATION.txt).
Artifact hashes identify this run, not bit-for-bit reproducibility across toolchains.

The host lacks AVX-512; no AVX-512 hardware execution or ThreadSanitizer result is
claimed. The affected implementation is excluded from the approved build. Remaining
internal SHA3 dispatch pointers use pthread_once, and RNG callbacks are not changed.
OQS_thread_stop/OQS_destroy remain no-ops in this OpenSSL-free configuration.

`python3 scripts/check_pqc_native.py` now also runs PQC-BUILD-002 (generated-header
and compile rejection) and PQC-SECRET-003 (four Rust error-path cleanup cases).
Cargo JSON identifies the artifacts from that invocation, rather than selecting a
potentially stale build by modification time. The script honors CARGO_TARGET_DIR
for fresh-build reproduction. Partial-output fixtures use only synthetic public
patterns and are not sanitizer-instrumented; the original four C shim contracts
retain Clang ASan/UBSan with leak scanning disabled. M-01/L-19 is retained.

Clean manual reproduction, without deleting any existing project target:

```sh
export CARGO_HOME="$PWD/.deps/cargo-home"
correction_target=$(mktemp -d /tmp/fp1-correction-check.XXXXXX)
export CARGO_TARGET_DIR="$correction_target"
make build
make check
cargo test --offline --locked --release --manifest-path trusted-client/Cargo.toml
cargo test --offline --locked --manifest-path trusted-client/Cargo.toml pqc_conc_001 -- --test-threads=1
rg -n 'OQS_USE_SHA3_AVX512VL|OQS_USE_PTHREADS|OQS_USE_OPENSSL' "$correction_target"/debug/build/cadp-pqc-*/out/liboqs-install/include/oqs/oqsconfig.h
```

H-01/L-01 corrections are TESTED; Astra re-audit is pending. Phase 2 remains unapproved.
Historical PHASE_1_VALIDATION.txt and Phase 0 evidence are unchanged.

## Authorized Phase 2 dependency inspection — 2026-10-07

Supersedes the historical pending re-audit/approval statements above. Phase 1's
independent [Astra re-audit](PHASE_1_ASTRA_REAUDIT.txt) returned APPROVED. The user
explicitly authorized Phase 2 only; Phase 3 remains unapproved. Phase 2 is currently
BLOCKED before implementation by missing offline cryptographic dependencies.

Fresh Rust/Cargo checks still report 1.98.1; liboqs source is clean at the approved
`5a1a854b0dc9f2141bdc771c555ee60c37950183` commit. Project Git reads work at HEAD
`a157ec7a4127e718c225a4e4f1132e2b774d5e5e`, with Phase 1 committed as `f5b2990`.
No Git metadata, source pin, compiler or host configuration was changed.

The project Cargo home has no cached crate sources. The user registry cache has
terminal/support crates but no `aes-gcm` or `argon2`. Searches found no alternative
offline vendor bundle in inspected repository, /tmp, /opt, /var/cache or system
Cargo paths. Four isolated offline lockfile-resolution probes (each package against
each cache) failed with exit 101, reporting no matching package. None produced a
lockfile. The user cache was tested through a disposable /tmp copy; all probes were
removed. No downloads or cryptographic operations occurred.

The user's explicit AES-GCM dependency stop condition applies. Supply reviewed,
exactly pinned offline sources and their complete transitive dependency closure
before implementation; no Phase 2 crate versions are approved by these availability
probes. See [PHASE_2_REVIEW.md](PHASE_2_REVIEW.md) for reproduction/recovery and
[PHASE_2_VALIDATION.txt](PHASE_2_VALIDATION.txt) for actual failures. No Argon2 benchmark
ran, no new unsafe boundary was added, and Phase 2 runtime tests remain PLANNED.

## Phase 2 vendor dependency audit — 2026-10-07

The owner's `.deps/phase2-vendor` now resolves/compiles the supplied 34-crate locked
graph offline in an isolated empty Cargo home: 31 crates on x86_64 Fedora. Host
rustc/Cargo 1.98.1; graph minimum Rust 1.89 (aes 0.9.3). Exact candidates: aes-gcm
0.11.1, argon2 0.6.0, sha2 0.11.0, getrandom 0.4.3, rpassword 7.5.4. All 1,421
Cargo source checksums and 1,455 owner inventory entries agree, with exact file sets.
Minimal functional defaults-off graph also compiled and API probes passed; these
are dependency checks, not Phase 2 tests or production parameter benchmarks.

Original AES/Argon2 source-absence blocker is superseded. Gate remains BLOCKED:
zeroize is missing, so the proposed AES/Argon2 cleanup-enabled features fail offline
resolution. Argon2 additionally requires caller-owned drop-cleansed workspace; its
internal convenience allocation does not cleanse on Drop. Linux syscall-only RNG
configuration was tested, including forced EPERM without fallback; it must be enforced
when production integration is separately authorized. Default terminal no-echo was
tested via public PTY input. No Cargo source config was installed in the project.

Complete pins, features, license/checksum inventory, scope caveats and reproduction:
[PHASE_2_REVIEW.md](PHASE_2_REVIEW.md), actual evidence:
[PHASE_2_VALIDATION.txt](PHASE_2_VALIDATION.txt). Bundle remains ignored. No network,
implementation or production keys. All 19 Phase 2 runtime IDs remain PLANNED.


## Phase 2 refreshed cleanup and design gate — 2026-10-07

This supersedes the historical missing-zeroize blocker, preserving earlier evidence.
Fresh rustc: 1.98.1 (48a229ceaefd4985c50990b14116b6d856af0985); Cargo 1.98.1
(797e8a9bca276c1c9f9f738d2a20f484fa4eea9d), Fedora 44 x86_64 Linux.
The owner's refreshed ignored vendor tree contains exactly 31 registry packages,
1367 Cargo-checksummed source files and 1398 owner SHA-256 inventory entries;
all sets/checksums matched, no symlinks. Zeroize is exactly 1.9.1, Apache-2.0 OR MIT,
registry checksum e13084392c5e4bc371903e2935a5eaeed24905a7511356b883835e18a78f6879.

Disposable build used a newly empty CARGO_HOME and fresh target directory, source
replacement exclusively to .deps/phase2-vendor, net.offline=true and --offline
--locked. Root lock was initially byte-identical to the supplied bootstrap lock.
The exact six direct pins include zeroize and match the cleanup-enabled metadata:
aes-gcm 0.11.1 defaults off/aes+zeroize; argon2 0.6.0 defaults off/zeroize;
sha2 0.11.0, getrandom 0.4.3, rpassword 7.5.4 defaults off;
zeroize 1.9.1 defaults off/alloc. Fedora OS RNG is constrained with
--cfg getrandom_backend="linux_getrandom" (no fallback introduced).

That graph compiled 28 host registry crates plus the disposable root. A public-fixture
API probe compiled and ran: AES expanded-key ZeroizeOnDrop trait, AES-256-GCM round
trip and safe Argon2id v0x13 caller-owned Zeroizing<Vec<Block>> plus 32-byte output.
It checked explicit output wiping only; no claim of observing destroyed backend memory.
Its tiny workspace was an API fixture, NOT the production Argon2 benchmark/profile.
Additional isolated offline builds enabled sha2 zeroize and directly pinned already
transitive blake2 0.11.0 defaults off/zeroize to activate relevant digest/core cleanup;
no new package sources were needed. Production manifests were not changed.

Remaining required source delta: minicbor =2.3.0 (defaults off, no features),
hmac =0.13.0 (defaults off, zeroize), hkdf =0.13.0 (defaults off, no features).
All three failed disposable offline resolution with exit 101 because absent from the
vendor tree. Proposed CBOR/PoP design is documented, not compiled or implemented.
An owner-supplied refreshed lock/metadata/checksum inventory/vendor closure is needed
before production work. No dependency download happened during this task; read-only
upstream documentation/source research is distinguished from network-free Cargo
builds. No package/advisory authenticity guarantee or full dependency audit claimed.

Phase 2 remains unimplemented and its 19 runtime IDs PLANNED. Q-02 PoP design and
Q-03 local time boundary are recorded in KEY_MANAGEMENT.md; benchmark/recovery and
actual trusted-time/host-rollback remain open. No new system/later-phase component,
production secret, raw secret accessor, unsafe code or Phase 1 configuration was added.
See PHASE_2_REVIEW.md and PHASE_2_VALIDATION.txt for details and exact reproduction.


## Phase 2 FINAL dependency verification — 2026-10-07

Current Phase 2 dependency gate: READY. This supersedes the preceding missing
minicbor/HMAC/HKDF source conclusions without rewriting historical results.
Actual refreshed integrity: 34 registry packages, 1435 Cargo-checksummed source
files, 1469 owner inventory entries; exact sets/versions/checksums, no symlinks.
All ten approved direct pins and feature intent match owner and independently
resolved metadata. Exact package pins/features and checksum fingerprints are recorded
in PHASE_2_REVIEW.md and PHASE_2_VALIDATION.txt.

Fresh host checks still report rustc/Cargo 1.98.1 on Fedora 44 x86_64 Linux.
A disposable project with an initially empty CARGO_HOME and nonexistent target
compiled 31 Linux registry crates plus its root using ONLY the vendor directory,
--offline --locked and net.offline=true. Build/check/run and full/Linux metadata plus
feature tree succeeded. The copied lock remains byte-identical to the owner lock.
The probe configured getrandom_backend="linux_getrandom"; production must enforce
that policy separately rather than assume the crate defaults select it.

Minimal minicbor 2.3.0 Decoder/Encoder types instantiate without optional features.
HMAC 0.13.0 + SHA-256 passes a public RFC 4231 vector and modified/short tag rejection;
HKDF 0.13.0 + SHA-256 passes RFC 5869 extract/expand and direct 32-byte output.
No CBOR encoding/parser, provisioning proof, certificate, identity, storage or CRL was
implemented. No production Argon2 benchmark or terminal/RNG runtime rerun claimed.
No network, package download, system configuration or production manifest change.
The supplied .deps inputs remain Git-ignored and untouched; disposable files removed.

Only existing getrandom/libc have custom build targets; no build-dependency edges or
proc macros. Added crates have no native code/build scripts. Minicbor has four Rust
unsafe blocks for ByteSlice conversions and initialized array ownership; HMAC/HKDF
contain no unsafe syntax in their Rust sources and forbid unsafe code via crate lints.
This is triage, not full dependency memory-safety/security audit or FIPS certification.
Existing cleanup/host-rollback/M-01 limitations remain. All 19 runtime IDs PLANNED;
only the dependency availability gate is closed. Await user direction before production.


## Phase 2A foundation implementation — 2026-10-08

Authorized scope: deterministic CBOR and utility foundation only; full Phase 2 is
NOT COMPLETE. All 19 PKI runtime IDs remain PLANNED. No certificate, provisioning,
CA, bundle, CRL, revocation, document workflow or Phase 3 implementation.
Historical dependency-gate and Phase 0/1 evidence remain unchanged.

The component now integrates D-26's exact ten pins and minimal features in
trusted-client/Cargo.toml/Cargo.lock. Rust minimum is 1.89 for the approved graph;
actual compiler/Cargo remains Fedora 1.98.1, x86_64-unknown-linux-gnu. Source
replacement lives in .cargo/config.toml; .deps/phase2-vendor remains ignored.
Cargo uses ONLY local directory sources and offline/locked mode. No downloads or
host changes. Integrity: 34 packages / 1435 Cargo files / 1469 owner hashes;
Linux resolution 31 packages, exact requested features. Checksum consistency is
not supplier authentication, a new advisory audit, legal approval or FIPS validation.

Cargo-wide rustflags explicitly select getrandom_backend="linux_getrandom";
build.rs rejects absent, duplicated, alternate or ambiguous backend flags.
Rustdoc needs the matching rustdocflags separately for its recompilation. The crate
also has a Linux/backend compile guard. Consumers building from another directory
must supply the same dependency-wide configuration; it fails closed without it.
ENOSYS/EPERM failure controls invoke the actual public RandomId API; no OS-device
or custom RNG fallback. Invalid flag configurations fail building. The conflicting
linux_getrandom/linux_raw control was rejected by getrandom's dependency setup
before the project guard ran; the guard predicate is independently unit-tested.

The liboqs commit/version, audited CMake arguments, shim, FFI, PQ secret ownership,
static linkage, pthread requirement, OpenSSL OFF and AVX512 SHA3 OFF are unchanged.
Fresh target builds revalidate generated/installed configuration and runtime sizes.
Project source requires a clean exact Git checkout as before. Registry dependencies
use the owner's final reviewed source bundle rather than an arbitrary system cache.

Actual checks and initial failed attempts are in PHASE_2A_VALIDATION.txt; scope,
self-review, cleanup limitations and demonstration are in PHASE_2A_REVIEW.md.
No Argon2 production benchmark, protected password input implementation, true fuzzing,
TSan, successful LeakSanitizer, AVX-512 execution or full-backend sanitization claim.


## Phase 2A P2A-A-M01 corrective build policy — 2026-10-12

The normal default configuration still selects only getrandom 0.4.3's
linux_getrandom syscall backend. The prior literal-only guard accepted hidden cfgs
in rustc response files; independent Astra evidence is retained unchanged.

Build.rs now rejects every @response-file token in effective Cargo encoded Rust
flags, validates exactly one approved literal and requires Cargo's effective backend
cfg to be exactly linux_getrandom. It also checks exposed Rust/rustdoc environment
flag sources. CARGO_ENCODED_RUSTFLAGS uses U+001F separators, not whitespace.
Cargo's precedence decides effective Rust flags; ignored lower-precedence settings
are not additional active cfgs. No project guard reads response-file contents.

A root compile-time guard independently forbids all eight other values recognized
by the pinned getrandom dispatcher; the complete nine-value set is declared in
check-cfg. This catches hidden CUSTOM after expansion even if textual validation is
bypassed. Multiplicity of identical cfgs is lost during cfg evaluation, so duplicate
approved options are rejected by argument validation rather than that compile guard.

Rustdoc's separately supplied configuration flags are not available to build.rs.
The portable checkout-relative build.rustdoc launcher scripts/guard_rustdoc.py checks
actual argv, rejects @files/conflicting/missing/duplicate backend options and then
execs PATH rustdoc without a shell. Build.rs fails closed if the selected launcher
does not resolve to that checked-in script. Python3 is an existing host prerequisite;
keep this script executable. Rustc/rustdoc/Python/Cargo/PATH and local sources remain
trusted build inputs, not a malicious-toolchain sandbox.

Validation used a fresh ignored verified-target and initially empty
.deps/p2a-m01-correction/verified-cargo-home, offline/locked vendor sources only.
Subsequent commands share that task-owned target/home; not every invocation is fresh.
P2A-BUILD-003 exercises 36 actual Cargo check/build/doctest rejections, including the
original hidden CUSTOM attack. P2A-BUILD-004 independently compiles root guard text
with rustc/rustdoc: each rejects eight alternatives and hidden CUSTOM; removing only
exclusivity makes both controls succeed. Existing ENOSYS/EPERM controls still run
the production RandomId API. Exact commands/output are in
PHASE_2A_CORRECTIONS_VALIDATION.txt; assessment in PHASE_2A_CORRECTIONS_REVIEW.md.

Pins/features, audited liboqs configuration, static linkage, algorithms, FFI and
foundation runtime implementations are unchanged. No Phase 2B/3 implementation or
independent correction approval is claimed.


## Phase 2A P2A-RA-L01 actual rustc argv enforcement — 2026-10-12

Cargo build.rustc-wrapper now selects the executable scripts/guard_rustc.py, alongside
the unchanged rustdoc launcher. Standard Cargo supplies the real tool and actual
argv, including per-crate cargo rustc extras that build.rs cannot see. No compiler
replacement, download, host change or hard-coded toolchain path. Python3 is already
required. Run supported commands from checkout root with its relative configuration.

The guard rejects response-file tokens before forwarding and shares rustdoc's strict
backend parser. Exactly one linux_getrandom cfg is required for runtime compilations;
identity/metadata-only host queries and host build-script tools may omit target cfgs.
No alternate/duplicate cfg or @file is permitted in those exceptions. Standard Clippy
nesting and explicit native --target builds are tested. Unrelated workspace wrappers
are unsupported. Build.rs rejects removal/replacement of the selected rustc wrapper.
Trusted compiler/PATH/source assumptions remain; this is build policy, not a sandbox.

Before correction, a fresh actual cargo rustc -- @duplicate.rsp produced a project
library with exit 0. After correction, P2A-BUILD-005 rejects all 23 per-crate cases
before any accepted project artifact in its initial stage, then rejects the same
cases with approved artifacts retained unchanged. The initial stage starts empty;
later failures share its dependency/native cache, not a fresh target per case.
A normal build populates the target between stages. Seven supported wrapper override
routes also reject. Existing 36 Cargo and 18 compile-time cfg controls are preserved.

Normal full regression uses an initially empty task-owned Cargo home and new
.deps/p2a-l01-correction/verified-target, local vendor only and offline/locked mode.
Subsequent commands share that target/home. Actual outputs and any initial corrective
failures are retained in PHASE_2A_L01_CORRECTIONS_VALIDATION.txt; final results are
recorded in PHASE_2A_L01_CORRECTIONS_REVIEW.md. No library pin, CMake option, runtime
algorithm, FFI, crypto/protocol byte or production secret API changed.

Corrective self-review additionally observed that Clippy supplies extra CLI flags
through CLIPPY_ARGS after the Cargo argv boundary. An actual duplicate @file
was accepted by the initial wrapper design. The final wrapper conservatively
rejects @ and getrandom_backend markers in that extra channel without parsing
Clippy's private separator. Ordinary warning flags remain supported. Six real
Clippy rejection controls preserve coverage. This is a corrected implementation
issue, not a new protocol decision or a hidden change to Astra history.

## Phase 2A P2A-FV-L01 cache-safe handshake correction — 2026-10-13

Before editing, an existing empty disposable target reproduced invalid Clippy exit
101 followed by valid strict-Clippy exit 101 without a cache reset. Its retained
.rustc_info.json records a failed guarded clippy-driver/rustc -vV invocation. Actual
argv tracing separately observed the fixed Cargo target-information query and
CLIPPY_ARGS separator __CLIPPY_HACKERY__. Direct real-driver tests confirm the
observed metadata/identity calls ignore that invocation-specific extra channel.
The tracing proxy is an isolated observation fixture, not an accepted project build.

D-32 now exempts only the enumerated exact handshakes from that channel check;
artifact-producing commands still enforce all policy. Generic --print exemptions
are removed. Query forms are reviewed for installed Cargo/Clippy 1.98.1 on native
x86_64-unknown-linux-gnu; future toolchain shapes require reinspection. Compiler,
Python, PATH and local tools remain trusted. No host configuration, dependency pin,
.cargo configuration, build.rs, liboqs or runtime cryptography change.

Initial P2A-BUILD-006 targeted run passed all five sequences on one retained target:
5 compilation rejections / 9 valid runs, with no .rustc_info.json deletion or cache
bypass. Full offline regression evidence is recorded separately in
PHASE_2A_FV_L01_CORRECTIONS_VALIDATION.txt. Existing Astra reports are historical and
unchanged; this correction is not independent Phase 2A approval. All 19 PKI rows
remain PLANNED, Phase 2B/3 unapproved.

Unreviewed Rust flags that alter the Cargo placeholder-query shape also fail closed
independently of CLIPPY_ARGS; ordinary Clippy lint flags remain supported. No generic
compiler-query parser is introduced.


Final verification: all 12 required commands returned 0. Final-source fresh debug
and release each passed 41 unit tests plus 3 doctests. Frozen-source policy suites,
make check, all five same-target sequences, 13 independent Clippy negatives with
valid recovery, ENOSYS/EPERM and Phase 1 checks passed. The original failed-cache
target also recovered without cache deletion. Recorded tool versions: cargo/rustc
1.98.1, clippy 0.1.98, Python 3.14.8. The first validation Cargo home was empty;
no network or dependency downloads were used. Phase 1 docs checks: 6 passed, Mermaid
render skipped (mmdc unavailable). Status: FIXED / READY FOR FINAL ASTRA CONFIRMATION.
