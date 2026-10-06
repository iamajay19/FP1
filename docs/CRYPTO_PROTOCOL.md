# Cryptographic Protocol

Status: PLANNED Phase 0 version-1 design baseline. No primitives or codecs are
implemented. Changes require a decision entry and affected tests before use.
[PROJECT_SPEC.md](PROJECT_SPEC.md) remains authoritative.

## Notation and deterministic bytes

`H(x)` is SHA-256 of exact bytes. `||` concatenates bytes. `B[n]` is an n-byte
CBOR byte string; `U64` is an unsigned integer up to 2^64-1; `T` is ASCII text.
`D(x)` is RFC 8949 section 4.2.1 core deterministic CBOR (bytewise key ordering,
not the alternate length-first profile). Use definite lengths and shortest integer
encodings; no floats, tags, indefinite items, duplicate keys or trailing bytes.
Schemas use exact text field names below. Reject unknown fields, invalid UTF-8,
noncanonical encoding and unsupported versions. Text identifiers are not names:
all identity/document/key IDs are B[32]. Human labels stay outside signed identity
selection. Times are U64 Unix seconds UTC; validity is `valid_from <= now < valid_until`.

`S(label, body) = D([label, body])`. Each signed object is an exact map:
`{body: <canonical byte string>, signature: B[3309]}` where `body` contains S bytes.
The verifier parses the expected label/schema, checks canonical byte equality,
and verifies the original bytes. Sign with pure ML-DSA-65 and empty algorithm
context; domain separation is in S. Do not substitute a prehash variant or sign
an unauthenticated JSON rendering. Receipts are S bytes, matching the specification's
`CanonicalReceipt`. Labels: `CADP-CERT-v1`, `CADP-CRL-v1`, `CADP-MANIFEST-v1`,
`CADP-RECEIPT-v1`. Bind issuer/signing-key selection inside each relevant body.

References: [CBOR deterministic encoding](https://www.rfc-editor.org/rfc/rfc8949.html#section-4.2.1),
[FIPS 203](https://doi.org/10.6028/NIST.FIPS.203),
[FIPS 204](https://doi.org/10.6028/NIST.FIPS.204).

## Algorithm and identifier rules

| Item | Version-1 encoding / size |
|---|---|
| ML-KEM-768 | Public key 1184, expanded private key 2400, ciphertext 1088, shared secret 32 bytes |
| ML-DSA-65 | Public key 1952, expanded private key 4032, signature 3309 bytes |
| AES-256-GCM | Key 32, nonce 12, full tag 16 bytes; encode ciphertext followed by tag |
| SHA-256 | 32-byte digest |
| HKDF-SHA-256 | Extract then expand, output 32 bytes |
| Random IDs/nonces | document_id, recipient_id, certificate_serial, receipt_nonce: independent OS-random B[32] |
| Key IDs | KEM: H(D(["CADP-KEM-ID-v1", public_key])); signing: H(D(["CADP-SIG-ID-v1", public_key])) |

Always check algorithm names exactly: `ML-KEM-768`, `ML-DSA-65`, `AES-256-GCM`.
Reject legacy Kyber/Dilithium identifiers. Independently generate recipient key pairs;
never derive one from the other. Key identifiers are public fingerprints, not key
material. Every trust anchor has a locally approved issuer B[32], ML-DSA public key
and permitted roles. CA and publisher signing keys are separate from recipient keys.
Size constants are format requirements to verify against selected library APIs in
Phase 1, not permission to implement the primitives manually.

## Certificate and revocation schemas

Certificate body (all fields required):

| Field | Type / rule |
|---|---|
| schema_version | U64 = 1 |
| certificate_serial, recipient_id | B[32] each |
| kem_algorithm, kem_key_id, kem_public_key | Exact ML-KEM name, B[32], B[1184] |
| signature_algorithm, signature_key_id, signature_public_key | Exact ML-DSA name, B[32], B[1952] |
| issuer | B[32], resolves to pinned CA trust anchor |
| valid_from, valid_until | U64, increasing |

Certificate validation checks every field, signature, expected identity, key sizes,
recomputed key IDs, separate IDs, time and current signed revocation state. CA
signatures authenticate enrollment, not possession of both private keys. Provisioning
must prove possession and protect the delivery path in Phase 2. Publisher authority
uses a separately pinned ML-DSA signing-key/role registry; a recipient certificate
alone never grants publishing, CA or ledger administration permission.

CRL body: `schema_version=1`, `issuer:B[32]`, `generated_at:U64`,
`sequence:U64`, `next_update:U64`, `revocations:array`. Each entry is an exact map
`{key_id:B[32], recipient_id:B[32], reason:T, revoked_at:U64}`. Reasons are bounded
ASCII (1..128 bytes). Sort entries by `(recipient_id,key_id)` bytewise and reject
duplicates. CRLs are cumulative per issuer; sequence strictly advances on updates.
Require `generated_at <= now < next_update`, no issuer/sequence rollback, and matching
recipient-key bindings. Missing/stale CRL rejects new operations. Maximum CRL size
is 16 MiB / 100,000 entries; certificate envelope max 16 KiB. Both size constraints
apply. Secure storage of accepted epochs and clock policy remains Q-03.

## Encrypt once and wrap per recipient

Generate new random document_id, CEK and content nonce for each new package. Never
reuse a CEK/nonce pair; retries discard the incomplete package and generate fresh
material. Read only the source document, without creating extra raw copies.

```text
AAD_C = D(["CADP-CONTENT-v1", 1, document_id, media_type,
           "AES-256-GCM", publisher_id, publisher_signature_key_id])
C = AES256GCM.Encrypt(CEK, content_nonce, P, AAD_C)
(ct_i, ss_i) = MLKEM768.Encaps(recipient_kem_public_key)
KEK_i = HKDF-SHA256(IKM=ss_i, salt=H(document_id),
                   info=ASCII("CADP-CEK-WRAP-v1") || recipient_id, L=32)
AAD_i = D(["CADP-WRAP-AAD-v1", document_id, recipient_id, kem_key_id, 1])
W_i = AES256GCM.Encrypt(KEK_i, fresh_random_12_byte_nonce, CEK, AAD_i)
```

The fixed-size recipient_id makes the specification's HKDF concatenation
unambiguous. Both HKDF stages are required, per [RFC 5869](https://www.rfc-editor.org/rfc/rfc5869.html).
Content AAD deliberately excludes the ciphertext hash/signature/recipient wraps,
avoiding a circular dependency. After C and wrappers exist, sign the complete
manifest containing H(C) and all recipient entries. Content is stored exactly once.
W_i is 48 bytes. Erase ss_i, KEK_i and publisher CEK when no longer needed.

Version 1 does not support changing recipient sets on an existing package:
republish with new document_id/CEK and a fresh document registration. A signed
manifest binds each recipient certificate hash and expected signing key in addition
to its KEM entry; substitutions cannot silently switch the receipt identity.

## Recipient processing

Verify package/publisher/certificate/current revocation and select the exact
recipient entry before committing a reservation. Confirm reservation commit before
unlocking keys. Validate private/public bundle consistency. Decapsulate ct_i,
derive KEK_i, authenticate W_i and require exactly 32 CEK bytes, then authenticate
C completely before giving any plaintext to the watermark engine. Library output
before tag verification must be withheld and erased on failure.

ML-KEM implicit rejection means a correctly sized modified ciphertext or wrong key
may return an unrelated secret without an error. Tests must require secret mismatch
and subsequent CEK-wrap authentication failure, not a decapsulation error. Truncated
inputs must be rejected before the FFI call. This refines the existing PQC-KEM-002
matrix expectation without changing the mandatory algorithms or phase sequence.

After in-memory watermarking and self-verification, freeze output bytes, hash and
sign the receipt defined in [LEDGER_PROTOCOL.md](LEDGER_PROTOCOL.md), then await
valid matching commit. No output file on failed or unknown commit.

## Password-protected private-key bundle

Status: design only, implemented in Phase 2. Use Argon2id version 0x13 with random
16-byte salt and 32-byte output, then AES-256-GCM separately for each independent
private key under that local KEK with distinct random 12-byte nonces. Include the
full public bundle header and role (`kem`/`signature`) in canonical AAD. Reject
nonce equality, metadata substitution and wrong passwords without secret diagnostics.
Fresh salt/nonces on each rewrite; never edit ciphertext under an old nonce.

Header: schema_version, recipient_id, both key IDs/public keys/algorithm names,
certificate hash, KDF name/version/memory/time/parallelism/salt; body contains the
two nonce/ciphertext pairs. Envelope max 32 KiB. Before running Argon2, bound
memory to 64..256 MiB, time cost 3..6, parallelism 1..4. Initial candidate is
64 MiB, 3 passes, parallelism 4; benchmark on target hosts before selecting an
approved profile. No silent cost reduction. See [RFC 9106](https://www.rfc-editor.org/rfc/rfc9106.html).
Passwords are read from a protected terminal, never argv/environment/logs.
CA and publisher private-key storage needs the same protection and role separation.
