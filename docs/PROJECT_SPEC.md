# Project Specification

## Cryptographic Attribution and Immutable Decryption Provenance for Multi-Recipient Encrypted Document Distribution

---

# 1. Purpose

This project implements an offline secure document-distribution system capable of cryptographically associating a released plaintext document instance with the recipient cryptographic identity that generated it.

The project addresses a limitation of conventional encrypted document distribution:

Once an authorized recipient decrypts a file, conventional encryption does not normally distinguish that recipient's plaintext copy from another authorized recipient's plaintext copy.

Traditional access logs are insufficient because:

- they may be mutable;
- they identify access events rather than leaked document instances;
- they do not necessarily bind the leaked document itself to a recipient.

Static watermarks are insufficient because identical recipients may receive identical watermarks or watermarks may be applied independently of authenticated decryption.

This system combines:

1. post-quantum recipient key establishment;
2. post-quantum recipient signatures;
3. authenticated content encryption;
4. immutable local provenance;
5. atomic in-memory watermark application;
6. forensic extraction.

---

# 2. Security Claim

The project attempts to demonstrate:

> A recovered watermarked document instance can be cryptographically associated with the recipient cryptographic identity whose trusted decryption workflow generated that instance.

The system does **not** claim to prove:

> the human owner of that identity personally leaked the document.

A recipient may voluntarily provide their decrypted copy to another person.

Therefore forensic evidence establishes provenance of the generated document instance, not human intent.

---

# 3. Deployment Constraint

The finished system must support a fully offline/air-gapped environment.

The runtime system must not require:

- public internet access;
- cloud KMS;
- public blockchain;
- cloud database;
- online CA;
- external identity provider;
- SaaS;
- remote telemetry;
- CDN assets;
- third-party network APIs.

Development-time dependency downloads are permitted.

Final deployment must be reproducible offline.

---

# 4. Major Components

The system contains four primary security components plus publisher tooling.

```text
                      AIR-GAPPED ENVIRONMENT

     +-----------------------+
     | Offline PQC CA / PKI  |
     +-----------+-----------+
                 |
                 | PQ certificates
                 v
+-----------+    +-----------------------+
| Publisher |--->| Encrypted .cadp File |
+-----------+    +-----------+-----------+
                            |
                            v
                 +-----------------------+
                 | Trusted Client        |
                 |                       |
                 | ML-KEM                |
                 | AES-GCM               |
                 | RAM-only plaintext    |
                 | watermarking          |
                 | ML-DSA receipt        |
                 +-----------+-----------+
                             |
                             v
                 +-----------------------+
                 | Permissioned Ledger   |
                 | Hyperledger Fabric    |
                 +-----------+-----------+
                             |
                             |
                      forensic query
                             |
                             v
                 +-----------------------+
                 | Forensic Extractor    |
                 +-----------------------+
```

---

# 5. Recommended Technology Stack

## 5.1 Trusted Client

Language:

**Rust**

Responsibilities:

- encrypted package parsing;
- package authenticity verification;
- recipient identity selection;
- private-key unlock;
- ML-KEM decapsulation;
- CEK recovery;
- AES-256-GCM decryption;
- secure in-memory plaintext handling;
- watermark application;
- watermark self-check;
- receipt generation;
- ML-DSA signing;
- ledger interaction;
- output release.

Suggested libraries:

- `oqs` / `oqs-sys`
- `aes-gcm`
- `hkdf`
- `sha2`
- `zeroize`
- `secrecy`
- `serde`
- deterministic CBOR implementation
- `thiserror`
- `lopdf`
- OS RNG facilities

No cryptographic logic should be implemented in JavaScript.

---

# 6. Administrative and Forensic Tooling

Language:

**Python 3.12+**

Responsibilities may include:

- CA tooling;
- recipient provisioning;
- certificate inspection;
- publisher tooling if appropriate;
- watermark extraction;
- forensic analysis;
- evidence bundle generation;
- automated integration testing.

Suggested libraries:

- `liboqs-python`
- `PyMuPDF`
- `pytest`
- `Hypothesis`
- `reedsolo`

---

# 7. Ledger

Preferred ledger:

**Hyperledger Fabric**

Deployment:

Docker Compose.

Initial logical topology:

```text
IssuerOrg
  |
  +-- peer0

AuditorOrg
  |
  +-- peer0

Ordering Service
  |
  +-- orderer0
```

Application channel:

```text
provenancechannel
```

Suggested chaincode language:

**Go**

Suggested endorsement policy for critical receipt operations:

```text
AND('IssuerMSP.peer','AuditorMSP.peer')
```

The exact policy may be adjusted during implementation, but changes must be documented.

---

# 8. Post-Quantum Scope

## 8.1 Application Security Plane

Must use NIST-standardized PQC.

### Key encapsulation

FIPS 203:

**ML-KEM-768**

### Signatures

FIPS 204:

**ML-DSA-65**

### Symmetric encryption

AES-256-GCM

### Key derivation

HKDF-SHA-256

### Hashing

SHA-256

---

## 8.2 Ledger Infrastructure Plane

Stock Hyperledger Fabric may use classical cryptography internally.

That limitation must be explicitly documented.

The project must never falsely claim:

> the entire infrastructure is cryptographically post-quantum.

Instead the correct statement is:

> the document-security and recipient-attribution protocol uses standardized post-quantum application cryptography, while the permissioned ledger infrastructure may retain Fabric's native classical identity mechanisms.

---

# 9. Recipient Key Model

Every recipient receives two cryptographically independent PQ key pairs.

## 9.1 KEM Key Pair

Algorithm:

ML-KEM-768

Fields:

```text
kem_key_id
kem_public_key
kem_private_key
```

Purpose:

Recover the document CEK.

---

# 10. Signature Key Pair

Algorithm:

ML-DSA-65

Fields:

```text
signature_key_id
signature_public_key
signature_private_key
```

Purpose:

Authenticate decryption receipts.

---

# 11. Key Separation Requirement

KEM and signature keys must never:

- share secret material;
- derive from one another;
- use one another's private keys;
- share identifiers.

---

# 12. Private-Key Storage

Private keys must be encrypted at rest.

Prototype design:

```text
password
   |
   v
Argon2id
   |
   v
local KEK
   |
   +---- AES-256-GCM(ML-KEM private key)
   |
   +---- AES-256-GCM(ML-DSA private key)
```

The implementation must store:

- salt;
- KDF parameters;
- encryption nonces;
- encrypted private keys;
- key identifiers;
- relevant public metadata.

The password must never be stored.

Unlocked private keys must remain inside the trusted process.

Sensitive buffers must be zeroized where practical.

---

# 13. Application-Level PQ Certificate

A lightweight deterministic PQ application certificate will be used rather than requiring a fully PQ-enabled X.509 stack.

Logical certificate:

```text
schema_version
certificate_serial
recipient_id

kem_algorithm
kem_key_id
kem_public_key

signature_algorithm
signature_key_id
signature_public_key

issuer
valid_from
valid_until
```

The certificate must be deterministically serialized.

Recommended signed representation:

canonical CBOR.

The CA signs the certificate bytes using ML-DSA.

---

# 14. Certificate Validation

Validation must verify:

1. supported schema version;
2. deterministic format;
3. certificate signature;
4. trusted issuer;
5. certificate validity period;
6. supported algorithms;
7. key identifiers;
8. revocation status;
9. expected key sizes/format;
10. required fields.

Failure must reject the operation.

---

# 15. Revocation

Support a signed revocation list.

Logical record:

```text
schema_version
issuer
generated_at
revocations[]
```

Each revocation contains at minimum:

```text
key_id
recipient_id
reason
revoked_at
```

The revocation list itself must be ML-DSA signed.

---

# 16. Document Encryption Model

The document is encrypted only once.

Let plaintext be:

\[
P
\]

Generate:

\[
CEK \leftarrow CSPRNG(32)
\]

Generate a fresh 96-bit content nonce:

\[
N_C
\]

Encrypt:

\[
C =
AES256GCM.Encrypt(
CEK,
N_C,
P,
AAD_C
)
\]

`C` is shared by all authorized recipients.

---

# 17. Recipient CEK Encapsulation

For each recipient \(R_i\):

\[
(ct_i, ss_i)=MLKEM.Encaps(pk_i)
\]

where:

- \(ct_i\) is the ML-KEM ciphertext;
- \(ss_i\) is the KEM shared secret.

Derive recipient-specific key-encryption key:

\[
KEK_i =
HKDF\_SHA256(
IKM=ss_i,
salt=SHA256(documentID),
info="CADP-CEK-WRAP-v1" || recipientID
)
\]

Generate:

\[
N_i \leftarrow CSPRNG(12)
\]

Wrap CEK:

\[
W_i =
AES256GCM.Encrypt(
KEK_i,
N_i,
CEK,
AAD_i
)
\]

Recommended recipient-specific AAD:

```text
document_id
recipient_id
kem_key_id
manifest_version
```

The ML-KEM shared secret and KEK must be zeroized after use.

---

# 18. `.cadp` Package

Package extension:

```text
.cadp
```

The exact binary container may be chosen during implementation, but parsing must be deterministic and hardened.

Logical contents:

```text
manifest.cbor
encrypted-content.bin
issuer-signature.bin
recipient entries
```

A recipient entry includes:

```text
recipient_id
kem_key_id
kem_ciphertext
cek_wrap_nonce
wrapped_cek
```

---

# 19. Manifest

The manifest should contain at minimum:

```text
schema_version
document_id
media_type
content_algorithm
content_nonce
encrypted_content_sha256
publisher_id
recipient_entry_metadata
creation_metadata
```

Anything signed must be serialized deterministically.

Do not sign arbitrary human-formatted JSON.

---

# 20. Package Authenticity

Publisher must authenticate the package using ML-DSA.

The package verifier must reject:

- modified manifest;
- modified recipient entries;
- unsupported schema;
- modified encrypted-content hash;
- invalid publisher signature;
- malformed encoding.

---

# 21. Trusted Client API Boundary

The trusted client must expose only high-level operations.

Examples:

```text
inspect_package()
verify_package()
decrypt_and_attribute()
```

It must not expose:

```text
decrypt_raw()
get_plaintext()
get_cek()
export_unwatermarked()
```

The UI must never receive unwatermarked plaintext.

---

# 22. Trusted Decryption State Machine

Required logical states:

```text
PACKAGE_LOADED
        |
        v
PACKAGE_VALIDATED
        |
        v
RECIPIENT_VALIDATED
        |
        v
RESERVATION_COMMITTED
        |
        v
KEYS_UNLOCKED
        |
        v
CEK_RECOVERED
        |
        v
CONTENT_DECRYPTED_IN_MEMORY
        |
        v
WATERMARK_APPLIED
        |
        v
WATERMARK_SELF_VERIFIED
        |
        v
RECEIPT_SIGNED
        |
        v
RECEIPT_COMMITTED
        |
        v
OUTPUT_RELEASED
        |
        v
CLEANUP_COMPLETE
```

Invalid transitions must fail.

---

# 23. In-Memory Plaintext Requirement

Unwatermarked plaintext may exist temporarily in memory because watermarking requires plaintext access.

It must never intentionally be:

- written to disk;
- stored in a temporary file;
- stored in application cache;
- logged;
- copied to clipboard;
- transmitted through UI IPC;
- sent to an external process;
- returned through a public API.

---

# 24. Memory Hardening

On supported Linux systems investigate:

```text
mlock
mlockall
MADV_DONTDUMP
PR_SET_DUMPABLE
RLIMIT_CORE
```

Sensitive buffers should use explicit zeroization.

Document that these protections do not defend against:

- compromised kernel;
- privileged debugger;
- malicious hypervisor;
- physical memory attacker.

---

# 25. Two-Phase Ledger Protocol

A watermark cannot safely depend on an identifier that does not yet exist.

Therefore use two phases.

---

# 26. Phase A — Reserve Decryption

Before plaintext decryption:

```text
ReserveDecryption(
    document_id,
    recipient_id,
    signature_key_id,
    nonce,
    relevant metadata
)
```

The transaction commits.

Its transaction identifier becomes:

```text
reservation_txid
```

This reservation ID becomes the forensic watermark payload or a component of that payload.

---

# 27. Phase B — Finalize Receipt

After in-memory watermarking:

1. hash final watermarked output;
2. construct canonical receipt;
3. sign receipt with recipient ML-DSA key;
4. submit `FinalizeReceipt`;
5. ledger verifies signature;
6. ledger commits receipt;
7. trusted client confirms commit;
8. only then may output be written.

---

# 28. Atomic Release Invariant

Mandatory invariant:

\[
OUTPUT\_RELEASED \Rightarrow RECEIPT\_COMMITTED
\]

Mandatory failure behavior:

If receipt finalization fails after successful decryption and watermarking:

```text
NO plaintext output file may exist.
```

This must have an automated failure-injection test.

---

# 29. Ledger State

Recommended key structure:

```text
IDENTITY::<recipient_id>::<signature_key_id>

DOCUMENT::<document_id>

RESERVATION::<reservation_txid>

RECEIPT::<reservation_txid>
```

---

# 30. Suggested Chaincode API

```text
RegisterRecipient
RevokeRecipientKey
RegisterDocument
ReserveDecryption
FinalizeReceipt
GetReservation
GetReceipt
GetReceiptsForDocument
GetReceiptsForRecipient
```

Keep chaincode small.

---

# 31. Receipt

Use canonical deterministic serialization.

Recommended:

canonical CBOR.

Logical receipt:

```text
schema_version
event_type
reservation_txid
document_id
encrypted_content_sha256
manifest_sha256
recipient_id
signature_key_id
kem_key_id
watermark_scheme
watermark_payload_sha256
final_output_sha256
trusted_client_version
trusted_client_build_id
receipt_nonce
optional_claimed_timestamp
```

---

# 32. Receipt Signature

Recipient computes:

\[
\sigma =
MLDSA65.Sign(
sk_{sig},
CanonicalReceipt
)
\]

Chaincode retrieves the registered ML-DSA public key and verifies:

\[
MLDSA65.Verify(
pk_{sig},
CanonicalReceipt,
\sigma
)
\]

Receipt finalization must fail when verification fails.

---

# 33. FinalizeReceipt Validation

Chaincode must check:

1. payload parses safely;
2. supported schema;
3. reservation exists;
4. reservation is not already finalized;
5. recipient ID matches reservation;
6. document ID matches reservation;
7. signing key matches expected identity;
8. signing key is active;
9. field sizes are valid;
10. signature is valid;
11. replay/duplicate finalization is prevented.

---

# 34. Watermark Payload

The watermark should not encode arbitrary unstructured data.

Logical frame:

```text
MAGIC
VERSION
RESERVATION_IDENTIFIER
DOCUMENT_BINDING
CHECKSUM
ECC
```

The exact frame should be versioned.

---

# 35. Text Watermark Prototype

Initial proof-of-concept may map bits to zero-width Unicode characters.

Example concept:

```text
0 -> U+200B
1 -> U+200C
```

Add:

- framing;
- redundancy;
- document binding;
- checksum;
- optional error-correcting code.

This technique must be documented as fragile against:

- Unicode normalization;
- sanitization;
- copy/paste pipelines;
- format conversions.

---

# 36. PDF Watermark

Primary forensic watermark should use subtle coordinate modulation of selected PDF text runs.

For selected carrier coordinate:

\[
x_j
\]

choose small:

\[
\delta
\]

Then:

\[
bit=0 \Rightarrow x'_j=x_j-\delta
\]

\[
bit=1 \Rightarrow x'_j=x_j+\delta
\]

Carrier positions must be selected deterministically/pseudorandomly.

Do not alter every glyph.

Use:

- redundancy;
- ECC;
- framing;
- checksum;
- document binding;
- confidence scoring.

---

# 37. Watermark Extraction

For each candidate carrier:

\[
d_j=x_{observed,j}-x_{reference,j}
\]

Using threshold \(\tau\):

```text
d_j > +tau  -> 1
d_j < -tau  -> 0
otherwise   -> erasure
```

Then:

```text
observations
   |
   v
bit/erasure stream
   |
   v
ECC recovery
   |
   v
frame validation
   |
   v
reservation identifier
```

The extractor must report uncertainty rather than invent a result.

---

# 38. Forensic Workflow

Input:

```text
suspected leaked file
```

Process:

```text
hash leaked file
      |
      v
extract watermark
      |
      v
decode frame
      |
      v
validate checksum/document binding
      |
      v
recover reservation_txid
      |
      v
query ledger
      |
      v
retrieve reservation
      |
      v
retrieve finalized receipt
      |
      v
retrieve registered ML-DSA public key
      |
      v
independently verify ML-DSA signature
      |
      v
produce attribution evidence
```

---

# 39. Forensic Result States

The forensic tool must distinguish:

```text
VERIFIED_ATTRIBUTION
LOW_CONFIDENCE
WATERMARK_NOT_FOUND
WATERMARK_CORRUPTED
UNKNOWN_RESERVATION
RECEIPT_NOT_FOUND
INVALID_SIGNATURE
DOCUMENT_BINDING_FAILURE
INVALID_CERTIFICATE
UNSUPPORTED_DOCUMENT
```

Do not collapse all failures into a single generic error.

---

# 40. Evidence Bundle

Suggested final structure:

```text
evidence/
|
+-- leaked-file.sha256
+-- extraction.json
+-- observations.csv
+-- reservation.json
+-- receipt.cbor
+-- receipt.json
+-- recipient-certificate.cbor
+-- receipt-signature.bin
+-- cryptographic-verification.json
+-- ledger-reference.json
+-- forensic-report.md
```

PDF report generation may be added later.

---

# 41. Threat Model Summary

Primary assets:

- plaintext documents;
- recipient private keys;
- CEK;
- KEKs;
- ML-KEM shared secrets;
- ledger receipt integrity;
- recipient identity mappings;
- forensic watermark integrity.

Potential attackers:

- unauthorized user possessing `.cadp`;
- authorized recipient attempting attribution avoidance;
- attacker modifying encrypted package;
- attacker submitting fake receipts;
- attacker replaying valid transactions;
- attacker corrupting watermarks;
- attacker supplying malformed files.

Out of scope for initial prototype:

- compromised kernel/root attacker;
- hardware implants;
- physical RAM extraction;
- malicious CPU/firmware;
- sophisticated collusion attacks;
- guaranteed watermark survival after arbitrary rasterization/OCR.

---

# 42. Required Test Categories

The test system must eventually include:

## Cryptography

- valid ML-KEM recovery;
- wrong ML-KEM key;
- modified ML-KEM ciphertext;
- valid ML-DSA signature;
- wrong ML-DSA key;
- modified receipt;
- corrupted signature;
- AES-GCM tag failure;
- wrong nonce;
- altered AAD;
- HKDF context mismatch.

## PKI

- valid certificate;
- invalid signature;
- expired certificate;
- future certificate;
- revoked key;
- wrong CA;
- malformed certificate;
- unsupported algorithm;
- rotation.

## Package

- valid package;
- empty package;
- random input;
- truncated package;
- malformed CBOR;
- duplicate recipients;
- missing manifest;
- modified content;
- invalid publisher signature;
- unsupported version;
- excessive lengths.

## Ledger

- valid reservation;
- duplicate reservation/replay behavior;
- finalization without reservation;
- duplicate finalization;
- wrong recipient;
- wrong document;
- revoked signing key;
- invalid signature;
- concurrent finalization;
- unavailable peer/orderer;
- restart after reservation.

## Atomicity

- no plaintext temp file;
- no plaintext logging;
- no UI plaintext exposure;
- decryption crash;
- watermark crash;
- signing failure;
- ledger finalization failure;
- commit timeout;
- output permission failure.

## Watermark

- original;
- rename;
- copy;
- metadata modification;
- PDF optimization;
- page extraction;
- partial carrier corruption;
- ECC correction;
- excessive corruption;
- print-to-PDF;
- rasterization;
- OCR where relevant.

## Forensics

- Alice attribution;
- Bob attribution;
- unknown document;
- missing watermark;
- corrupted watermark;
- invalid receipt signature;
- unknown reservation;
- document mismatch;
- low confidence.

---

# 43. Final Acceptance Scenario

The finished project must demonstrate:

1. Create offline CA.
2. Create Alice.
3. Create Bob.
4. Create Charlie.
5. Issue their PQ certificates.
6. Register them on the local permissioned ledger.
7. Register a document.
8. Encrypt the document once.
9. Authorize Alice, Bob, and Charlie.
10. Produce one `.cadp`.
11. Alice opens the package.
12. Alice creates ledger reservation.
13. Alice recovers CEK using ML-KEM.
14. Content decrypts in trusted memory.
15. Alice-specific watermark is applied.
16. Watermark is self-verified.
17. Receipt is created.
18. Alice signs receipt using ML-DSA.
19. Ledger verifies ML-DSA signature.
20. Ledger commits receipt.
21. Only then is Alice's output file written.
22. Bob performs the same sequence.
23. Alice and Bob receive visually equivalent content.
24. Their forensic identifiers differ.
25. Alice's file is copied and renamed `leaked.pdf`.
26. Forensic tool extracts the watermark.
27. Reservation is located.
28. Receipt is retrieved.
29. ML-DSA signature is independently verified.
30. Attribution identifies Alice's cryptographic identity.
31. Repeat successfully for Bob.
32. Unauthorized user fails to decrypt.
33. Modified package fails validation.
34. Modified ciphertext fails AES-GCM authentication.
35. Modified receipt fails ML-DSA validation.
36. Revoked recipient is rejected.
37. Duplicate finalization is rejected.
38. Malformed packages do not crash the parser.
39. Forced ledger-finalization failure creates no output plaintext file.
40. System works offline using documented deployment procedure.

---

# 44. Definition of Done

The project is complete only when:

- implementation exists;
- full tests execute locally;
- known failures are documented;
- major parsers have adversarial testing;
- security invariants are tested;
- documentation matches implementation;
- offline deployment is reproducible;
- final acceptance demonstration succeeds;
- no CRITICAL unresolved security finding remains;
- all known limitations are disclosed.
