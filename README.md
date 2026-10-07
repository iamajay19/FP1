# Cryptographic Attribution and Immutable Decryption Provenance

An offline engineering research project for multi-recipient encrypted document
distribution and attribution of released document instances to cryptographic
identities. Attribution does not establish who personally leaked a document.

**Status: Phase 1 PQC primitive wrappers IMPLEMENTED; later runtime features PLANNED.**
There is no encryption, decryption, ledger, or watermark implementation yet.
Do not use this prototype to protect real documents. Phase 2 requires explicit approval.

## Start here

Read [AGENTS.md](AGENTS.md), [project specification](docs/PROJECT_SPEC.md), and
[implementation plan](docs/IMPLEMENTATION_PLAN.md). They are authoritative.
The [architecture](docs/ARCHITECTURE.md), [threat model](docs/THREAT_MODEL.md),
[protocol](docs/CRYPTO_PROTOCOL.md), and [decisions](docs/DECISIONS.md) describe
the proposed design, not tested security guarantees.

Phase 1 needs Rust/Cargo, native C/CMake tools and the supplied pinned liboqs source.
The primitive crate has no registry dependencies and builds offline. See
[trusted-client](trusted-client/README.md) for the API and exact source pin.

```sh
make help
make check
make environment
```

`make build` compiles the primitive library; `make test`, `make lint` and
`make fmt-check` run Rust tests, strict Clippy and rustfmt respectively. `make check`
also checks documentation, phase scope, native failure paths and source pinning.
`make encrypt`, `make decrypt`, `make pki`, `make ledger` and `make forensic`
remain unavailable. Historical Phase 0 evidence and test IDs are preserved.

## Repository map

| Path | Responsibility / first implementation phase |
|---|---|
| `docs/` | Specifications, evidence, decisions, and review gates (0) |
| `pki/` | Offline CA and encrypted identity provisioning (2) |
| `publisher/` | Single-copy multi-recipient packaging (3) |
| `trusted-client/` | Rust PQ primitive integration (1), trusted workflow (5/8) |
| `ledger/network/` | Local Fabric topology (6) |
| `ledger/chaincode/` | Go registration, reservation, finalization (6) |
| `forensic/` | Evidence extraction and independent verification (10) |
| `tests/integration/` | Cross-component and atomic-release tests (later phases) |
| `tests/adversarial/` | Tampering, fuzzing, failure injection (later phases) |
| `tests/fixtures/` | Synthetic public test data only |
| `scripts/` | Environment, documentation, native and source-pin validation |

See [environment evidence](docs/ENVIRONMENT.md), [test matrix](docs/TEST_MATRIX.md),
[limitations](docs/KNOWN_LIMITATIONS.md), [user guide](docs/USER_GUIDE.md), and
[Phase 0 review](docs/PHASE_0_REVIEW.md). The application plane specifies ML-KEM-768
and ML-DSA-65; Fabric infrastructure may retain classical cryptography. See
[cryptographic scope](docs/CRYPTOGRAPHIC_SCOPE.md).

The repository's license selection is unresolved; [LICENSE](LICENSE) does not
grant an open-source license. Local ignored `.deps/` trees are inspection/build
inputs, not a completed offline distribution bundle.
