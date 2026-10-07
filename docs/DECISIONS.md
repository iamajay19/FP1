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
| D-22 | Exact zeroize 1.9.1; defaults-off AES-GCM aes+zeroize and Argon2 zeroize, caller-owned workspace; SHA-2/Blake2 zeroize feature unification | Refreshed vendor graph and isolated API probe passed offline. Argon2's feature does not cleanse its caller workspace or automatically enable Blake2 cleanup. No physical-erasure guarantee |
| D-23 | Exact minicbor 2.3.0, defaults off/no features, as low-level codec; bounded exact-schema decode then deterministic re-encode and exact-byte comparison | Preserve RFC 8949 core deterministic CBOR. Library alone does not enforce the project profile. Source/integrity/build gate verified by D-26; no production codec yet |
| D-24 | One-shot CA challenge binds the complete proposed certificate; recipient signs both challenge and HKDF/HMAC KEM confirmation | Exact labels/transcript and authenticated provisioning assumptions in KEY_MANAGEMENT.md. ML-KEM return status is not PoP. Exact hmac/hkdf 0.13.0 availability verified by D-26; no PoP runtime yet |
| D-25 | Zero-skew certificate/CRL checks relative to configured local time; persistent cumulative CRL sequence with explicit bootstrap | Fail closed on missing/stale state; reject lower/conflicting sequence and cumulative-entry removal. Conditional local replay rejection is not trusted time or whole-host anti-rollback |
| D-26 | Close Phase 2 dependency availability gate for the exact ten pinned candidates and approved features | Refreshed 34-package closure / 31 Linux registry crates passes integrity, independent empty-home locked offline build and SHA-256 HMAC/HKDF public-vector probes. No production implementation, full third-party audit or Phase 2 runtime approval implied |

## Open decisions and resolution gates

| ID | Issue / alternatives to evaluate | Owner and deadline |
|---|---|---|
| Q-01 | RESOLVED: approved Phase 1 wrappers build offline and statically link exact liboqs tag 0.16.0 at 5a1a854b0dc9f2141bdc771c555ee60c37950183. Both algorithms, sizes and mandatory primitive tests executed; see PHASE_1_REVIEW.md and PHASE_1_VALIDATION.txt | Keep support/update tracking and external review; Phase 2 still requires explicit approval |
| Q-02 | PoP transcript DESIGN RESOLVED by D-24; implementation untested. Key-bundle KDF benchmark, CA/publisher lifecycle and recovery procedure remain OPEN | PKI implementer, Phase 2 before deployment |
| Q-03 | Phase 2 conditional time/sequence semantics DESIGN RESOLVED by D-25; trusted local time, host anti-rollback and deployment import cadence remain OPEN | PKI/ledger operators, Phases 2/6; unavailable freshness rejects |
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

## Phase 2 authorization and dependency gate — 2026-10-07

Supersedes the historical pending Astra re-audit/Phase 2 approval statements above.
[PHASE_1_ASTRA_REAUDIT.txt](PHASE_1_ASTRA_REAUDIT.txt) records independent APPROVED
status. The user explicitly authorized Phase 2, not Phase 3. D-17 through D-21 and
the approved PQ API remain authoritative and unchanged; M-01 remains open.

Q-02 remains OPEN: no bundle/KDF profile, benchmark, provisioning transcript or CA
recovery procedure was implemented or finalized. Implementation is BLOCKED by absent
offline AES-GCM and Argon2 crates, confirmed by four disposable resolution failures.
The explicit user stop condition applies; no alternative cipher, custom primitive,
download or new unsafe boundary is approved as a workaround. Q-07's Phase 2 dependency
subset needs reviewed exact pins, integrity/license metadata and complete offline
sources before resumption; the full deployment gate remains Phase 13.

The availability probes' wildcard requirements select no version and are not project
pins. Exact dependency selection awaits supplied source and maintenance review.
All Phase 2 runtime IDs remain PLANNED. Evidence and recovery are in
[PHASE_2_REVIEW.md](PHASE_2_REVIEW.md) and [PHASE_2_VALIDATION.txt](PHASE_2_VALIDATION.txt).

## Phase 2 offline dependency evaluation — 2026-10-07

Supersedes source-absence statements in the earlier gate report, not historical
evidence or protocol decisions. Latest user authorization is dependency verification
ONLY. Candidates =aes-gcm 0.11.1 / =argon2 0.6.0 / =sha2 0.11.0 / =getrandom 0.4.3 /
=rpassword 7.5.4 match the owner's intact bundle and compile offline. These are
technically suitable API candidates, not unconditional production adoption.

Q-07's Phase 2 availability subset is partly verified but remains BLOCKED on missing
reviewed zeroize source/feature closure. Prefer defaults off, aes-gcm aes+zeroize and
argon2 zeroize with caller-owned bounded drop-cleansed workspace; no PHC/Rayon/derive
is needed. Argon2's convenience Blocks Drop only frees memory, even with zeroize.
SHA-256/getrandom/rpassword need no extra features. Enforce getrandom's syscall-only
linux_getrandom cfg on Fedora (default permits OS-device fallback); never introduce
fallback randomness. Host supports this graph, but eventual component MSRV must be
at least 1.89. Production Cargo files and Phase 1 unsafe boundary are unchanged.

Q-02 remains OPEN; no PoP transcript, KDF benchmark/profile or recovery design was
selected. No deterministic-CBOR dependency approved; choose/review the authorized
bounded subset or a separately supplied library before encoding implementation.
No HMAC/HKDF primitive was inferred from availability. No fresh advisory/support
verification was possible under the no-network rule; local integrity is not upstream
authenticity or exact-release audit certification. See
[PHASE_2_REVIEW.md](PHASE_2_REVIEW.md). All 19 Phase 2 runtime IDs remain PLANNED.


## Phase 2 final pre-implementation design gates — 2026-10-07

Latest user scope is gate verification/design/documentation ONLY. It overrides the
previous broad implementation authorization for this task. No Phase 2 production
implementation or Phase 3 work was performed. Earlier absence/failure reports above
remain historical evidence, not descriptions of the refreshed bundle.

D-22 closes the missing-cleanup-source blocker: exact zeroize 1.9.1 is present;
31-package cross-platform closure, Cargo/file/owner checksums, empty-CARGO_HOME
locked build and cleanup-enabled API probe passed. Defaults are off for all direct
candidates. AES-GCM forwards cleanup to AES/GHASH/POLYVAL. Argon2 requires a
project-owned bounded drop-cleansed workspace; its own feature is insufficient.
The tested SHA-2 and Blake2 zeroize additions reuse already supplied crates. These
feature changes are planned manifest requirements, not production Cargo edits.

D-23 selects a library-assisted strict CBOR route rather than a new wire format.
D-24 resolves the exact PoP design, not all of Q-02; see KEY_MANAGEMENT.md for the
full normative challenge, derived key, MAC, signed response and consumption rules.
D-25 separates implementable local checks from unresolved secure-time/host-rollback
assumptions. No existing certificate/bundle/CRL wire bytes or Phase 1 algorithm,
source pin, static linkage or unsafe boundary changed.

Remaining pre-implementation blocker: offline sources/checksums and exact pinned
build for minicbor 2.3.0, hmac 0.13.0 and hkdf 0.13.0. All three disposable offline
resolution probes actually failed (exit 101, no matching package); no download or
vendoring was attempted. Public upstream documentation/manifests were inspected
read-only for candidate maintenance/features, not treated as a local verified source
bundle or full dependency security audit. With the proposed minimal features these
add only three new package sources; re-resolve and inspect the refreshed graph before
implementation, rather than treating this prediction as build evidence.

Q-07's Phase 2 subset remains BLOCKED on those three additions; full deployment
closure remains Phase 13. Existing approved APIs remain authoritative. All 19 Phase 2
runtime IDs remain PLANNED; no PKI/storage/PoP benchmark or runtime success claimed.
Evidence and exact commands: PHASE_2_REVIEW.md / PHASE_2_VALIDATION.txt.


## Phase 2 final dependency gate closure — 2026-10-07

D-26 supersedes the preceding three-source absence blocker; historical negative
checks remain intact. Latest user authorization is FINAL DEPENDENCY VERIFICATION
ONLY. All ten exact pins and requested features match the refreshed owner metadata
and independently resolved graph. Integrity covers exact package/file sets, normalized
source versions, Cargo package/file checksums, owner hashes and symlink rejection.
A new empty-CARGO_HOME / fresh-target build, check and public API probe passed entirely
offline, with the supplied lock unchanged and local vendor source only.

There are 34 registry packages across platforms and 31 resolved on Fedora Linux,
excluding the disposable root (10 direct / 24 full or 21 Linux transitive). New delta
is exactly minicbor 2.3.0, hmac 0.13.0 and hkdf 0.13.0; no new native/build/proc-macro
dependency. Minicbor minimal codec types compile; HMAC-SHA-256/HKDF-SHA-256 public
reference-vector probes pass. These checks establish availability/API compatibility,
not a CBOR schema, PoP implementation, complete upstream audit or legal approval.

The D-23 validation strategy and D-24/D-25 Q-02/Q-03 transcript/time-policy documents
remain byte-identical to this task's start. Syscall-only Linux getrandom enforcement,
cleanup ownership, KDF benchmark and local-time/host-rollback caveats remain required
for future implementation. Q-07's Phase 2 source subset is READY; full deployment
closure is still Phase 13. No Phase 2 production or Phase 3 code was added, and all
19 existing Phase 2 runtime IDs remain PLANNED. This report stops at the user gate.


## Authorized Phase 2A implementation — 2026-10-08

D-27: implement D-23 with private borrowed typed Reader/Decode/Encode, a bounded
structural preflight, exact-field map helper and deterministic encoded-key sorting.
Only validated() returns an accepted decoded object, after full consumption and
whole-original-byte re-encoding equality. Embedded canonical byte objects share
item budget and logical depth; there is no generic Value tree or unknown-field skip.
The five supported major types match existing Phase 2 schemas; other RFC CBOR forms
are forbidden by this approved subset. No certificate/bundle/PoP/CRL schema exists yet.

D-28: enforce Cargo-wide Linux syscall-only getrandom cfg and matching rustdoc cfg,
with build predicate and crate compile guards. Keep fixed public RandomId B[32] and
separate KemKeyId/SignatureKeyId B[32] types. Key IDs use the unchanged exact D/H
labels and public-key lengths. There is no algorithm/domain-string selection API.

D-29: fixed SHA-256, extract+expand HKDF-SHA-256 output32 and full HMAC-SHA-256 Tag32
helpers remain internal. HMAC takes private Secret32 and authenticates via library
verify_slice only. Secret32 owns a fallibly allocated fixed heap buffer, redacts Debug
and zeroizes normal Drop. Existing audited Phase 1 secret types are unchanged.
Exposed PRK is explicitly zeroized; dependency-internal HKDF expand copies remain.
No new project unsafe, secret export, password input, PoP state or storage boundary.

D-26 dependency gate remains READY and exact pins/features are now integrated.
Phase 2A is implemented/tested only at its foundation boundary; all 19 original PKI
rows remain PLANNED. Q-02/Q-03 design is unchanged, with benchmark/recovery and trusted
time/host rollback still open. Phase 2B requires review and explicit approval; no
Phase 3 authorization. PHASE_2A_REVIEW.md / PHASE_2A_VALIDATION.txt record current evidence.
