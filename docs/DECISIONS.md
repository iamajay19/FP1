# Decision Log

Status: Phase 0 baseline. Decisions below select design contracts; they do not
approve later implementation phases or establish tested security.

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

## Open decisions and resolution gates

| ID | Issue / alternatives to evaluate | Owner and deadline |
|---|---|---|
| Q-01 | BLOCKED after approved environment preparation: Rust/Cargo, CMake, C++ and libclang missing; system paths read-only and shell dependency access unavailable. Published oqs 0.11.0 / oqs-sys 0.11.0+liboqs-0.13.0 identified, but maintained version selection, exact toolchain/lockfile pins and local algorithm/linkage verification remain unresolved; see ENVIRONMENT.md | Implementer and host owner, before Phase 1 tests; no fallback primitives |
| Q-02 | Key-bundle KDF benchmark, CA/publisher key lifecycle, provisioning possession proof and recovery procedure | PKI implementer, Phase 2 before deployment |
| Q-03 | Trusted local time and persistent anti-rollback CRL epochs; permitted validity skew and import cadence | PKI/ledger operators, Phases 2/6; unavailable freshness rejects |
| Q-04 | Deterministic liboqs verification in Go/external chaincode, Fabric mapping/ACLs, authenticated two-peer commit evidence and rollback checkpoints | Ledger implementer, Phase 6 before real release integration |
| Q-05 | Watermark ECC/repetition profile, PDF reference signature/storage, carrier alignment/capacity and thresholds | Watermark implementer, Phase 7 (text) and Phase 9 (PDF) |
| Q-06 | In-process PDF library suitability, secret copies, memory limits, output path/crash semantics | Core implementer, Phases 5/8/9; reject unsupported environment |
| Q-07 | Offline pinned dependency bundle, licenses and clean-host compatibility (Python 3.14 observed) | Deployment implementer, Phase 13; versions pinned as each component enters scope |
| Q-08 | Project licensing and ownership notices | Project owner, before external distribution |
| Q-09 | Git metadata inaccessible/unrecognized: fresh inspection shows empty read-only tmpfs mounted at .git; underlying host history unknown. Inspect outside this surface before deciding whether initialization is appropriate | Workspace owner, before version-control operations |
| Q-10 | Independent organizational audit beyond this agent's separate review pass | Project owner before any real-document deployment |

Phase 0 resolves ambiguities by specifying initial contracts, not by claiming these
implementation-dependent questions are experimentally settled. Q-01 blocks Phase 1
execution in the inspected environment until prerequisites are made available and
the user explicitly approves the next phase. No other phase is implicitly approved.

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
