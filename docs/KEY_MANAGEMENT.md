# Key Management

Status: PLANNED. Phase 2 will implement provisioning, private-key storage and PKI.

| Role | Material | Custody / permitted use |
|---|---|---|
| Offline CA | Independent ML-DSA-65 signing key | Restricted offline administration; signs certificates and CRLs only |
| Publisher | Independent ML-DSA-65 signing key | Approved publisher registry; signs package manifests only |
| Recipient KEM | ML-KEM-768 key pair | Encrypted local key bundle; recover CEK only |
| Recipient signature | Independent ML-DSA-65 key pair | Same protected bundle with distinct role/nonces; signs receipts only |
| Fabric operator/client | Native MSP/TLS keys | Infrastructure plane; separate from application PQ keys |
| Per-document | Random CEK and nonce | One package encryption; erase after use |
| Per-recipient wrap | KEM shared secret, HKDF KEK and nonce | Scoped temporary values; erase immediately after wrap/recovery |

Provisioning must independently generate both recipient pairs using OS-backed
library randomness, recompute distinct fingerprints and verify possession without
logging secrets. Pin CA and publisher public trust anchors through an authenticated
out-of-band administrative process. Package-supplied trust anchors are never trusted.
Recipient labels such as Alice are display-only; cryptographic IDs are the bindings.

Store keys at rest using the [bundle protocol](CRYPTO_PROTOCOL.md); protect directory
permissions and avoid symlink-following writes. Do not put private keys, passwords,
source documents or real evidence in Git/test fixtures. Encrypted key material can
be backed up offline under controlled custody; never plaintext export as a debug
convenience. Backup encryption, password recovery policy and CA rotation procedures
must be finalized under Q-02 before real provisioning.

Rotate by issuing new IDs/certificates and preserving old public verification records.
Revoke either KEM or signing key through a signed cumulative CRL and ledger update;
reject new reservations/finalizations using revoked keys. Existing ciphertext can
still be decrypted by anyone retaining its usable KEM private key; republishing does
not erase old copies. Retain signed historical CRLs/certificates and ledger references
for forensics; disclose later compromise instead of hiding it.

A memory-hardened Rust process should handle unlocked material. Python orchestration
must not casually duplicate private-key bytes into immutable strings or subprocess
arguments. Zeroization is best-effort across library boundaries and is not proof of
physical erasure. Audit concrete allocation/lifetime paths in the implementing phases.
