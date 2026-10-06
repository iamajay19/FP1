# Known Limitations

Status: Phase 0 foundation only; no runtime security property TESTED.

| ID | Severity | Unresolved limitation / required follow-up |
|---|---|---|
| L-01 | HIGH | Usable recipient private keys permit custom-client decryption without watermark/ledger. Trusted workflow only; no universal enforcement or attestation (D-16) |
| L-02 | HIGH | No cryptographic, PKI, ledger, atomic-release or watermark implementation exists. Never use scaffold for real secrets |
| L-03 | HIGH | Ledger immutability depends on infrastructure/admin trust; native classical cryptography and one orderer remain (Q-04) |
| L-04 | HIGH | Offline time/revocation freshness and rollback protection are not implemented; revocation cannot retract prior keys/content (Q-03) |
| L-05 | HIGH | Watermark survival, false-positive rates and resistance to transplant/collusion are unmeasured. Exact-hash verification is required for strong version-1 attribution (Q-05) |
| L-06 | MEDIUM | In-memory handling does not defend against root/kernel/hypervisor; library copies, swap/core dumps and output crash behavior need tests (Q-06) |
| L-07 | MEDIUM | Fresh Phase 1 preparation confirms missing Rust/Cargo, CMake, C++, libclang and liboqs. Published bindings target liboqs 0.13.x, not current 0.16.0; maintained pairing/pins and runtime linkage remain unverified. Read-only system paths and unavailable shell dependency access block installation here. rustup is optional; Go/global pip are later-phase concerns (Q-01/Q-07) |
| L-08 | MEDIUM | Docker CLI/Compose present but daemon access denied; no Fabric readiness test possible (Q-04) |
| L-09 | MEDIUM | Local Python 3.14 satisfies 3.12+ but future dependency/wheel compatibility untested; no offline bundle exists (Q-07) |
| L-10 | LOW | Mermaid CLI, Markdown linter and Ruff absent; only fence/link/basic style checks and manual diagram review performed |
| L-11 | LOW | Exposed .git is an empty read-only tmpfs mount; underlying host history cannot be inferred. No Git diff/status/commit validation available; host-side inspection required before initialization (Q-09) |
| L-12 | LOW | License selection remains with owner; LICENSE grants no project license (Q-08) |
| L-13 | INFORMATIONAL | Separate adversarial self-review is not an external audit or formal verification (Q-10) |
| L-14 | INFORMATIONAL | Receipt commit proves acceptance of a signed claim, not human action, trusted execution or successful filesystem delivery |
| L-15 | MEDIUM | Long-term KEM key compromise can expose archived packages; forward secrecy is not provided |
| L-16 | MEDIUM | Ledger reveals recipient/document relationships and timestamps to operators; no metadata confidentiality mechanism specified |

The inspected host has an 8192 KiB memory-lock soft limit, unlimited core dumps and
active zram swap. No host configuration was changed. Phase 5 must establish and test
required hardening before unlocking secrets; current defaults are not a secure-client
profile. See [ENVIRONMENT.md](ENVIRONMENT.md).

Open questions, owners and required phases are in [DECISIONS.md](DECISIONS.md).
Known limitations are not waived tests; planned negative/security tests remain in
[TEST_MATRIX.md](TEST_MATRIX.md). No unresolved CRITICAL finding was identified in
the Phase 0 document/scaffold review; this says nothing about unimplemented runtime safety.

Phase 1 environment preparation (2026-10-06) did not execute any cryptographic tests
or resolve Q-01. Source-level algorithm identifiers are documented, but local
ML-KEM-768/ML-DSA-65 availability and Rust/liboqs linkage remain BLOCKED.
Go, Docker/Fabric, GUI, PDF and ECC dependencies are not Phase 1 prerequisites.
See [ENVIRONMENT.md](ENVIRONMENT.md) for fresh evidence and host-side recovery steps.
