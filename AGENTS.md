# AGENTS.md

## 1. Project

Project title:

**Cryptographic Attribution and Immutable Decryption Provenance for Multi-Recipient Encrypted Document Distribution**

This repository contains a security-sensitive final-year engineering project.

The system must:

- encrypt a document once;
- authorize multiple recipients independently;
- use NIST-standardized post-quantum cryptography for the application security plane;
- bind document decryption to a recipient cryptographic identity;
- record immutable decryption provenance;
- generate a recipient-distinct invisible forensic watermark;
- prevent intentional persistence of unwatermarked plaintext;
- support forensic attribution of leaked document instances;
- work entirely inside an offline/air-gapped deployment.

---

# 2. Authoritative Documentation

Before making architectural, cryptographic, security-sensitive, or phase-level changes, read:

- `docs/PROJECT_SPEC.md`
- `docs/IMPLEMENTATION_PLAN.md`
- `docs/THREAT_MODEL.md`
- `docs/CRYPTO_PROTOCOL.md`
- `docs/TEST_MATRIX.md`
- `docs/KNOWN_LIMITATIONS.md`
- `docs/DECISIONS.md`

`docs/PROJECT_SPEC.md` defines what the system must do.

`docs/IMPLEMENTATION_PLAN.md` defines the order in which it must be built.

If code and documentation disagree, do not silently choose one. Identify the inconsistency and update the appropriate source.

---

# 3. Development Philosophy

Keep the project:

- simple;
- minimal;
- auditable;
- modular;
- well documented;
- well tested;
- easy for an engineering student to understand.

Prefer straightforward code over clever abstractions.

Do not introduce dependencies, frameworks, services, databases, containers, or abstractions unless they solve a demonstrated requirement.

Prefer:

- CLI before GUI;
- explicit state machines over implicit behavior;
- typed structures over loosely structured data;
- deterministic formats over ambiguous formats;
- existing cryptographic libraries over custom implementations;
- small functions over complex functions;
- clear module boundaries over generic architecture.

---

# 4. Mandatory Phase Control

Development MUST happen phase-by-phase.

Work only on the currently approved phase.

At the end of every phase:

1. stop implementation;
2. run all relevant tests;
3. run formatting/linting/static checks;
4. perform an independent security review;
5. update documentation;
6. update `docs/TEST_MATRIX.md`;
7. update `docs/KNOWN_LIMITATIONS.md`;
8. list files created or modified;
9. list bugs found and fixed;
10. provide exact manual verification commands;
11. provide exact demonstration steps;
12. describe the proposed next phase;
13. STOP and wait for explicit user approval.

Never automatically begin the next phase.

---

# 5. Cryptographic Requirements

The application security plane must use:

- **ML-KEM-768** — FIPS 203
- **ML-DSA-65** — FIPS 204
- **AES-256-GCM**
- **HKDF-SHA-256**
- **SHA-256**
- operating-system CSPRNG

Never replace application-level ML-KEM or ML-DSA with:

- RSA
- ECDH
- ECDSA
- Ed25519
- X25519

unless the project specification is explicitly changed.

Do not implement cryptographic primitives manually.

Do not implement:

- custom ML-KEM;
- custom ML-DSA;
- custom AES;
- custom SHA;
- custom random-number generator;
- novel encryption schemes.

Use maintained cryptographic libraries.

Preferred PQC implementation:

`liboqs`

---

# 6. Cryptographic Scope Honesty

Hyperledger Fabric may internally depend on classical cryptographic mechanisms.

Therefore distinguish clearly between:

## Application security plane

Post-quantum requirements apply here.

Includes:

- recipient key establishment;
- CEK recovery;
- recipient receipt signatures;
- application-level PQ certificates.

## Ledger infrastructure plane

May use Fabric's native cryptography where unavoidable.

Never claim that an unmodified Hyperledger Fabric deployment is entirely post-quantum.

Document this limitation in:

`docs/CRYPTOGRAPHIC_SCOPE.md`

---

# 7. Key Separation

Every recipient must have two independent key pairs:

## KEM

ML-KEM-768:

- public key
- private key

## Signature

ML-DSA-65:

- public key
- private key

Never reuse key material between KEM and signature operations.

Never derive one key pair from the other.

Use distinct identifiers:

- `kem_key_id`
- `signature_key_id`

---

# 8. Code Quality

Code must be:

- readable;
- minimal;
- documented;
- explicitly error-handled;
- security-reviewable.

Use comments to explain:

- security assumptions;
- invariants;
- why sensitive operations are required;
- secret lifetimes;
- failure behavior;
- trust-boundary decisions.

Do not add comments that merely repeat obvious syntax.

Example good comment:

```rust
// Plaintext must never leave the trusted core before watermarking.
// The UI must only receive the final watermarked artifact.
```

Example bad comment:

```rust
// Create variable.
```

Avoid unnecessary generic abstractions.

Avoid unnecessary `unsafe` Rust.

Do not use `.unwrap()` or equivalent uncontrolled panic behavior on attacker-controlled input.

---

# 9. Secret Handling

Never log:

- plaintext document contents;
- CEK;
- KEK;
- ML-KEM shared secret;
- passwords;
- private keys;
- unlocked key material;
- sensitive internal buffers.

Sensitive material must be zeroized where practical.

Investigate and use appropriate Linux memory-hardening controls in the trusted client, including where practical:

- `mlock`
- `mlockall`
- `MADV_DONTDUMP`
- `PR_SET_DUMPABLE`
- disabled core dumps

Document limitations honestly.

Do not claim that application-level memory protections defend against a fully compromised kernel/root attacker.

---

# 10. Atomic Release Invariant

The most important application invariant is:

```text
OUTPUT_RELEASED implies RECEIPT_COMMITTED
```

Equivalent fail-closed requirement:

```text
NOT RECEIPT_COMMITTED implies NOT OUTPUT_RELEASED
```

Unwatermarked plaintext must never intentionally be persisted.

It must never intentionally be:

- written to disk;
- written to a temporary file;
- returned to the UI;
- copied to clipboard;
- emitted through logs;
- sent to an external helper program;
- serialized into debugging output.

If receipt finalization fails, no plaintext output file may be created.

---

# 11. Error Handling

Security-sensitive failures must fail closed.

Examples:

Certificate validation failure:

**REJECT**

Expired certificate:

**REJECT**

Revoked recipient:

**REJECT**

Invalid package signature:

**REJECT**

AES-GCM authentication failure:

**REJECT**

Unsupported package version:

**REJECT**

Watermark generation failure:

**REJECT**

Watermark self-verification failure:

**REJECT**

ML-DSA signing failure:

**REJECT**

Ledger finalization failure:

**DO NOT RELEASE OUTPUT**

Unknown ledger commit state:

**DO NOT RELEASE OUTPUT**

Never silently downgrade security.

---

# 12. Testing Rules

Testing is a primary deliverable.

Every security-sensitive feature should receive appropriate:

- positive tests;
- negative tests;
- malformed-input tests;
- boundary tests;
- tampering tests;
- state-machine tests;
- replay tests;
- failure-injection tests;
- integration tests;
- regression tests.

All significant tests must be recorded in:

`docs/TEST_MATRIX.md`

Use stable identifiers such as:

```text
PQC-KEM-001
PQC-SIG-001
PKI-CERT-001
PKG-TAMPER-001
MEM-ATOMIC-001
LEDGER-REPLAY-001
WM-PDF-001
FORENSIC-001
E2E-001
```

Never remove a valid security test merely because implementation cannot pass it.

Failed tests must be investigated and documented.

---

# 13. Fuzzing and Property Testing

Use fuzzing/property testing where useful.

Preferred Rust tools:

- `cargo test`
- `proptest`
- `cargo-fuzz`
- `cargo clippy`
- `cargo fmt`

Preferred Python tools:

- `pytest`
- `Hypothesis`
- `ruff`

Preferred Go tools:

- `go test`
- `go vet`
- native Go fuzz testing where appropriate

High-value fuzz targets:

- `.cadp` parser;
- PQ certificate parser;
- receipt decoder;
- watermark decoder;
- PDF watermark parser;
- chaincode input validation.

---

# 14. Never Fake Results

Never claim that:

- tests passed when they were not executed;
- commands succeeded when they were not executed;
- a feature is secure merely because it compiles;
- a watermark survives a transformation that was not tested;
- an implementation is post-quantum when classical cryptography exists in that security plane;
- a bug is fixed unless the fix has been verified.

Use explicit status language:

- IMPLEMENTED
- TESTED
- PARTIALLY TESTED
- PLANNED
- BLOCKED
- FAILED

---

# 15. Security Review

At the end of each implementation phase, review the code as if another engineer wrote it.

Review:

## Cryptography

- Are algorithms correct?
- Is key separation preserved?
- Is nonce handling correct?
- Is authenticated data correct?
- Is deterministic serialization used where required?

## Identity

- Can identity fields be spoofed?
- Is the correct public key used?
- Are revoked/expired credentials rejected?

## Parsing

- Can malformed input crash the process?
- Are input sizes bounded?
- Are duplicate/ambiguous fields rejected?

## Memory

- Where does plaintext exist?
- Where do CEK/KEK/shared secrets exist?
- Are buffers zeroized?

## Filesystem

- Are temporary plaintext files ever created?
- Can error paths leave plaintext?

## Ledger

- Can reservations be replayed?
- Can receipts be duplicated?
- Can one recipient finalize another recipient's reservation?

## State Machine

- Can steps execute out of order?
- Can plaintext be released before receipt finalization?

## Watermark

- What destroys the watermark?
- What produces false positives?
- What evidence is required for attribution?

## Logging

- Are secrets or plaintext accidentally logged?

## Concurrency

- Can simultaneous workflows bypass invariants?

Classify findings:

- CRITICAL
- HIGH
- MEDIUM
- LOW
- INFORMATIONAL

Fix legitimate issues and create regression tests.

---

# 16. Documentation

Documentation is part of the implementation.

Keep documentation synchronized with actual behavior.

Important documentation includes:

- architecture;
- threat model;
- cryptographic protocol;
- key management;
- package format;
- ledger protocol;
- watermark format;
- test matrix;
- attack surface;
- known limitations;
- offline deployment;
- user guide.

Never hide limitations.

---

# 17. Review Gate Response

At the completion of every phase, report:

## Phase N Complete

### 1. What Was Implemented

### 2. Files Created or Modified

### 3. Important Design Decisions

### 4. Tests Executed

Include:

- exact command;
- passed;
- failed;
- skipped.

### 5. Security Tests

List test IDs.

### 6. Security Review Findings

Use severity labels.

### 7. Bugs Found and Fixed

### 8. Known Limitations

### 9. Documentation Updated

### 10. Manual Verification Commands

### 11. Demonstration Procedure

### 12. Test Matrix Status

### 13. Proposed Next Phase

Then STOP.

Do not continue without explicit approval.

---

# 18. Current Source of Truth

The implementation sequence is defined in:

`docs/IMPLEMENTATION_PLAN.md`

The detailed technical requirements are defined in:

`docs/PROJECT_SPEC.md`

Follow those documents unless explicitly instructed otherwise.
