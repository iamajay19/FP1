# Cryptographic Scope

Status: PLANNED, not a claim about an implemented or certified system.

The application security plane fixes ML-KEM-768 (FIPS 203), ML-DSA-65 (FIPS 204),
AES-256-GCM, HKDF-SHA-256, SHA-256, and OS CSPRNG. CA certificates, publisher
signatures and recipient receipt signatures use ML-DSA-65. Recipients have
independent KEM/signing keys and distinct identifiers. Argon2id protects local
password-derived key storage; it does not replace KEM or signature algorithms.
No RSA/ECDH/ECDSA/Ed25519/X25519 substitution is allowed at this application layer.

The ledger infrastructure plane may retain native Fabric MSP certificates, TLS,
transaction identity and consensus cryptography. Unmodified Fabric is not claimed
to be entirely post-quantum. Classical ledger compromise may affect authorization,
ordering, freshness and history even when a forged application signature is rejected.
Application PQ signatures alone do not make ledger commit evidence quantum-safe.

Use standardized algorithms through maintained libraries, preferably liboqs; no
custom primitives. Using FIPS-named algorithms is not FIPS module validation or a
proof of protocol security. No library version or production suitability has been
validated in Phase 0. Exact versions/build options must be pinned in Phase 1.

References: [FIPS 203](https://doi.org/10.6028/NIST.FIPS.203),
[FIPS 204](https://doi.org/10.6028/NIST.FIPS.204), and
[liboqs scope](https://openquantumsafe.org/liboqs/).
