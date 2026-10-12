# Known Limitations

Status: Phase 1 primitives IMPLEMENTED/TESTED; later system security remains PLANNED.

| ID | Severity | Unresolved limitation / required follow-up |
|---|---|---|
| L-01 | HIGH | Usable recipient private keys permit custom-client decryption without watermark/ledger. Trusted workflow only; no universal enforcement or attestation (D-16) |
| L-02 | HIGH | Only PQ primitive wrappers and Phase 2A foundations exist; no protected key storage, PKI, document encryption, ledger, atomic release or watermark implementation. Never use this prototype for real secrets |
| L-03 | HIGH | Ledger immutability depends on infrastructure/admin trust; native classical cryptography and one orderer remain (Q-04) |
| L-04 | HIGH | Offline time/revocation freshness and rollback protection are not implemented; revocation cannot retract prior keys/content (Q-03) |
| L-05 | HIGH | Watermark survival, false-positive rates and resistance to transplant/collusion are unmeasured. Exact-hash verification is required for strong version-1 attribution (Q-05) |
| L-06 | MEDIUM | In-memory handling does not defend against root/kernel/hypervisor; library copies, swap/core dumps and output crash behavior need tests (Q-06) |
| L-07 | MEDIUM | Project-owned FFI has Phase 1 tests and a separate self-review; external review and ongoing ABI/support tracking remain necessary. Static binaries must be rebuilt for liboqs fixes. Tested on Fedora 44 x86_64 native Linux; other hosts/cross-compilation unverified. No FIPS module certification or constant-time proof claimed |
| L-08 | MEDIUM | Docker CLI/Compose present but daemon access denied; no Fabric readiness test possible (Q-04) |
| L-09 | MEDIUM | Local Python 3.14 satisfies 3.12+ but future dependency/wheel compatibility untested; no offline bundle exists (Q-07) |
| L-10 | LOW | Mermaid CLI, Markdown linter and Ruff absent; only fence/link/basic style checks and manual diagram review performed |
| L-11 | LOW | Historical Git read blocker resolved: git status/log now work; current HEAD 200bc4ab96a8d8fdf3bc5ffa846a668de7f375c8, .deps ignored and user metadata preserved. Git writes remain outside execution permission scope and untested (Q-09) |
| L-12 | LOW | License selection remains with owner; LICENSE grants no project license (Q-08) |
| L-13 | INFORMATIONAL | Separate adversarial self-review is not an external audit or formal verification (Q-10) |
| L-14 | INFORMATIONAL | Receipt commit proves acceptance of a signed claim, not human action, trusted execution or successful filesystem delivery |
| L-15 | MEDIUM | Long-term KEM key compromise can expose archived packages; forward secrecy is not provided |
| L-16 | MEDIUM | Ledger reveals recipient/document relationships and timestamps to operators; no metadata confidentiality mechanism specified |
| L-17 | HIGH | liboqs explicitly targets research/prototyping and discourages production/sensitive-data reliance. Supported release and FIPS-standard algorithm names do not establish production suitability or FIPS validation |
| L-18 | LOW | Clang ASan/UBSan instrument the C shim and native contract harness only; backend archive/assembly and Rust are not sanitizer-instrumented. LeakSanitizer cannot run in this execution surface (ptrace restriction); explicit native-handle counts are checked instead |
| L-19 | MEDIUM | Upstream OS RNG failure and some internal SHA3 allocation failures terminate the process, bypassing Rust Drop. Wrapper allocation/operation errors are typed where the C API returns control. Zeroization cannot cover abort/exit, backend/register copies, swap/dumps or privileged observers |

The inspected host has an 8192 KiB memory-lock soft limit, unlimited core dumps and
active zram swap. No host configuration was changed. Phase 5 must establish and test
required hardening before unlocking secrets; current defaults are not a secure-client
profile. See [ENVIRONMENT.md](ENVIRONMENT.md).

Open questions, owners and required phases are in [DECISIONS.md](DECISIONS.md).
Known limitations are not waived tests; planned negative/security tests remain in
[TEST_MATRIX.md](TEST_MATRIX.md). No unresolved CRITICAL finding was identified in
the Phase 0 document/scaffold review; this says nothing about unimplemented runtime safety.

Historical first Phase 1 preparation attempt (2026-10-06) did not execute any
cryptographic tests or resolve Q-01; library/linkage checks were then BLOCKED.
Go, Docker/Fabric, GUI, PDF and ECC dependencies are not Phase 1 prerequisites.
See [ENVIRONMENT.md](ENVIRONMENT.md) for fresh evidence and host-side recovery steps.

Historical post-installation reinspection superseded the earlier missing-tool and Git-read
findings: host compilers are present and a Rust compiler-only executable ran.
The supported liboqs target is 0.16.0; neither that library nor a Rust linkage probe
has been built. Main-branch revision/compatibility cannot be established from
inconsistently cached web views. Q-01 remains open until source access and actual
integration proof succeed; no older unsupported library is approved.

Final local-source evaluation supersedes those historical Q-01 blockers. Clean
liboqs tag 0.16.0, commit `5a1a854b0dc9f2141bdc771c555ee60c37950183`, was built
and used by a successful offline Rust metadata probe. The current Rust bindings
were inspected but not selected: their external version range rejects this release
here, and their vendored library is post-release. No third-party source was patched.
The readiness probe uses opaque pointers and C-header struct access, creates no
keys and cleans up temporary builds. Only its reproducible script and text evidence
remain. Q-01 is RESOLVED for environment preparation; cryptographic correctness,
adversarial inputs, memory safety and all Phase 1 runtime tests remain PLANNED.
Future Go/Fabric integration and complete offline packaging remain Q-04/Q-07;
they do not block Phase 1. See [ENVIRONMENT.md](ENVIRONMENT.md) and
[PHASE_1_Q01_READINESS.txt](PHASE_1_Q01_READINESS.txt).

Authorized Phase 1 implementation supersedes the historical preparation status
above. Primitive runtime tests, boundary checks, empty-context interoperability,
cleanup observations and concurrent smoke tests now execute. See
[PHASE_1_REVIEW.md](PHASE_1_REVIEW.md) and
[PHASE_1_VALIDATION.txt](PHASE_1_VALIDATION.txt). No Phase 2 or later feature is
implemented. Generated secret types deliberately have no public import/export;
protected storage is a later reviewed boundary. Message processing is linear in
borrowed input length; future protocol callers must apply their message-size limits.
The strict offline build currently requires a clean Git checkout, not a plain source
archive. Complete offline distribution/clean-host validation remains Phase 13.

## Independent Astra findings and corrective history — 2026-10-07

The original independent model audit verdict was APPROVED WITH REQUIRED FIXES.
The original build enabled AVX-512 SHA3; its callback-table first-use race was
missed by the pthread-only self-review and by smoke tests on this non-AVX-512 host.
The native failure harness also did not prove Rust cleanup of partial outputs.
These findings remain part of the review history:

| Astra finding | Severity | Current disposition |
|---|---|---|
| H-01 | HIGH | FIXED / TESTED: explicit SHA3 AVX512VL OFF; generated/installed header validation and shim compile rejection; PQC-BUILD-002. Clean rebuild and ON-cache override verified |
| L-01 | LOW | FIXED / TESTED: PQC-SECRET-003 permanently covers four native partial-write failures through actual Rust Drop, cleansing and subsequent free; no manual fixture cleanse |
| M-01 | MEDIUM | RETAINED: same upstream fatal RNG/internal SHA3 allocation limitation as L-19. No fallback RNG, signal recovery, exit interception or backend patch |

The Astra L-01 identifier above is distinct from the existing HIGH project limitation
L-01 concerning recipient bypass; that limitation remains unresolved. Organizational
audit Q-10 also remains open. H-01/L-01 corrections await independent Astra re-audit
before any Phase 2 approval. See [PHASE_1_REVIEW.md](PHASE_1_REVIEW.md) and
[PHASE_1_CORRECTIONS_VALIDATION.txt](PHASE_1_CORRECTIONS_VALIDATION.txt).

No AVX-512 hardware execution, ThreadSanitizer, full-backend sanitizer coverage,
LeakSanitizer success or FIPS module validation is claimed. The security correction
excludes the affected AVX-512 dispatch implementation from the build. Zeroization
remains best effort and cannot cover process termination; later trusted-workflow
phases must fail closed across process death rather than release unaudited output.

## Phase 2 authorized dependency blocker — 2026-10-07

The independent Astra re-audit APPROVED Phase 1; historical pending-review wording
above is preserved as history. Phase 2 is now authorized, but not implemented.
MEDIUM environment limitation: reviewed AES-GCM and Argon2 Rust sources are absent
from both inspected Cargo caches and no alternative offline vendor bundle was found.
Four actual offline resolution checks failed with exit 101. The user's explicit
dependency stop condition prevents implementation until a pinned offline dependency
closure is supplied and reviewed. This does not invalidate Phase 1 primitive approval.
See [PHASE_2_REVIEW.md](PHASE_2_REVIEW.md) and [PHASE_2_VALIDATION.txt](PHASE_2_VALIDATION.txt).
No benchmark or PKI/storage test ran; all Phase 2 runtime IDs remain PLANNED.
M-01/L-19, host-memory and time/rollback limitations remain unresolved. Phase 3 remains
unapproved. No protocol, production secret API or unsafe boundary was changed.

## Refined Phase 2 dependency gate — 2026-10-07

Earlier missing-AES/Argon2 statements describe historical inspection. The owner's
vendor graph now passes integrity and offline compilation/API checks. This does not
complete Phase 2. Remaining dependency blocker: reviewed zeroize sources are absent,
preventing AES expanded-key/GHASH and Argon2 temporary cleanup feature selection.
Argon2 internal alloc workspace is not wiped on Drop even with zeroize; future code
must own/bound/cleanse it safely on all paths. Software cleanup still cannot guarantee
physical erasure, removal of all library/register copies, swap or termination cleanup.

The RNG default on this host allows an OS-device fallback; syscall-only configuration
was proved and must be enforced during integration. rpassword returns a plain String;
its internal logical-byte wipes do not erase reallocations/deleted buffers, and input
is not internally length bounded. Normal terminal restoration is best effort and
process signals can bypass Drop. No production secret owners/input code were added.

The supplied graph needs Rust >=1.89, which the host meets. Vendored Cargo/file/owner
checksums prove consistency, not malicious-supplier resistance, registry provenance
or new-release advisory clearance. No network/current advisory lookup was permitted.
Full dependency unsafe review and target coverage are not claimed. CBOR route, Q-02
PoP/KDF benchmark and existing offline time/rollback assumptions remain open; no
protocol changed. Phase 1 M-01 remains accepted/open and unaffected.

[PHASE_2_REVIEW.md](PHASE_2_REVIEW.md) contains the full inventory and narrowed blocker.
All 19 Phase 2 runtime tests remain PLANNED; no implementation or runtime pass claimed.


## Final Phase 2 pre-implementation gate refinement — 2026-10-07

Historical zeroize-absence statements above are superseded by actual refreshed
source/feature/build proof. The cleanup-enabled closure is available and tested only
as a disposable dependency/API probe. Production Phase 2 remains NOT IMPLEMENTED.
Remaining MEDIUM environment blocker: minicbor 2.3.0, hmac 0.13.0 and hkdf 0.13.0
sources are absent; three offline negative resolution checks confirmed this. Candidate
upstream inspection does not replace local integrity review and a complete locked
build. All 19 existing runtime IDs remain PLANNED. No Argon2 production benchmark ran.

Cleanup limitations remain: AES/POLYVAL owned state Drop and Argon2 temporaries are
covered by enabled source features; Argon2 workspace is not automatically wiped.
SHA-2/Blake2 zeroize feature unification is additionally required and was built using
existing vendor sources. Future code must own and cleanse password, workspace, KEK,
PoP key/PRK, decrypted private-key plaintext and failure-path outputs with bounded,
fallible allocations and minimal copies. Zeroizing alone is not a role-safe public
secret API (it supports dereference/clone). Library/register/stack copies, old Vec
allocations and premature process termination prevent complete erasure guarantees.
HKDF's proposed API has intermediate PRK/output copies not all automatically erased;
use explicit owner cleanup where exposed and audit remaining copies at implementation.

D-23's minicbor route is not a claim that minicbor is canonical by default. It permits
non-profile forms; explicit bounded schema/type/duplicate checks and deterministic
re-encoding/exact comparison are mandatory. Unreviewed parser use or normalization
before signature verification could reintroduce ambiguity. The crate is not yet local
or compiled; future malformed-input coverage remains required.

D-24 resolves only Q-02's PoP protocol. Authenticated administrative provisioning is
required; air-gapped media alone are unauthenticated. PoP demonstrates live key access,
not identity, exclusive possession, prevention of relay to a willing key holder or
honesty of the CA. Its one-shot state and timers have not been implemented/tested.
KDF benchmark, recovery and CA/publisher lifecycle procedures remain open.

D-25 defines fail-closed certificate/CRL checks relative to explicitly configured local
time, with zero skew and cumulative monotonic sequence relative to retained state.
It supplies NO trusted time or whole-host anti-rollback. Old snapshots can restore
clock and sequence together; first bootstrap requires authenticated administrative
freshness. Import cadence/protected checkpoints/restore controls remain deployment
questions under Q-03/Q-04. Persistence failures must reject, not reset sequence.

Phase 1 M-01/L-19 remains OPEN/accepted: upstream fatal RNG/internal allocation may
terminate the process before Rust Drop. No fallback RNG, process interception,
custom entropy, new unsafe boundary, protocol downgrade or backend modification was
introduced. No FIPS module validation or production security certification claimed.


## Final Phase 2 dependency availability closure — 2026-10-07

The historical MEDIUM missing-source environment blocker is RESOLVED by D-26.
The owner supplied the three additions and their 34-package graph passed full file/
checksum/version checks plus an independent empty-home locked offline build/API probe.
This closes availability only: Phase 2 production, adversarial schema/PoP/bundle/CRL
runtime coverage and Argon2 benchmarking are NOT IMPLEMENTED/TESTED. All 19 IDs
remain PLANNED. Complete offline deployment/other targets remain Phase 13.

The added minicbor source has four unsafe blocks (transparent byte-slice casts and
initialized fixed-array ownership); review/triage found no dependency-gate blocker,
but no complete third-party memory-safety audit is claimed. HMAC/HKDF source forbids
unsafe; underlying existing hash/digest dependencies still require normal supply-chain
review. Checksums provide owner-bundle consistency, not malicious-supplier resistance,
upstream authenticity, current advisory clearance or legal approval. No network used.

D-23 bounded exact-schema decode/re-encode/equality remains mandatory. D-24 PoP and
D-25 time-policy documents are unchanged: signature covers KEM confirmation, implicit
rejection is not possession, and local clock/retained sequence provide no trusted time
or host/snapshot rollback protection. Cleanup ownership, HKDF copies, protected password
input, syscall-only production RNG and CA recovery remain future review obligations.
M-01/L-19 remains accepted/open; no backend patch, fallback RNG, custom primitive,
abort interception, raw secret export or new project unsafe was added.


## Phase 2A foundation residuals — 2026-10-08

The current utilities are not completed PKI, key storage, PoP or CRL processing.
The CBOR helper is private and supports only the approved bounded subset. Exact
protocol schema implementations must use validated(), perform field-specific length/
algorithm/policy checks and decode embedded objects through the shared-budget helper.
A successful structural scan alone is not certificate/protocol acceptance. Class
limits are reusable policies, not claims that a certificate or CRL parser exists.
Trusted typed encoders must stay simple/bounded; untrusted values cannot supply a
recursive generic encoding tree. Ordinary output Vec contains public canonical bytes
in this phase; unlocked private-key serialization needs separately reviewed cleanup.

SHA/HMAC cleanup features and explicit PRK/output ownership improve normal-Drop
cleanup. HKDF's internal expand output/previous-block copies are not universally
wiped; no physical-erasure guarantee, root defense or fatal-exit cleanup is claimed.
Secret32 has no Clone/Display/public raw export; its observation hook exists ONLY
under cfg(test), before heap free. New helper does not replace PQC SecretBuffer.
Argon2 workspace/password/bundle owners and production benchmark remain Phase 2B work.

Deterministic malformed/property loops ran without a new fuzz dependency; true
coverage-guided fuzzing/formal verification remain pending. Self-review is not an
independent Astra/organizational audit. Phase 1 M-01/L-19 remains OPEN/accepted;
no RNG fallback, backend patch, abort interception or new unsafe boundary.
Vendor checks cannot establish supplier authenticity/current advisory clearance.
Configured local time and retained CRL state still do not provide trusted time or
clock/whole-host/snapshot rollback protection. No time-policy runtime implemented.


## Phase 2A independent RNG finding and correction — 2026-10-12

P2A-A-M01 (MEDIUM): FIXED / READY FOR TARGETED ASTRA RE-AUDIT. The independent audit
demonstrated response-file CUSTOM dispatch despite the literal approved flag. The
original finding/verdict/evidence are preserved in PHASE_2A_ASTRA_AUDIT.md and
PHASE_2A_ASTRA_VALIDATION.txt; correction evidence is separate.

The approved build now rejects @response files rather than parsing arbitrary paths,
requires effective exclusive Linux cfg and forbids every recognized alternate cfg
at compile time. The rustdoc launcher is required because build.rs cannot inspect
configuration-supplied rustdoc flags. Disabling this launcher via supported Cargo
overrides is rejected. Build tools/wrappers/PATH/local sources remain trusted; these
checks do not sandbox a malicious compiler or prove toolchain authenticity. Cargo
may ask rustc to process flags before the project build script runs; some invalid
configurations reject in rustc/getrandom instead of the project guard. No accepted
project build results. This is not protection against arbitrary hostile response
files' effects inside a compromised compiler.

No randomness fallback, new unsafe, secret export, protocol or algorithm change.
M-01/L-19 fatal upstream failure remains OPEN/accepted and may bypass Rust Drop.
No TSan, successful LeakSanitizer, AVX-512 hardware or full-backend sanitizer claim.
Independent targeted review remains required; Phase 2B is unapproved and all 19 PKI
tests remain PLANNED. See PHASE_2A_CORRECTIONS_REVIEW.md.


## Targeted Astra LOW finding and final correction — 2026-10-12

The independent re-audit verified the original MEDIUM backend substitution fix but
found LOW P2A-RA-L01: cargo rustc per-crate @duplicate arguments escaped build.rs,
producing an approved-backend library despite the blanket response-file policy.
Its original report/verdict/evidence remain unchanged; no RNG downgrade was claimed.

P2A-RA-L01 is FIXED / READY FOR FINAL TARGETED ASTRA VERIFICATION by D-31: an actual-
argv Cargo rustc wrapper, required by build.rs, complements the unchanged compile-
time exclusivity/rustdoc guards. P2A-BUILD-005 tests initial and approved reused
targets, duplicate/response/alternate values, wrapper overrides and forwarding.
This is not independent approval. Hold Phase 2B/3; all 19 PKI IDs remain PLANNED.

The wrapper trusts Cargo's real tool, Clippy/Python/PATH and local source. It permits
flag-free non-artifact host queries and host build-script tools in explicit-target
builds, not runtime RNG consumers; it does not inject cfgs. Other workspace wrappers
are unsupported. Backend upgrades and new host dependencies require renewed policy
review. Checks do not authenticate/sandbox a malicious toolchain. M-01/L-19 fatal
termination, dependency copies, software erasure and offline time/rollback limitations
remain open and unchanged. See PHASE_2A_L01_CORRECTIONS_REVIEW.md and its validation.

Corrective self-review additionally observed that Clippy supplies extra CLI flags
through CLIPPY_ARGS after the Cargo argv boundary. An actual duplicate @file
was accepted by the initial wrapper design. The final wrapper conservatively
rejects @ and getrandom_backend markers in that extra channel without parsing
Clippy's private separator. Ordinary warning flags remain supported. Six real
Clippy rejection controls preserve coverage. This is a corrected implementation
issue, not a new protocol decision or a hidden change to Astra history.
