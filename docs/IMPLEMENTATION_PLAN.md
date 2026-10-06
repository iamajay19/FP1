# Implementation Plan

## Cryptographic Attribution and Immutable Decryption Provenance for Multi-Recipient Encrypted Document Distribution

---

# Development Strategy

The project will be implemented incrementally.

Do not build the entire project at once.

Each phase must:

1. implement one coherent capability;
2. include automated tests;
3. include appropriate negative/adversarial tests;
4. update documentation;
5. update `docs/TEST_MATRIX.md`;
6. perform a security review;
7. stop at a review gate.

No phase may begin until the previous phase is explicitly approved.

---

# Phase 0 — Repository and Specification Foundation

## Goal

Establish a complete technical specification and repository structure before implementing security-sensitive code.

## Tasks

Inspect development environment:

```text
operating system
CPU architecture
Rust version/toolchain
Cargo
Python
pip/venv
Go
Docker
Docker Compose
Git
C/C++ compiler
CMake
pkg-config
liboqs availability
```

Do not assume tools exist.

Create repository structure approximately:

```text
.
├── AGENTS.md
├── README.md
├── LICENSE
├── .gitignore
├── Makefile
├── docs/
│   ├── PROJECT_SPEC.md
│   ├── IMPLEMENTATION_PLAN.md
│   ├── ARCHITECTURE.md
│   ├── THREAT_MODEL.md
│   ├── CRYPTOGRAPHIC_SCOPE.md
│   ├── CRYPTO_PROTOCOL.md
│   ├── PACKAGE_FORMAT.md
│   ├── LEDGER_PROTOCOL.md
│   ├── WATERMARK_DESIGN.md
│   ├── SECURITY_ASSUMPTIONS.md
│   ├── ATTACK_SURFACE.md
│   ├── TEST_MATRIX.md
│   ├── KNOWN_LIMITATIONS.md
│   ├── DECISIONS.md
│   ├── OFFLINE_DEPLOYMENT.md
│   └── USER_GUIDE.md
├── pki/
├── publisher/
├── trusted-client/
├── ledger/
│   ├── network/
│   └── chaincode/
├── forensic/
├── tests/
│   ├── integration/
│   ├── adversarial/
│   └── fixtures/
└── scripts/
```

Create initial:

- architecture diagram using Mermaid;
- trust-boundary diagram;
- threat model;
- cryptographic scope;
- protocol notation;
- certificate schema;
- `.cadp` package specification;
- receipt schema;
- ledger state machine;
- initial watermark design;
- test matrix;
- decision log.

## Required Tests

Phase 0 mostly contains documentation/build validation.

At minimum verify:

- documentation files exist;
- repository structure matches documentation;
- Mermaid/code blocks are syntactically coherent where tooling is available;
- top-level commands fail clearly when implementation does not yet exist.

## Phase 0 Exit Criteria

- architecture documented;
- security claims accurately scoped;
- threat model written;
- initial test matrix created;
- no implementation beyond necessary scaffolding;
- unresolved design questions recorded.

STOP.

---

# Phase 1 — PQC Primitive Integration

## Goal

Prove that the development environment can correctly use standardized PQC implementations.

## Implement

Small Rust module for:

```text
ML-KEM-768 key generation
ML-KEM-768 encapsulation
ML-KEM-768 decapsulation

ML-DSA-65 key generation
ML-DSA-65 signing
ML-DSA-65 verification
```

Use liboqs.

Keep wrapper code minimal.

## Tests

Create at minimum:

```text
PQC-KEM-001 valid encapsulation/decapsulation
PQC-KEM-002 wrong private key
PQC-KEM-003 modified KEM ciphertext
PQC-KEM-004 truncated KEM ciphertext

PQC-SIG-001 valid sign/verify
PQC-SIG-002 wrong public key
PQC-SIG-003 modified message
PQC-SIG-004 modified signature
PQC-SIG-005 truncated signature
```

Use property testing where useful.

## Exit Criteria

- wrappers compile;
- tests pass;
- exact liboqs version documented;
- API does not expose unnecessary operations;
- no custom cryptographic primitive exists.

STOP.

---

# Phase 2 — Private-Key Storage and Offline PQ PKI

## Goal

Create the offline identity infrastructure.

## Implement

### CA tooling

```text
create-ca
```

### Recipient creation

```text
create-recipient
```

Generate:

```text
ML-KEM-768 key pair
ML-DSA-65 key pair
```

### Certificate issuance

```text
issue-certificate
```

### Certificate verification

```text
verify-certificate
```

### Revocation

```text
revoke-key
```

### Key-bundle storage

Protect private keys using:

```text
Argon2id
       |
       v
AES-256-GCM
```

Store only encrypted private-key material.

Use deterministic certificate encoding.

Sign certificates with ML-DSA.

## Tests

At minimum:

```text
PKI-CERT-001 valid certificate
PKI-CERT-002 modified certificate
PKI-CERT-003 invalid CA signature
PKI-CERT-004 wrong CA
PKI-CERT-005 expired certificate
PKI-CERT-006 future certificate
PKI-CERT-007 malformed certificate
PKI-CERT-008 unsupported algorithm

PKI-KEY-001 correct password
PKI-KEY-002 wrong password
PKI-KEY-003 corrupted encrypted key
PKI-KEY-004 key separation
PKI-KEY-005 secret zeroization review

PKI-REV-001 revoked signing key
PKI-REV-002 revoked KEM key
PKI-REV-003 modified revocation list
```

## Exit Criteria

- recipient identity can be created offline;
- private keys are encrypted at rest;
- certificate verification fails closed;
- revocation works;
- documentation updated.

STOP.

---

# Phase 3 — Single-Copy Multi-Recipient Encryption

## Goal

Encrypt one document once and allow multiple authorized recipients to recover the same CEK independently.

## Implement

Publisher CLI.

Suggested conceptual command:

```text
publisher encrypt \
    --input secret.pdf \
    --recipient alice.cert \
    --recipient bob.cert \
    --recipient charlie.cert \
    --output secret.cadp
```

Implement:

1. document ID generation;
2. CEK generation;
3. AES-256-GCM content encryption;
4. ML-KEM encapsulation per recipient;
5. HKDF KEK derivation;
6. AES-GCM CEK wrapping;
7. deterministic manifest;
8. publisher signature;
9. package serialization.

## Tests

```text
ENC-001 Alice recovers CEK
ENC-002 Bob recovers same CEK
ENC-003 Charlie recovers same CEK
ENC-004 unauthorized key fails
ENC-005 altered wrapped CEK fails
ENC-006 wrong recipient AAD fails
ENC-007 wrong document ID fails
ENC-008 altered content nonce fails
ENC-009 altered ciphertext fails
ENC-010 altered authentication tag fails
ENC-011 duplicate recipients rejected
```

Verify encrypted content exists only once.

## Exit Criteria

- one ciphertext;
- multiple independent recipient entries;
- authorized recipients recover same CEK;
- unauthorized recipient cannot recover CEK;
- cryptographic failures are controlled.

STOP.

---

# Phase 4 — Hardened `.cadp` Parser and Verifier

## Goal

Treat encrypted packages as attacker-controlled input.

## Implement

Parser with:

- schema-version validation;
- field validation;
- bounded sizes;
- duplicate detection;
- recipient-entry validation;
- deterministic manifest parsing;
- issuer-signature verification;
- encrypted-content hash verification.

Never panic on malformed input.

## Tests

```text
PKG-PARSE-001 valid package
PKG-PARSE-002 empty file
PKG-PARSE-003 random bytes
PKG-PARSE-004 truncated package
PKG-PARSE-005 malformed CBOR
PKG-PARSE-006 missing manifest
PKG-PARSE-007 missing encrypted content
PKG-PARSE-008 duplicate recipient
PKG-PARSE-009 unsupported schema
PKG-PARSE-010 excessive declared length
PKG-PARSE-011 unknown algorithm
PKG-TAMPER-001 modified manifest
PKG-TAMPER-002 modified ciphertext
PKG-TAMPER-003 modified recipient entry
PKG-TAMPER-004 invalid publisher signature
```

Add:

- property tests;
- fuzz target.

## Exit Criteria

- malformed packages do not panic;
- resource limits exist;
- tampering is rejected;
- fuzz harness exists.

STOP.

---

# Phase 5 — Trusted Decryption Core

## Goal

Implement secure recipient-side decryption without a GUI.

## Implement

Rust CLI/core only.

Workflow:

```text
load package
    |
verify package
    |
validate recipient
    |
unlock key bundle
    |
ML-KEM decapsulation
    |
derive KEK
    |
recover CEK
    |
AES-GCM decrypt in memory
```

At this phase use mock implementations for:

- ledger;
- watermark.

Implement explicit state machine.

Implement secret-zeroization strategy.

Investigate Linux memory hardening.

## API Rule

Do not expose:

```text
get_plaintext
decrypt_raw
get_cek
export_unwatermarked
```

## Tests

```text
CLIENT-STATE-001 legal transition sequence
CLIENT-STATE-002 invalid transition rejected
CLIENT-KEY-001 wrong recipient key
CLIENT-KEY-002 wrong password
CLIENT-DEC-001 valid decryption
CLIENT-DEC-002 altered ciphertext
CLIENT-DEC-003 invalid CEK wrap
MEM-001 no public raw-plaintext API
MEM-002 sensitive buffer cleanup
MEM-003 logging review
```

## Exit Criteria

- trusted core can decrypt correctly;
- plaintext remains internal;
- state machine exists;
- no GUI yet;
- memory-security limitations documented.

STOP.

---

# Phase 6 — Hyperledger Fabric Network

## Goal

Create the local immutable provenance layer.

## Implement

Dockerized local Fabric network.

Organizations:

```text
IssuerOrg
AuditorOrg
```

Ordering service.

Channel.

Go chaincode.

Implement:

```text
RegisterRecipient
RevokeRecipientKey
RegisterDocument
ReserveDecryption
FinalizeReceipt
GetReservation
GetReceipt
```

Optional later:

```text
GetReceiptsForDocument
GetReceiptsForRecipient
```

## Receipt Signature Verification

`FinalizeReceipt` must verify ML-DSA-65 before storing finalized receipt state.

If native liboqs use in conventional chaincode is fragile, investigate Fabric external chaincode/service deployment.

Do not remove ML-DSA verification merely to simplify development.

## Tests

```text
LEDGER-ID-001 register recipient
LEDGER-ID-002 duplicate identity
LEDGER-ID-003 revoked key

LEDGER-DOC-001 register document
LEDGER-DOC-002 duplicate document

LEDGER-RES-001 valid reservation
LEDGER-RES-002 unknown recipient
LEDGER-RES-003 revoked recipient
LEDGER-RES-004 unknown document

LEDGER-FIN-001 valid finalization
LEDGER-FIN-002 no reservation
LEDGER-FIN-003 wrong recipient
LEDGER-FIN-004 wrong document
LEDGER-FIN-005 wrong key ID
LEDGER-FIN-006 invalid ML-DSA signature
LEDGER-FIN-007 altered receipt
LEDGER-REPLAY-001 duplicate finalization
LEDGER-CONC-001 concurrent finalization
```

## Exit Criteria

- network runs offline locally;
- ledger state persists;
- ML-DSA receipt validation works;
- duplicate finalization is prevented.

STOP.

---

# Phase 7 — Text Watermark Prototype

## Goal

Validate watermark framing, redundancy, extraction, and ledger-ID association with a simple medium.

## Implement

Watermark frame:

```text
MAGIC
VERSION
RESERVATION_ID
DOCUMENT_BINDING
CHECKSUM
ECC
```

Implement:

- binary encoding;
- zero-width carrier mapping;
- repeated embedding;
- extraction;
- ECC/error correction;
- checksum verification.

## Tests

```text
WM-TEXT-001 clean extraction
WM-TEXT-002 copied file
WM-TEXT-003 renamed file
WM-TEXT-004 single-bit corruption
WM-TEXT-005 recoverable corruption
WM-TEXT-006 unrecoverable corruption
WM-TEXT-007 normalization
WM-TEXT-008 text sanitization
WM-TEXT-009 false positive
```

Expected:

Normalization/sanitization may destroy the text watermark.

Record this honestly.

## Exit Criteria

- framing protocol proven;
- extraction confidence exists;
- limitations documented.

STOP.

---

# Phase 8 — Atomic End-to-End Integration

## Goal

Connect ledger reservation, decryption, watermarking, receipt signing, receipt commit, and output release.

## Required Order

```text
PACKAGE_VALIDATED
        |
RECIPIENT_VALIDATED
        |
RESERVATION_COMMITTED
        |
KEYS_UNLOCKED
        |
CEK_RECOVERED
        |
CONTENT_DECRYPTED_IN_MEMORY
        |
WATERMARK_APPLIED
        |
WATERMARK_SELF_VERIFIED
        |
RECEIPT_SIGNED
        |
RECEIPT_COMMITTED
        |
OUTPUT_RELEASED
        |
CLEANUP_COMPLETE
```

## Mandatory Invariant

```text
OUTPUT_RELEASED implies RECEIPT_COMMITTED
```

## Failure Injection

Provide mechanisms for intentionally failing:

- watermark step;
- receipt signing;
- ledger connection;
- ledger finalization;
- commit confirmation;
- output write.

## Mandatory Test

```text
MEM-ATOMIC-001
```

Scenario:

1. valid package;
2. reservation succeeds;
3. decryption succeeds;
4. watermark succeeds;
5. force `FinalizeReceipt` to fail.

Expected:

```text
NO plaintext output file exists.
```

## Other Tests

```text
MEM-ATOMIC-002 signing failure
MEM-ATOMIC-003 watermark failure
MEM-ATOMIC-004 commit timeout
MEM-ATOMIC-005 output permission failure
MEM-ATOMIC-006 crash simulation after decrypt
MEM-ATOMIC-007 no plaintext temp files
MEM-ATOMIC-008 no plaintext logs
```

## Exit Criteria

Atomic release invariant demonstrated.

STOP.

---

# Phase 9 — PDF Coordinate Watermark

## Goal

Implement the primary forensic PDF watermark.

## Implement

1. PDF parsing.
2. Eligible carrier identification.
3. Carrier-selection strategy.
4. Configurable displacement `delta`.
5. Bit encoding.
6. redundancy.
7. ECC.
8. document binding.
9. extraction.
10. erasure handling.
11. confidence scoring.

Do not modify every glyph.

## Tests

```text
WM-PDF-001 clean file
WM-PDF-002 copy
WM-PDF-003 rename
WM-PDF-004 metadata removal
WM-PDF-005 incremental save
WM-PDF-006 PDF optimization
WM-PDF-007 page extraction
WM-PDF-008 partial text edits
WM-PDF-009 carrier corruption
WM-PDF-010 recoverable bit errors
WM-PDF-011 excessive corruption
WM-PDF-012 print-to-PDF
WM-PDF-013 rasterization
WM-PDF-014 OCR recovery experiment
WM-PDF-015 false positive test
```

Do not claim survival for any operation that fails.

## Exit Criteria

- visually acceptable PDF output;
- extraction works on supported files;
- robustness measurements documented;
- limitations recorded.

STOP.

---

# Phase 10 — Forensic Extractor

## Goal

Recover attribution evidence from a leaked file.

## Implement

CLI workflow:

```text
forensic analyze leaked.pdf
```

Steps:

1. hash leaked file;
2. detect supported watermark;
3. extract carrier observations;
4. recover bitstream;
5. apply ECC;
6. validate frame;
7. recover reservation ID;
8. validate document binding;
9. query ledger;
10. retrieve reservation;
11. retrieve finalized receipt;
12. obtain recipient ML-DSA public key;
13. independently verify receipt signature;
14. calculate confidence;
15. create evidence bundle.

## Result States

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

## Tests

```text
FORENSIC-001 Alice leak
FORENSIC-002 Bob leak
FORENSIC-003 Charlie leak
FORENSIC-004 no watermark
FORENSIC-005 corrupt watermark
FORENSIC-006 unknown reservation
FORENSIC-007 reservation without receipt
FORENSIC-008 altered receipt
FORENSIC-009 invalid ML-DSA signature
FORENSIC-010 document mismatch
FORENSIC-011 low-confidence extraction
FORENSIC-012 false-positive corpus
```

## Exit Criteria

A leaked supported file can produce a reproducible evidence bundle.

STOP.

---

# Phase 11 — GUI

## Goal

Add usability without moving security-sensitive logic out of the trusted core.

## Recipient UI

Suggested screens:

```text
Identity
Open Package
Package Verification
Decrypt & Attribute
Result
```

UI must never receive unwatermarked plaintext.

Security-sensitive functions remain in Rust core.

## Forensic UI

Suggested screens:

```text
Select Evidence
Extraction
Ledger Evidence
Signature Verification
Result
Export
```

Keep GUI minimal.

## Tests

Include:

- UI cannot request raw plaintext;
- invalid package displays safe error;
- ledger failure prevents output;
- cancellation does not release plaintext;
- repeated operations behave safely.

## Exit Criteria

UI is a thin layer over proven core logic.

STOP.

---

# Phase 12 — Adversarial Security Testing

## Goal

Systematically attack the implementation.

## Cryptographic Tests

Test:

- wrong ML-KEM key;
- malformed KEM ciphertext;
- wrong CEK;
- corrupted wrapped CEK;
- modified nonce;
- modified ciphertext;
- modified GCM tag;
- modified AAD;
- invalid ML-DSA signature;
- altered receipt;
- wrong signature public key;
- certificate corruption;
- key rotation;
- revocation.

## Package Tests

Test:

- random bytes;
- malformed CBOR;
- huge fields;
- duplicate fields;
- missing entries;
- unknown algorithms;
- unsupported versions;
- mutation corpus.

## Ledger Tests

Test:

- replay;
- duplicate finalization;
- nonexistent reservation;
- wrong identity;
- wrong document;
- revoked key;
- invalid signature;
- network outage;
- peer outage;
- orderer outage;
- restart;
- concurrent transactions.

## Atomicity Tests

Test:

- crash after decryption;
- crash during watermark;
- crash after watermark;
- signing failure;
- ledger finalization failure;
- commit timeout;
- output write failure;
- disk-full simulation where feasible;
- plaintext log inspection;
- filesystem temporary-file inspection.

## Fuzzing

Run fuzzers against:

- `.cadp` parser;
- certificate parser;
- receipt parser;
- watermark decoder;
- PDF carrier parser;
- chaincode input.

## Deliverable

Create:

```text
docs/SECURITY_TEST_REPORT.md
```

Classify findings:

```text
CRITICAL
HIGH
MEDIUM
LOW
INFORMATIONAL
```

Create regression tests for resolved issues.

STOP.

---

# Phase 13 — Offline Deployment

## Goal

Prove that the project works in a fully disconnected environment.

## Implement

Create offline dependency strategy.

Document:

### Rust

- dependency locking;
- vendoring where practical;
- toolchain requirements.

### Python

- local wheelhouse;
- pinned requirements.

### Docker

Export required images:

```text
docker save
```

Import offline:

```text
docker load
```

### Fabric

Document all required images/binaries.

### liboqs

Document installation/build procedure.

## Test

Prepare clean test environment with network disabled.

Perform:

1. dependency installation;
2. build;
3. Fabric start;
4. CA creation;
5. recipient creation;
6. encryption;
7. decryption;
8. forensic extraction.

No runtime network access may be required.

## Exit Criteria

Offline deployment is reproducible.

STOP.

---

# Phase 14 — Final End-to-End Validation

## Goal

Execute the complete acceptance demonstration.

## Scenario

Create:

```text
Alice
Bob
Charlie
```

Issue PQ certificates.

Register them.

Register source document.

Encrypt:

```text
secret.pdf
```

once.

Create:

```text
secret.cadp
```

authorized for all three.

---

## Alice Workflow

Alice:

1. opens package;
2. verifies package;
3. creates ledger reservation;
4. recovers CEK;
5. decrypts in memory;
6. receives Alice-specific watermark;
7. self-verifies watermark;
8. creates receipt;
9. signs receipt with ML-DSA;
10. ledger verifies signature;
11. receipt commits;
12. output appears.

Expected:

```text
alice-secret.pdf
```

---

## Bob Workflow

Repeat.

Expected:

```text
bob-secret.pdf
```

Visually equivalent content.

Different forensic watermark.

---

## Leak Attribution

Copy:

```text
alice-secret.pdf
```

to:

```text
leaked.pdf
```

Forensic tool must:

1. extract watermark;
2. recover Alice reservation;
3. retrieve receipt;
4. independently verify ML-DSA;
5. validate document binding;
6. identify Alice's cryptographic identity;
7. create evidence bundle.

Repeat with Bob.

---

## Mandatory Negative Tests

Execute:

- unauthorized recipient;
- modified package;
- modified ciphertext;
- modified GCM tag;
- modified manifest;
- invalid publisher signature;
- expired certificate;
- revoked recipient;
- wrong signing key;
- invalid receipt signature;
- duplicate finalization;
- malformed package;
- ledger failure;
- forced receipt-finalization failure.

Mandatory expected result for forced finalization failure:

```text
NO plaintext output file.
```

## Exit Criteria

All mandatory acceptance tests execute.

Any failures must be documented.

STOP.

---

# Phase 15 — Final Review and Cleanup

## Goal

Finish the project without unnecessary feature expansion.

## Tasks

Review entire repository.

Remove:

- dead code;
- debug code;
- unused dependencies;
- unnecessary abstractions;
- temporary scripts;
- unsafe test shortcuts.

Improve:

- naming;
- comments;
- error messages;
- documentation;
- build instructions.

Run:

```text
formatters
linters
static checks
unit tests
property tests
integration tests
security tests
end-to-end tests
selected fuzzing
```

Audit logs.

Audit filesystem behavior.

Audit secret handling.

Audit documentation against implementation.

Update:

```text
README.md
docs/KNOWN_LIMITATIONS.md
docs/TEST_MATRIX.md
docs/SECURITY_TEST_REPORT.md
docs/OFFLINE_DEPLOYMENT.md
docs/USER_GUIDE.md
```

Generate final repository tree.

Prepare final demonstration instructions.

---

# Final Definition of Done

The project is complete only when:

- all required components exist;
- repository is documented;
- code is commented appropriately;
- tests exist for all critical behavior;
- test suite runs locally;
- cryptographic negative cases are tested;
- malformed input is handled safely;
- ledger replay/finalization attacks are tested;
- atomic output-release invariant is tested;
- forensic attribution works;
- limitations are documented;
- offline deployment works;
- no unresolved CRITICAL security issue remains;
- final end-to-end demonstration succeeds.
