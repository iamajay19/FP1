# Cryptographic Attribution and Immutable Decryption Provenance

An offline engineering research project for multi-recipient encrypted document
distribution and attribution of released document instances to cryptographic
identities. Attribution does not establish who personally leaked a document.

**Status: Phase 0 foundation IMPLEMENTED; runtime security features PLANNED.**
There is no encryption, decryption, ledger, or watermark implementation yet.
Do not use this scaffold to protect real documents. Phase 1 requires explicit approval.

## Start here

Read [AGENTS.md](AGENTS.md), [project specification](docs/PROJECT_SPEC.md), and
[implementation plan](docs/IMPLEMENTATION_PLAN.md). They are authoritative.
The [architecture](docs/ARCHITECTURE.md), [threat model](docs/THREAT_MODEL.md),
[protocol](docs/CRYPTO_PROTOCOL.md), and [decisions](docs/DECISIONS.md) describe
the proposed design, not tested security guarantees.

Phase 0 requires only Python 3.12+ and GNU Make; validation needs no downloads.

```sh
make help
make check
make environment
```

`make build`, `make encrypt`, `make decrypt`, `make pki`, `make ledger`, and
`make forensic` deliberately fail with a Phase 0 message. `make test`, `make lint`,
and `make fmt-check` validate only documentation and scaffolding.

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
| `scripts/` | Dependency-free Phase 0 validation and environment inspection |

See [environment evidence](docs/ENVIRONMENT.md), [test matrix](docs/TEST_MATRIX.md),
[limitations](docs/KNOWN_LIMITATIONS.md), [user guide](docs/USER_GUIDE.md), and
[Phase 0 review](docs/PHASE_0_REVIEW.md). The application plane specifies ML-KEM-768
and ML-DSA-65; Fabric infrastructure may retain classical cryptography. See
[cryptographic scope](docs/CRYPTOGRAPHIC_SCOPE.md).

The repository's license selection is unresolved; [LICENSE](LICENSE) does not
grant an open-source license. No third-party dependencies are bundled.
