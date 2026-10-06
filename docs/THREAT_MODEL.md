# Threat Model

Status: PLANNED; Phase 0 design review only. Assets are source/plaintext documents,
CEK/KEK/shared secrets, passwords/private keys, identity mappings, signed package
integrity, ledger history, output provenance, watermark evidence, and availability.

Adversaries include an unauthorized package holder, a malicious recipient, package
or media tamperer, receipt forger, replaying network client, malicious input author,
and watermark remover. Administrators and privileged host compromise are treated
explicitly in [SECURITY_ASSUMPTIONS.md](SECURITY_ASSUMPTIONS.md), not hidden behind
an air-gap claim. Entry points are catalogued in [ATTACK_SURFACE.md](ATTACK_SURFACE.md).

| Threat | Boundary / attack | Planned control | Residual / test IDs |
|---|---|---|---|
| T-01 | Package holder attempts key recovery | ML-KEM-768, per-recipient authenticated CEK wrap | No forward secrecy for archived ciphertext; PQC-KEM-002, ENC-004 |
| T-02 | Substitute identities, keys or publisher | CA signatures, pinned role authorization, exact ID/key binding | Compromised CA remains trusted; PKI-CERT-004, PKI-KEY-004 |
| T-03 | Alter package or mix recipient entries | Signed deterministic manifest covers entries and ciphertext hash; explicit AAD | PKG-TAMPER-001 through 004, ENC-006/007 |
| T-04 | Oversized, ambiguous or malicious encodings/PDF | Bounds before allocation, strict schema, parser fuzzing, no active PDF execution | Resource exhaustion still possible; PKG-PARSE-010, FUZZ-001 |
| T-05 | Extract keys/plaintext through logs, temp files, dumps | In-process watermarking, redacted errors, scoped zeroization, memory controls | Root and allocator copies not solved; MEM-002/003, MEM-ATOMIC-007/008 |
| T-06 | Release before committed receipt | Explicit state machine, valid commit evidence, no output opened early | Receipt does not prove delivery; MEM-ATOMIC-001 through 008 |
| T-07 | Replay/finalize someone else's reservation | Bound identity/document/key/nonces, ML-DSA verification, unique immutable state | Transaction conflicts must fail closed; LEDGER-REPLAY-001, LEDGER-CONC-001 |
| T-08 | Stale CRL, clock rollback, revocation race | Signed monotonic revocation epochs, local trusted time, recheck at finalization | Offline freshness needs operational policy; PKI-REV-004, LEDGER-REV-001 |
| T-09 | Remove, transplant or average watermarks | Document binding, receipt signature, exact output hash, conservative confidence | Transformation/collusion may erase or transplant evidence; FORENSIC-013/014 |
| T-10 | Patch client or use exported KEM key | Approved-client operational assumption only | HIGH unresolved architectural limitation; no software-only enforcement claim |
| T-11 | Colluding ledger admins or restoring old snapshots | Both-org endorsements, retained independent audit/checkpoints, controlled restores | Conditional immutability; single orderer is an availability weakness |
| T-12 | Malicious paths/concurrent operations | Exclusive workflow state, immutable final buffer, no-clobber output and symlink checks | FS and crash semantics need tests; MEM-ATOMIC-009, CLIENT-CONC-001 |
| T-13 | Offline dependency or removable-media compromise | Pinned verified artifacts, reviewed imports, no telemetry or auto-fetch | Supply chain cannot be inferred safe; OFFLINE-001 |

## Claims and nonclaims

The goal is evidence associating a generated instance with a cryptographic identity.
It does not prove human intent, that every decryption is logged, or that a signed
receipt attests an unmodified client. The issuer already has the source document.
Identical content ciphertext is deliberately shared; recipients have independent
KEM wrappers, not independent content encryption.

A public watermark identifier/checksum is not a signature. An exact output hash
match plus independently verified receipt is stronger than a surviving watermark
in a changed file. Transformed/transplanted candidates require qualified evidence;
never infer verified attribution solely from an existing reservation ID.

Denial of service must fail closed rather than release content. Sensitive ledger
metadata (recipient/document relationships and timings) is not confidential from
ledger operators. Logs must exclude secrets and document contents.
