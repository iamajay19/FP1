# Decision Log

Status: Phase 0 contracts preserved; Phase 1 primitive integration authorized and
implemented. Evidence below covers primitives only, not later protocol security.

| ID | Decision | Rationale / consequence |
|---|---|---|
| D-01 | Keep authoritative specification and phase order unchanged | Supporting documents were empty; fill them without rewriting requirements |
| D-02 | Dependency-free Python/Make Phase 0 scaffold | Actual host lacks Rust/Go/liboqs; no speculative build manifests or downloads |
| D-03 | Rust trusted core, Python administration/forensics, Go Fabric chaincode | Follow recommended stack; no JS cryptography, no GUI before Phase 11 |
| D-04 | Independent ML-KEM-768 and ML-DSA-65 keys via liboqs | Fixed application algorithms; never substitute classical primitives |
| D-05 | RFC 8949 core deterministic CBOR, domain-labelled signed arrays | Removes ambiguity in the specification's term canonical CBOR |
| D-06 | Length-prefixed CADP with one content object and complete signed entries | Simple framing; no ZIP paths, compression or unsigned recipient envelopes |
| D-07 | Explicit content AAD, specification HKDF with fixed-size recipient ID | Avoid ciphertext-hash AAD cycle and ambiguous concatenation |
| D-08 | Signed recipient entries bind certificate hash and both key IDs | Prevent identity/key substitution between decryption and receipt |
| D-09 | Separate pinned publisher signing-role registry | Recipient certificate never implies publishing/administrative authority |
| D-10 | Both-org endorsement and valid matching commit confirmation | Submission/endorsement alone cannot authorize output |
| D-11 | Fail closed on unavailable/stale revocation state and unknown commit | Availability loss is preferable to an unaudited release |
| D-12 | Strict exact-hash requirement for VERIFIED_ATTRIBUTION in version 1 | Public watermark data alone cannot authenticate transformed instances |
| D-13 | No open-source license chosen on owner's behalf | LICENSE records unresolved owner decision |
| D-14 | No Git metadata repair in exposed read-only .git | Filesystem work is possible, Git history/status cannot be verified here |
| D-15 | Phase 5 mock ledger/watermark never authorize real output | Preserve phase order without weakening the final release invariant |
| D-16 | Document recipient bypass as a trust assumption, not a solved control | Distributed KEM keys cannot force use of a software-only workflow |
| D-17 | Small project-owned Rust FFI boundary to exact liboqs 0.16.0; C owns native handles and reads their metadata using matching headers | Q-01 route approved and Phase 1 wrappers implemented. No copied Rust OQS layouts, third-party patch, oqs crate, unsupported release or alternate algorithm |
| D-18 | OnceLock process-lifetime backend; per-call C handle ownership and same-thread OQS_thread_stop; no global OQS_destroy API | Prevent initialization races and destruction while concurrent callers are active. OpenSSL-free destroy has no work; pthread-enabled build required. Initialization failure is cached and fail-closed |
| D-19 | Separate generated KEM/signature secret types over one private heap SecretBuffer, with reviewed liboqs cleanse on Drop | No Clone, raw export/import, string conversion or secret Debug; fixed-size fallible allocation. Phase 2 must design protected storage without adding an unrestricted public getter |
| D-20 | Explicit OQS_USE_SHA3_AVX512VL=OFF; check generated and installed configuration and reject the macro in the C shim | Authorized H-01 correction excludes the callback-table dispatch race without patching liboqs or changing primitives. Pthreads, OpenSSL OFF and static linkage remain required |
| D-21 | Isolated test-only linker wrapping exercises production Rust partial-output error/Drop/cleanse/free paths | Authorized L-01 correction, PQC-SECRET-003. Synthetic public patterns only; no production failure hooks, new unsafe Rust or secret export |

## Open decisions and resolution gates

| ID | Issue / alternatives to evaluate | Owner and deadline |
|---|---|---|
| Q-01 | RESOLVED: approved Phase 1 wrappers build offline and statically link exact liboqs tag 0.16.0 at 5a1a854b0dc9f2141bdc771c555ee60c37950183. Both algorithms, sizes and mandatory primitive tests executed; see PHASE_1_REVIEW.md and PHASE_1_VALIDATION.txt | Keep support/update tracking and external review; Phase 2 still requires explicit approval |
| Q-02 | Key-bundle KDF benchmark, CA/publisher key lifecycle, provisioning possession proof and recovery procedure | PKI implementer, Phase 2 before deployment |
| Q-03 | Trusted local time and persistent anti-rollback CRL epochs; permitted validity skew and import cadence | PKI/ledger operators, Phases 2/6; unavailable freshness rejects |
| Q-04 | Deterministic liboqs verification in Go/external chaincode, Fabric mapping/ACLs, authenticated two-peer commit evidence and rollback checkpoints | Ledger implementer, Phase 6 before real release integration |
| Q-05 | Watermark ECC/repetition profile, PDF reference signature/storage, carrier alignment/capacity and thresholds | Watermark implementer, Phase 7 (text) and Phase 9 (PDF) |
| Q-06 | In-process PDF library suitability, secret copies, memory limits, output path/crash semantics | Core implementer, Phases 5/8/9; reject unsupported environment |
| Q-07 | Offline pinned dependency bundle, licenses and clean-host compatibility (Python 3.14 observed) | Deployment implementer, Phase 13; versions pinned as each component enters scope |
| Q-08 | Project licensing and ownership notices | Project owner, before external distribution |
| Q-09 | Read-access blocker RESOLVED: git status/log work, current HEAD 200bc4ab96a8d8fdf3bc5ffa846a668de7f375c8; user-owned Git metadata and .deps ignore rule preserved. Git writes remain outside execution permission scope and untested | Workspace owner before version-control writes |
| Q-10 | Independent organizational audit beyond this agent's separate review pass | Project owner before any real-document deployment |

Phase 0 resolves ambiguities by specifying initial contracts, not by claiming these
implementation-dependent questions are experimentally settled. Q-01's environment
blocker is now resolved by the final local-source evaluation below. The user must
has now explicitly approved Phase 1 implementation. Phase 2 is not approved.

## Phase 1 environment preparation update — 2026-10-06

The user approved environment preparation only, not Phase 1 implementation.
Fresh inspection and upstream API/build-script research are recorded in
[ENVIRONMENT.md](ENVIRONMENT.md) and
[PHASE_1_ENVIRONMENT_VALIDATION.txt](PHASE_1_ENVIRONMENT_VALIDATION.txt).
No dependencies installed or runtime code created. The published binding pair targets
liboqs 0.13.x, while upstream liboqs lists 0.16.0; compatibility with the newer release
must not be assumed. No exact working project dependency set is approved yet.
No production keys or project secrets were processed. Phase 1 tests remain PLANNED.
Go/Docker and other later-phase tooling do not block Phase 1 primitive preparation.

## Supported-liboqs reinspection — 2026-10-06

Supersedes the earlier missing-tool findings. User-installed Rust/Cargo 1.98.1,
CMake 4.3.0, GCC/C++ 16.2.1 and Clang/libclang 22.1.8 are visible. A temporary
compiler-only Rust program ran; no liboqs linkage occurred. Upstream security policy
supports only liboqs 0.16.0, so the earlier 0.13.0 compatibility candidate is not an
approved dependency. Evaluate exact current main, local-library configuration, then
minimal audited FFI in that order. Main could not be pinned because shell GitHub
access fails DNS; differently cached web files are not a coherent source checkout.
No unsupported release, production wrapper, project key or Phase 1 test was used.
Q-01 remains BLOCKED. See ENVIRONMENT.md and PHASE_1_ENVIRONMENT_RECHECK.txt.

## Final local-source Q-01 resolution — 2026-10-06

Supersedes the two historical preparation attempts above. Actual source provenance,
binding inspection, supported-release build and Rust static-linkage proof are recorded
in [ENVIRONMENT.md](ENVIRONMENT.md) and
[PHASE_1_Q01_READINESS.txt](PHASE_1_Q01_READINESS.txt).
liboqs-rust commit `c6fd1a68179255b2461800e7da61231d7d3c61da` has
`oqs 0.11.0`, `oqs-sys 0.11.0+liboqs-0.16.0-rc1` and submodule
`2966c35b3b7691eda295d465b500673377250810` (post-release 0.16.0).
Its external-library pkg-config range rejects the final release on this host;
the offline vendored-crate check failed on unavailable `cstr_core`, not on a
demonstrated API mismatch. Neither crate route is claimed built/tested.

D-17 selects exact release 0.16.0, Release/static/minimal algorithms, CPU dispatch
enabled and OpenSSL disabled. The successful readiness probe has no Cargo package
dependencies and creates no keys. A small retained script reproduces and cleans up
the probe offline. Pure ML-DSA with empty context agrees with the existing protocol;
no protocol or requirement changes were needed. The shared C API is a future Go/cgo
candidate, not proof of Phase 6 readiness. Maintain support tracking and review
ownership, errors, input bounds and zeroization during Phase 1; static binaries must
be rebuilt for library fixes. Q-01 is RESOLVED only for preparation. No production
PQC wrapper exists and all nine Phase 1 runtime test IDs remain PLANNED.

## Authorized Phase 1 implementation — 2026-10-06

Supersedes earlier statements that Phase 1 is unapproved/unimplemented. User
authorized the primitive phase only and confirmed D-17 as authoritative. The
dependency-free Rust crate builds the clean pinned release itself; it rejects
missing/wrong/dirty/untracked/ignored source and concealed Git index flags, never
downloads or probes a system library. `CMAKE_INSTALL_LIBDIR=lib` stabilizes local
layout; shim `-fPIC` permits Fedora PIE linking. Baseline liboqs options are unchanged.

D-18/D-19 record lifecycle and secret ownership. Safe wrappers expose only the six
required primitive operations and public checked metadata. KEM implicit rejection
is tested as secret mismatch, never mislabeled authorization failure. Pure ML-DSA
ordinary/empty-context interoperability passed; protocol unchanged except status
clarification. Exact evidence, separate review, limitations and manual commands are
in [PHASE_1_REVIEW.md](PHASE_1_REVIEW.md). This agent's separate self-review is not
the user's forthcoming GPT-6 Astra review or an independent organizational audit.

## Independent Astra audit and authorized corrections — 2026-10-07

The independent model audit returned **PHASE 1 SECURITY AUDIT: APPROVED WITH
REQUIRED FIXES**. It found H-01 (HIGH): the enabled AVX-512 SHA3 first-use dispatch
can write shared callback tables while public hash functions read them before
pthread_once synchronization. OQS_init initializes CPU detection, not that dispatch.
It also found L-01 (LOW): existing native failure tests and Rust Drop tests did not
connect partial native writes to Rust's error-path cleanup. M-01 remains the existing
L-19 fatal RNG/internal-allocation limitation. This history is not superseded by a
claim that the original configuration was safe.

The user authorized these two corrections only. D-20 adds one build option to the
original baseline; D-17's exact source, algorithms and integration architecture are
unchanged. Generated headers prove the option is disabled; PQC-BUILD-002 rejects
forbidden definitions, including zero, because upstream checks macro definition.
A separately configured ON cache was overwritten to OFF and rebuilt by build.rs.
D-21's four persistent cases prove cleanup through the unmodified Rust API, including
the subsequent free. A disposable negative control omitted cleansing and all four
cases rejected it. Evidence: [PHASE_1_CORRECTIONS_VALIDATION.txt](PHASE_1_CORRECTIONS_VALIDATION.txt).
H-01/L-01 are FIXED and TESTED for corrective work; Astra re-audit remains pending.
M-01 is not fixed. Protocol bytes/semantics, phase gates and Phase 0 evidence are
unchanged. No Phase 2 authorization or organizational audit is implied.
