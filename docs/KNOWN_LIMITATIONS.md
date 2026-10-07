# Known Limitations

Status: Phase 1 primitives IMPLEMENTED/TESTED; later system security remains PLANNED.

| ID | Severity | Unresolved limitation / required follow-up |
|---|---|---|
| L-01 | HIGH | Usable recipient private keys permit custom-client decryption without watermark/ledger. Trusted workflow only; no universal enforcement or attestation (D-16) |
| L-02 | HIGH | Only PQ primitive wrappers exist; no protected key storage, PKI, document encryption, ledger, atomic release or watermark implementation. Never use this prototype for real secrets |
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
