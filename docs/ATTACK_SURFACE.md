# Attack Surface

Status: PLANNED. No runtime entry points exist yet.

| Input / surface | Trust boundary and required treatment | Phase |
|---|---|---|
| CADP header, lengths, CBOR, entries | Untrusted; enforce limits/overflow checks before allocations; reject duplicate/unknown fields and trailing bytes | 3/4 |
| Certificates, CRLs, trust anchors | Authenticated objects still require strict parsing; anchor installation is privileged and out of band | 2 |
| Encrypted private-key bundle/password | Bound KDF cost before allocation; authenticate public metadata; password via protected terminal, never CLI argv/logs | 2 |
| PDF/text parser and watermark library | Malformed input, decompression bombs, active content, allocation copies; no helper process receives raw plaintext | 7/9 |
| Ledger proposals/responses | Check roles, signed identity bindings, revocation, replay and valid committed transaction status | 6/8 |
| Output and evidence paths | No traversal/symlink overwrite, restrict permissions, immutable input snapshot, handle disk full and concurrent writers | 8/10 |
| Logs/errors/core dumps/swap | No secret formatting or plaintext; fail closed on required hardening failures | 5/8 |
| UI IPC and cancellation | Only high-level operations, no secret serialization; cancellation cleanup before release | 11 |
| Removable media/dependencies/builds | Pin and verify imports; no unreviewed executable fixtures or network auto-download | 13 |
| Ledger backups/admin commands | Access control, audit/checkpoint retention and rollback protection | 6/13 |
| Forensic candidate and reference data | Treat both as untrusted until authenticated; preserve candidate hash and collection metadata | 9/10 |

Phase 0 scripts only read local documents, inspect versions and test unavailable
Make targets. They never install dependencies, contact the public network, start
containers, or process real cryptographic material. Environment inspection may
contact the local Docker socket solely to query readiness.
