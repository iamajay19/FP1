# Phase 1 — PQC Primitive Integration Review

Current gate: authorized H-01/L-01 corrections IMPLEMENTED and TESTED; independent
Astra re-audit pending. Phase 2 is unapproved. See section 14 and
[PHASE_1_CORRECTIONS_VALIDATION.txt](PHASE_1_CORRECTIONS_VALIDATION.txt).

Original implementation status (historical): Phase 1 COMPLETE, IMPLEMENTED and TESTED; separate self-review completed. Final gate command evidence
is recorded in [PHASE_1_VALIDATION.txt](PHASE_1_VALIDATION.txt). This is not the
user's forthcoming GPT-6 Astra review or an external audit. Phase 2 is unapproved.

## 1. Implemented scope

Dependency-free Rust primitive library `cadp-pqc`, using exactly liboqs 0.16.0,
tag `0.16.0`, commit `5a1a854b0dc9f2141bdc771c555ee60c37950183`.
Safe APIs generate independent ML-KEM-768 and ML-DSA-65 key pairs, encapsulate,
decapsulate, sign and verify. Public backend metadata is checked before operations.
There are no certificates, storage bundles, symmetric/document encryption,
serialization, ledger, watermark, state machine, forensics or GUI implementations.

## 2. Files created or modified

Created: `trusted-client/Cargo.toml`, `Cargo.lock`, `build.rs`, `native/oqs_shim.c`,
`native/oqs_shim.h`, `src/lib.rs`, `src/pqc/{mod,ffi,kem,signature,secret,error,tests}.rs`,
`tests/native_contract.c`; `scripts/{check_pqc_native,check_pqc_source_pin,validate_phase1}.py`;
this review and `PHASE_1_VALIDATION.txt`.
Modified: `Makefile`, root/component/scripts READMEs, `docs/ENVIRONMENT.md`,
`CRYPTO_PROTOCOL.md`, `DECISIONS.md`, `KNOWN_LIMITATIONS.md`, `TEST_MATRIX.md`.
Previous readiness evidence/probe, authoritative specification/plan, AGENTS.md,
Phase 0 evidence, user history and third-party source trees are preserved.
No Git commit or metadata repair was attempted.

## 3. Design and build contracts

D-17 selects the approved small FFI route; D-18/D-19 define lifetime/secret ownership.
The build verifies the exact tag/HEAD, rejects dirty/untracked/ignored source and
concealing Git index flags, then builds locally under Cargo OUT_DIR. The source can
be explicitly relocated with CADP_LIBOQS_SOURCE but must pass the identical checks.
No registry dependencies, automatic download, system liboqs selection or fallback.
Cargo.lock pins only this project crate; Git metadata is required for source checking.

Approved flags: Release, static library, build-only-library, distribution CPU
selection, OpenSSL OFF, minimal `KEM_ml_kem_768;SIG_ml_dsa_65`. The current build
also explicitly enforces `OQS_USE_SHA3_AVX512VL=OFF` under D-20; the original
baseline did not exclude this path. Generated and installed headers are checked
and the shim rejects the macro. See the historical finding and correction below.
Installation layout adds `CMAKE_INSTALL_LIBDIR=lib` and `CMAKE_C_COMPILER=cc`. Shim compilation:
`-std=c11 -O2 -fPIC -Wall -Wextra -Wpedantic -Werror`.
Linkage: static shim/liboqs, system pthread/runtime dynamically linked as needed.
Generated pthread support is required at compile time; other host/target builds
are unverified. Exact expanded commands and archive SHA appear in build evidence.

Native metadata verifies version 0.16.0, exactly one enabled KEM/signature, names
ML-KEM-768/ML-DSA-65, FIPS203/FIPS204 algorithm metadata, function pointers and sizes:
KEM 1184/2400/1088/32; signature 1952/4032/3309. No alternate algorithm selection API.

## 4. Unsafe and FFI inventory

One private `unsafe extern "C"` block and **10 unsafe call blocks**, all in
`trusted-client/src/pqc/ffi.rs`. The crate denies unsafe elsewhere; only that module
has an explicit exception. No unsafe Rust in build code, wrappers, secret owner or tests.
Each call block documents lifetimes, pointer validity, lengths and aliasing.

| Shim entry | Contract |
|---|---|
| cadp_initialize | OnceLock serializes global initialization; writes seven size_t metadata entries into exclusive Rust storage |
| cadp_thread_stop | Same-thread per-call cleanup, after C handles are freed |
| cadp_cleanse | Exclusive live mutable slice, byte count, reviewed OQS_MEM_cleanse |
| cadp_secret_equal | Equal-length immutable slices, reviewed OQS_MEM_secure_bcmp |
| cadp_kem_keypair | Exclusive fixed-size public/secret outputs |
| cadp_kem_encaps | Borrowed checked public key; exclusive ciphertext/secret outputs |
| cadp_kem_decaps | Borrowed checked secret/ciphertext; exclusive secret output |
| cadp_sig_keypair | Exclusive fixed-size public/secret outputs |
| cadp_sig_sign | Borrowed secret/message; verified maximum output capacity; checked returned length; ordinary or NULL,0 context API only |
| cadp_sig_verify | Borrowed key/message/signature; exact ML-DSA sizes; ordinary or NULL,0 context API only |

Rust reproduces no OQS struct layouts and exposes no raw pointer/native handle.
C creates/frees a local OQS handle per call; all allocation, metadata and operation
failure returns balance ownership. Rust's slice borrows prevent overlapping mutable
outputs and keep inputs live throughout synchronous native calls. Empty messages
use a readable static address without copying. No native callbacks into Rust.

## 5. Lifecycle and concurrency

OnceLock caches one initialization result for process lifetime. Initialization failure
is terminal for that process. The barrier prevents native CPU-detection initialization
races. Every operation creates handles in C, frees them before return, then Rust's
cleanup guard calls OQS_thread_stop on that same thread. OQS_destroy is intentionally
not exposed or called: it could race active callers, and is a no-op in the approved
OpenSSL-free configuration. The OS reclaims remaining process state at exit.

The pinned SHA3 backend uses pthread_once when OQS_USE_PTHREADS is enabled; RNG
callbacks remain the default OS implementation and are never changed by the API.
The original inference from pthread_once alone was insufficient: Astra H-01 found
AVX-512 callback-table writes with unsynchronized readers. D-20 now excludes that
branch; no upstream source was patched and no AVX-512 hardware test is claimed.
Concurrent operations have separate handles and buffers. Smoke tests exercise four
threads and fresh-process concurrent initialization. This is not a ThreadSanitizer
proof or a claim about untested liboqs configurations/architectures.

## 6. Secret ownership and errors

One private SecretBuffer owns a Vec allocation, reserved fallibly at fixed sizes,
zero-initialized before use, and never resized after native writes. Generated
KEM/signing keys are separate types. There is no Clone, public import/export,
Deref/AsRef, Display or implicit text conversion. Debug is a literal redacted label.
Shared secrets compare through the reviewed library comparison and never expose bytes.
Drop cleanses the live allocation before Vec deallocation, including failed native
encapsulation outputs. Tests observe zeroed memory while still allocated, not after free.

Lengths reject before FFI; C repeats them before constructing a handle. Allocation,
backend metadata and operation failures have typed errors without secret diagnostics.
Correct-size invalid signatures produce Ok(false); malformed signatures produce Err.
Future callers must authorize only on Ok(true). Actual signature length is checked
against the FIPS ML-DSA-65 contract before truncation/indexing. Messages are borrowed,
so wrapper allocation is independent of attacker-selected message size.

No production unwrap/expect/panic/TODO. The one expect is a test-thread join assertion;
compile-fail examples use unwrap solely as nonexecuted type-check fixtures. Backend
fatal RNG/internal allocation failures may exit instead of return; this is an explicit
upstream limitation, not a promise of universal recoverable errors or cleanup.

## 7. Cryptographic semantics

ML-KEM implicit rejection is handled correctly: wrong-key and modified ciphertext
cases assert a different secret, not an error. Authorization requires future Phase 3
CEK-wrap authentication. Truncated ciphertexts never reach native operations.
Noncanonical public-key encoding is rejected by liboqs and failure output is cleansed.

ML-DSA uses ordinary pure signing/verification with empty context. The pinned
backend selects MLD_PREHASH_NONE and the pure empty-context domain prefix. Ordinary
signatures verify through explicit NULL,0 context API, and signatures from that API
verify ordinarily. Randomized signatures are not compared byte-for-byte. There is
no prehash, external-mu, legacy or non-empty-context API. No protocol discrepancy.
FIPS-standard algorithm use does not mean this module is FIPS validated.

## 8. Test coverage and evidence

Mandatory PQC-KEM-001..004 and PQC-SIG-001..005 have actual runtime assertions.
Additional IDs cover backend identity/sizes, key and output length boundaries,
empty/1 MiB messages, empty-context interoperability, secret Debug/drop, allocation
capacity failure, lifecycle, concurrent operations and noncanonical public-key failure.
Three compile-fail checks prevent raw secret access, cloning and role confusion.
Four native contract groups exercise null/length guards, allocation/operation faults,
metadata faults and balanced native handles. Source tests reject missing, wrong,
dirty, untracked, ignored and concealed-index source in a disposable clone.

Final per-command counts and exact outputs are in PHASE_1_VALIDATION.txt and the
[TEST_MATRIX.md](TEST_MATRIX.md). Later-phase test statuses remain PLANNED.
Documentation validation reuses applicable historical checks and adds Phase 1 scope;
it does not reinterpret scaffold-only Phase 0 tests or rewrite their evidence.

| Final check | Actual result |
|---|---|
| Rust debug unit tests / compile-fail doctests | 23 / 3 passed; 0 failed; 0 ignored |
| Rust release unit tests / compile-fail doctests | 23 / 3 passed; 0 failed; 0 ignored |
| Fresh-process concurrent initialization | 1 passed; 0 failed; 22 unrelated cases filtered |
| Native shim contracts, Clang ASan/UBSan | 4 groups passed; 0 failed; leak scanning unavailable |
| Source-pin guard | 6 expected rejection cases passed; each Cargo rejection exited 101; suite exited 0 |
| Documentation and phase scope | 6 passed; 0 failed; 1 skipped (Mermaid renderer unavailable) |
| rustfmt / strict Clippy / git diff --check | All exited 0 |

No Phase 1 test is failed or ignored. LeakSanitizer and full-backend instrumentation
are verification limitations, not successful checks. No FIPS validation is claimed.

## 9. Separate security review findings

Review pass re-read the complete build/native/FFI/secret/API sources after initial
implementation and tests, against the user's checklist and applicable AGENTS categories.
Identity/parsing/state-machine/filesystem/ledger/watermark requirements remain outside
this primitive phase; no corresponding system invariant is claimed tested.

| Finding | Severity / disposition | Evidence / follow-up |
|---|---|---|
| P1-R01 | MEDIUM, FIXED: Git status can conceal edits using index flags | Build rejects assume-unchanged/skip-worktree; concealed tracked-edit regression added to PQC-BUILD-001 |
| P1-R02 | MEDIUM, FIXED: concurrency depended on an implicit pthread build assumption | Shim compile guard requires OQS_USE_PTHREADS and OpenSSL OFF; current generated header verified; concurrent startup test |
| P1-R03 | LOW, FIXED: defensive null metadata checks were incomplete in first draft | Version/name/algorithm metadata checks added; PQC-FFI-004 null metadata injection rejects safely |
| P1-R04 | MEDIUM, retained limitation | Fatal upstream RNG/SHA3 allocation paths bypass Drop; L-19, Phase 5 hardening still required |
| P1-R05 | HIGH, retained deployment limitation | liboqs itself discourages production/sensitive-data reliance; L-17. Academic prototype only |
| P1-R06 | LOW, retained verification limitation | LeakSanitizer unusable under this surface's ptrace restriction; L-18. ASan/UBSan shim checks and native handle counts still execute |
| P1-R07 | INFORMATIONAL | Static linking requires rebuild/redeployment for updates; ongoing supported-release tracking and external review remain necessary |

No unresolved CRITICAL finding was identified. This limited review does not certify
memory safety, constant-time behavior or real-document deployment suitability.

## 10. Bugs found and fixed

Initial optimized shim lacked -fPIC, causing Fedora PIE relocation/link failure.
Added -fPIC; actual linkage/tests succeeded. Strict Clippy found items after a
cfg(test) module; reordered them and retained strict warnings. The source-pin harness
assumed Git parents existed; supplied checkout was shallow. It now creates an empty
commit only in its temporary clone. GCC sanitizer linking and LeakSanitizer failures
were environment limitations; Clang ASan/UBSan is used with leak scanning explicitly
unavailable. Historical failed outputs are retained in phase validation evidence.

## 11. Known limitations and scope

Only primitives exist. No protected storage or safe document workflow, no root/kernel
protection, no heap-copy/register/swap/core guarantees, no full-backend ASan/Miri/fuzz/
constant-time or FIPS validation. Large borrowed messages incur linear processing;
future protocol callers must enforce their bounded schemas. Only Fedora x86_64 native
Linux was tested. Source requires trusted Git metadata and build tools; malicious
local tooling/artifacts are outside the integrity check's claim. No external review
is claimed; the user's Astra review will follow. See KNOWN_LIMITATIONS.md.

## 12. Manual verification and demonstration

From the repository root with clean pinned .deps source:

```sh
export CARGO_HOME="$PWD/.deps/cargo-home"
make build
make check
cargo test --offline --locked --release --manifest-path trusted-client/Cargo.toml
cargo test --offline --locked --manifest-path trusted-client/Cargo.toml pqc_conc_001
rg -n 'unsafe|unwrap\(|expect\(|panic!|TODO' trusted-client/src trusted-client/build.rs
git diff --check
git -C .deps/liboqs-0.16.0 status --short
```

For a short public demonstration, run the mandatory cases:

```sh
cargo test --offline --locked --manifest-path trusted-client/Cargo.toml pqc_kem_
cargo test --offline --locked --manifest-path trusted-client/Cargo.toml pqc_sig_
cargo test --offline --locked --manifest-path trusted-client/Cargo.toml pqc_ctx_001
```

Show successful case names and checked public backend metadata from the source/tests;
never print secret buffers. Wrong-key/tampered KEM tests pass because secrets differ;
negative signature tests pass because verification rejects. No generated key artifact
or recipient identity is retained. Native/source-test temporary trees are removed.
Optional on an unrestricted suitable host: `CADP_TEST_LEAKS=1 make native-check`;
this command is not claimed successful in this session.

## 13. Proposed next phase

Phase 2 — Private-Key Storage and Offline PQ PKI, only after explicit approval and
review. Its storage/export/import boundary, Argon2id/AES-GCM, CA/certificates and
revocation are entirely unimplemented. Stop after the Phase 1 report.

## 14. Independent Astra audit and authorized corrective review — 2026-10-07

The independent GPT-6 Astra audit returned **PHASE 1 SECURITY AUDIT: APPROVED WITH
REQUIRED FIXES**. H-01 (HIGH) identified shared SHA3/SHA3x4 callback-table assignments
inside first-use dispatch while public hash entry points read the tables before
the once guard. OQS_init does not initialize those tables. L-01 (LOW) identified
missing persistent end-to-end partial-output cleanup coverage. M-01 retained the
existing fatal RNG/internal-allocation limitation. Earlier P1-R02 pthread guards and
passing smoke tests did not establish absence of H-01. Original validation output
and finding history above are preserved; the old status is not current gate approval.

### Corrections and evidence

H-01: build.rs explicitly passes SHA3 AVX512VL OFF, validates the actual generated
header before compilation, and validates the installed header before compiling the
shim. Only the pinned template's single explicit undef marker is accepted. A macro
definition of zero is also forbidden because upstream uses defined-macro checks.
The shim has an independent compile rejection. PQC-BUILD-002 executes the build
checker's positive/seven-negative header cases and two forbidden shim compilations.
Validation from fresh target state confirms pthreads ON, OpenSSL OFF, AVX512VL OFF
and no avx512vl archive symbols. A separate ON cache was overridden to OFF and the
backend rebuilt by the real build script. H-01 is FIXED / TESTED for this correction.

L-01: PQC-SECRET-003 links the production Rust rlib with isolated C test wrappers.
The wrappers intercept four primitive operations, write a nonzero public pattern
into half of the zeroed secret output, and return failure. Setup keys/ciphertexts
are synthetic; no real key operation or secret is required. The actual safe API
returns the expected typed error after its local SecretBuffer drops. A wrapper
observes OQS_MEM_cleanse before/after the real library cleanse, then observes the
same allocation at free. State/count assertions require exactly one write, cleanse
and free in order; no memory is inspected after free. The fixture never calls
cleanse manually. All four child-process modes passed. A disposable Rust source
copy with cleansing omitted failed all four modes, demonstrating detection rather
than a vacuous pass. L-01 is FIXED / TESTED; no production cleanup code changed.

### Corrective self-review

The explicit CMake option overwrites an ON cache; actual headers and compile guards
fail closed. Cargo JSON now identifies current native/rlib artifacts in the check
script, avoiding stale mtime selection, and the script honors an isolated target.
The exact source pin, minimal algorithms, static linkage and no-download policy are
unchanged. All seven sizes, pure ML-DSA empty-context cross-verification and fresh
concurrent initialization/KEM/signature activity passed again. Protocol file contents
are unchanged by this task. No new unsafe Rust, production panic/unwrap/expect,
public pointer/secret getter or production fault hook exists. The unsafe inventory
remains one extern block and ten call blocks in ffi.rs; the support Rust driver
denies unsafe and contains test assertions only. Test wrapping is isolated to
temporary executables and synchronous child processes.

M-01/L-19 remains MEDIUM and is not fixed: no RNG fallback, signal/abort recovery or
liboqs modification was introduced. No new unresolved CRITICAL/HIGH corrective
finding was identified in this self-review. This is not the pending Astra re-audit.

### Executed checks and changed files

Fresh debug/release: each 23 unit and 3 compile-fail tests passed, 0 failed/ignored.
Fresh-process PQC-CONC-001: 1 passed, 22 filtered. Native contracts: 4 groups passed.
PQC-BUILD-002: 1 checker test (8 header cases) plus 2 expected compile rejections.
PQC-SECRET-003: 4 paths passed; 4 broken-copy negative controls rejected. Source pin:
6 expected rejection cases passed. Current documentation checks: 6 passed, 1 Mermaid
skip. Formatting, strict Clippy, Make entry points and whitespace checks executed;
exact command output is in PHASE_1_CORRECTIONS_VALIDATION.txt.

Changed for this corrective task: build.rs, native/oqs_shim.c,
scripts/check_pqc_native.py, scripts/validate_phase1.py, component/scripts READMEs,
ENVIRONMENT.md, DECISIONS.md, KNOWN_LIMITATIONS.md, TEST_MATRIX.md and this review.
Created: tests/support/partial_output.c, tests/support/partial_output.rs and
PHASE_1_CORRECTIONS_VALIDATION.txt. Existing native_contract.c, Rust production
modules, Cargo manifests/lockfile, specification/plan/protocol, original validation,
Phase 0 evidence and third-party sources are unchanged by this task.

Manual verification/demonstration: use the clean-target commands in ENVIRONMENT.md,
show the disabled generated macro, run the fresh concurrency filter and
`python3 scripts/check_pqc_native.py`; show four cleanup PASS lines and expected
forbidden-config rejections without printing any secret values. H-01/L-01 are ready
for Astra re-audit; do not begin Phase 2 without review and explicit approval.
