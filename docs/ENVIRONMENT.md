# Development Environment Inspection

Current status: Phase 1 environment preparation **BLOCKED**. The original Phase 0
inspection is preserved below; the fresh preparation findings follow it.

Inspection date: 2026-10-06, from `/home/iamajay/Projects/FP1` in the agent execution
surface. Read-only probes only; no dependency installation or host configuration
changes. Availability means visible to this session, not proof absent everywhere.

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
