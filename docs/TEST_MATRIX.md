# Test Matrix

Phase 0 evidence is recorded in [PHASE_0_REVIEW.md](PHASE_0_REVIEW.md).
TESTED means the named Phase 0 check ran; it does not validate runtime cryptography.
All later tests remain PLANNED, not passed or silently removed. The implementation
plan supplies the detailed expected outcomes for its mandatory IDs. Additional cases
close design-review gaps. Phase is the intended implementation/validation phase.

## Phase 0 validation

| ID | Phase | Test / expected outcome | Status | Evidence |
|---|---|---|---|---|
| P0-DOC-001 | 0 | Required substantive documents exist | TESTED | make check; PHASE_0_VALIDATION.txt |
| P0-STRUCT-001 | 0 | Documented directories are scaffold-only | TESTED | make check; PHASE_0_VALIDATION.txt |
| P0-LINK-001 | 0 | Local Markdown links resolve | TESTED | make check; PHASE_0_VALIDATION.txt |
| P0-SYNTAX-001 | 0 | Balanced fences and shell/Python example syntax | TESTED | make check; PHASE_0_VALIDATION.txt |
| P0-CLI-001 | 0 | Unavailable runtime targets exit nonzero and create no files | TESTED | make check; PHASE_0_VALIDATION.txt |
| P0-STATIC-001 | 0 | Python AST and basic whitespace/format checks | TESTED | make check; PHASE_0_VALIDATION.txt |
| P0-MATRIX-001 | 0 | Unique matrix IDs cover all explicit plan IDs | TESTED | make check; PHASE_0_VALIDATION.txt |
| P0-MERMAID-001 | 0 | Mermaid renderer compiles diagrams if installed | SKIPPED | mmdc unavailable; manual diagram review only |
| P0-REVIEW-001 | 0 | Separate adversarial design review against all AGENTS categories | TESTED | Separate manual design review; PHASE_0_REVIEW.md |

## Runtime security tests

Each scenario requires positive/negative assertions appropriate to its implementing
phase, with exact evidence added when executed. For PQC-KEM-002/003, correctly sized
invalid inputs may yield implicit-rejection secrets: assert secret mismatch, then
CEK-wrap authentication failure when wrapping exists in Phase 3. Do not require an
ML-KEM error that the library API need not return. No mock can satisfy a real-ledger
atomic release or final acceptance test.

| ID | Phase | Scenario / required assertion | Status | Evidence |
|---|---|---|---|---|
| ML-KEM-768 | 1 | key generation | PLANNED | Not implemented or executed |
| PQC-KEM-001 | 1 | valid encapsulation/decapsulation | PLANNED | Not implemented or executed |
| PQC-KEM-002 | 1 | wrong private key | PLANNED | Not implemented or executed |
| PQC-KEM-003 | 1 | modified KEM ciphertext | PLANNED | Not implemented or executed |
| PQC-KEM-004 | 1 | truncated KEM ciphertext | PLANNED | Not implemented or executed |
| PQC-SIG-001 | 1 | valid sign/verify | PLANNED | Not implemented or executed |
| PQC-SIG-002 | 1 | wrong public key | PLANNED | Not implemented or executed |
| PQC-SIG-003 | 1 | modified message | PLANNED | Not implemented or executed |
| PQC-SIG-004 | 1 | modified signature | PLANNED | Not implemented or executed |
| PQC-SIG-005 | 1 | truncated signature | PLANNED | Not implemented or executed |
| PKI-CERT-001 | 2 | valid certificate | PLANNED | Not implemented or executed |
| PKI-CERT-002 | 2 | modified certificate | PLANNED | Not implemented or executed |
| PKI-CERT-003 | 2 | invalid CA signature | PLANNED | Not implemented or executed |
| PKI-CERT-004 | 2 | wrong CA | PLANNED | Not implemented or executed |
| PKI-CERT-005 | 2 | expired certificate | PLANNED | Not implemented or executed |
| PKI-CERT-006 | 2 | future certificate | PLANNED | Not implemented or executed |
| PKI-CERT-007 | 2 | malformed certificate | PLANNED | Not implemented or executed |
| PKI-CERT-008 | 2 | unsupported algorithm | PLANNED | Not implemented or executed |
| PKI-KEY-001 | 2 | correct password | PLANNED | Not implemented or executed |
| PKI-KEY-002 | 2 | wrong password | PLANNED | Not implemented or executed |
| PKI-KEY-003 | 2 | corrupted encrypted key | PLANNED | Not implemented or executed |
| PKI-KEY-004 | 2 | key separation | PLANNED | Not implemented or executed |
| PKI-KEY-005 | 2 | secret zeroization review | PLANNED | Not implemented or executed |
| PKI-REV-001 | 2 | revoked signing key | PLANNED | Not implemented or executed |
| PKI-REV-002 | 2 | revoked KEM key | PLANNED | Not implemented or executed |
| PKI-REV-003 | 2 | modified revocation list | PLANNED | Not implemented or executed |
| ENC-001 | 3 | Alice recovers CEK | PLANNED | Not implemented or executed |
| ENC-002 | 3 | Bob recovers same CEK | PLANNED | Not implemented or executed |
| ENC-003 | 3 | Charlie recovers same CEK | PLANNED | Not implemented or executed |
| ENC-004 | 3 | unauthorized key fails | PLANNED | Not implemented or executed |
| ENC-005 | 3 | altered wrapped CEK fails | PLANNED | Not implemented or executed |
| ENC-006 | 3 | wrong recipient AAD fails | PLANNED | Not implemented or executed |
| ENC-007 | 3 | wrong document ID fails | PLANNED | Not implemented or executed |
| ENC-008 | 3 | altered content nonce fails | PLANNED | Not implemented or executed |
| ENC-009 | 3 | altered ciphertext fails | PLANNED | Not implemented or executed |
| ENC-010 | 3 | altered authentication tag fails | PLANNED | Not implemented or executed |
| ENC-011 | 3 | duplicate recipients rejected | PLANNED | Not implemented or executed |
| PKG-PARSE-001 | 4 | valid package | PLANNED | Not implemented or executed |
| PKG-PARSE-002 | 4 | empty file | PLANNED | Not implemented or executed |
| PKG-PARSE-003 | 4 | random bytes | PLANNED | Not implemented or executed |
| PKG-PARSE-004 | 4 | truncated package | PLANNED | Not implemented or executed |
| PKG-PARSE-005 | 4 | malformed CBOR | PLANNED | Not implemented or executed |
| PKG-PARSE-006 | 4 | missing manifest | PLANNED | Not implemented or executed |
| PKG-PARSE-007 | 4 | missing encrypted content | PLANNED | Not implemented or executed |
| PKG-PARSE-008 | 4 | duplicate recipient | PLANNED | Not implemented or executed |
| PKG-PARSE-009 | 4 | unsupported schema | PLANNED | Not implemented or executed |
| PKG-PARSE-010 | 4 | excessive declared length | PLANNED | Not implemented or executed |
| PKG-PARSE-011 | 4 | unknown algorithm | PLANNED | Not implemented or executed |
| PKG-TAMPER-001 | 4 | modified manifest | PLANNED | Not implemented or executed |
| PKG-TAMPER-002 | 4 | modified ciphertext | PLANNED | Not implemented or executed |
| PKG-TAMPER-003 | 4 | modified recipient entry | PLANNED | Not implemented or executed |
| PKG-TAMPER-004 | 4 | invalid publisher signature | PLANNED | Not implemented or executed |
| CLIENT-STATE-001 | 5 | legal transition sequence | PLANNED | Not implemented or executed |
| CLIENT-STATE-002 | 5 | invalid transition rejected | PLANNED | Not implemented or executed |
| CLIENT-KEY-001 | 5 | wrong recipient key | PLANNED | Not implemented or executed |
| CLIENT-KEY-002 | 5 | wrong password | PLANNED | Not implemented or executed |
| CLIENT-DEC-001 | 5 | valid decryption | PLANNED | Not implemented or executed |
| CLIENT-DEC-002 | 5 | altered ciphertext | PLANNED | Not implemented or executed |
| CLIENT-DEC-003 | 5 | invalid CEK wrap | PLANNED | Not implemented or executed |
| MEM-001 | 5 | no public raw-plaintext API | PLANNED | Not implemented or executed |
| MEM-002 | 5 | sensitive buffer cleanup | PLANNED | Not implemented or executed |
| MEM-003 | 5 | logging review | PLANNED | Not implemented or executed |
| LEDGER-ID-001 | 6 | register recipient | PLANNED | Not implemented or executed |
| LEDGER-ID-002 | 6 | duplicate identity | PLANNED | Not implemented or executed |
| LEDGER-ID-003 | 6 | revoked key | PLANNED | Not implemented or executed |
| LEDGER-DOC-001 | 6 | register document | PLANNED | Not implemented or executed |
| LEDGER-DOC-002 | 6 | duplicate document | PLANNED | Not implemented or executed |
| LEDGER-RES-001 | 6 | valid reservation | PLANNED | Not implemented or executed |
| LEDGER-RES-002 | 6 | unknown recipient | PLANNED | Not implemented or executed |
| LEDGER-RES-003 | 6 | revoked recipient | PLANNED | Not implemented or executed |
| LEDGER-RES-004 | 6 | unknown document | PLANNED | Not implemented or executed |
| LEDGER-FIN-001 | 6 | valid finalization | PLANNED | Not implemented or executed |
| LEDGER-FIN-002 | 6 | no reservation | PLANNED | Not implemented or executed |
| LEDGER-FIN-003 | 6 | wrong recipient | PLANNED | Not implemented or executed |
| LEDGER-FIN-004 | 6 | wrong document | PLANNED | Not implemented or executed |
| LEDGER-FIN-005 | 6 | wrong key ID | PLANNED | Not implemented or executed |
| LEDGER-FIN-006 | 6 | invalid ML-DSA signature | PLANNED | Not implemented or executed |
| LEDGER-FIN-007 | 6 | altered receipt | PLANNED | Not implemented or executed |
| LEDGER-REPLAY-001 | 6 | duplicate finalization | PLANNED | Not implemented or executed |
| LEDGER-CONC-001 | 6 | concurrent finalization | PLANNED | Not implemented or executed |
| WM-TEXT-001 | 7 | clean extraction | PLANNED | Not implemented or executed |
| WM-TEXT-002 | 7 | copied file | PLANNED | Not implemented or executed |
| WM-TEXT-003 | 7 | renamed file | PLANNED | Not implemented or executed |
| WM-TEXT-004 | 7 | single-bit corruption | PLANNED | Not implemented or executed |
| WM-TEXT-005 | 7 | recoverable corruption | PLANNED | Not implemented or executed |
| WM-TEXT-006 | 7 | unrecoverable corruption | PLANNED | Not implemented or executed |
| WM-TEXT-007 | 7 | normalization | PLANNED | Not implemented or executed |
| WM-TEXT-008 | 7 | text sanitization | PLANNED | Not implemented or executed |
| WM-TEXT-009 | 7 | false positive | PLANNED | Not implemented or executed |
| MEM-ATOMIC-001 | 8 | Force finalization failure after successful watermarking; no plaintext output | PLANNED | Not implemented or executed |
| MEM-ATOMIC-002 | 8 | signing failure | PLANNED | Not implemented or executed |
| MEM-ATOMIC-003 | 8 | watermark failure | PLANNED | Not implemented or executed |
| MEM-ATOMIC-004 | 8 | commit timeout | PLANNED | Not implemented or executed |
| MEM-ATOMIC-005 | 8 | output permission failure | PLANNED | Not implemented or executed |
| MEM-ATOMIC-006 | 8 | crash simulation after decrypt | PLANNED | Not implemented or executed |
| MEM-ATOMIC-007 | 8 | no plaintext temp files | PLANNED | Not implemented or executed |
| MEM-ATOMIC-008 | 8 | no plaintext logs | PLANNED | Not implemented or executed |
| WM-PDF-001 | 9 | clean file | PLANNED | Not implemented or executed |
| WM-PDF-002 | 9 | copy | PLANNED | Not implemented or executed |
| WM-PDF-003 | 9 | rename | PLANNED | Not implemented or executed |
| WM-PDF-004 | 9 | metadata removal | PLANNED | Not implemented or executed |
| WM-PDF-005 | 9 | incremental save | PLANNED | Not implemented or executed |
| WM-PDF-006 | 9 | PDF optimization | PLANNED | Not implemented or executed |
| WM-PDF-007 | 9 | page extraction | PLANNED | Not implemented or executed |
| WM-PDF-008 | 9 | partial text edits | PLANNED | Not implemented or executed |
| WM-PDF-009 | 9 | carrier corruption | PLANNED | Not implemented or executed |
| WM-PDF-010 | 9 | recoverable bit errors | PLANNED | Not implemented or executed |
| WM-PDF-011 | 9 | excessive corruption | PLANNED | Not implemented or executed |
| WM-PDF-012 | 9 | print-to-PDF | PLANNED | Not implemented or executed |
| WM-PDF-013 | 9 | rasterization | PLANNED | Not implemented or executed |
| WM-PDF-014 | 9 | OCR recovery experiment | PLANNED | Not implemented or executed |
| WM-PDF-015 | 9 | false positive test | PLANNED | Not implemented or executed |
| FORENSIC-001 | 10 | Alice leak | PLANNED | Not implemented or executed |
| FORENSIC-002 | 10 | Bob leak | PLANNED | Not implemented or executed |
| FORENSIC-003 | 10 | Charlie leak | PLANNED | Not implemented or executed |
| FORENSIC-004 | 10 | no watermark | PLANNED | Not implemented or executed |
| FORENSIC-005 | 10 | corrupt watermark | PLANNED | Not implemented or executed |
| FORENSIC-006 | 10 | unknown reservation | PLANNED | Not implemented or executed |
| FORENSIC-007 | 10 | reservation without receipt | PLANNED | Not implemented or executed |
| FORENSIC-008 | 10 | altered receipt | PLANNED | Not implemented or executed |
| FORENSIC-009 | 10 | invalid ML-DSA signature | PLANNED | Not implemented or executed |
| FORENSIC-010 | 10 | document mismatch | PLANNED | Not implemented or executed |
| FORENSIC-011 | 10 | low-confidence extraction | PLANNED | Not implemented or executed |
| FORENSIC-012 | 10 | false-positive corpus | PLANNED | Not implemented or executed |
| E2E-001 | 14 | Full Alice/Bob/Charlie workflow and all mandatory acceptance negatives | PLANNED | Not implemented or executed |
| PKI-REV-004 | 2 | Reject stale CRL, sequence rollback, missing CRL and incorrect identity binding | PLANNED | Not implemented or executed |
| PKI-ROT-001 | 2 | Rotate keys while preserving historical public verification evidence | PLANNED | Not implemented or executed |
| PKI-BUNDLE-001 | 2 | Reject oversized KDF parameters, role/metadata swaps and reused nonces | PLANNED | Not implemented or executed |
| ENC-012 | 3 | Assert one encrypted content object for multiple recipient wrappers | PLANNED | Not implemented or executed |
| ENC-013 | 3 | Reject HKDF context mismatch and nonce/key reuse in publisher retry paths | PLANNED | Not implemented or executed |
| PKG-PARSE-012 | 4 | Reject duplicate CBOR keys, noncanonical encodings, overflow and trailing bytes | PLANNED | Not implemented or executed |
| LEDGER-REV-001 | 6 | Revocation before finalization rejects; concurrent stale finalization conflicts | PLANNED | Not implemented or executed |
| LEDGER-RES-005 | 6 | Replayed nonce under fresh transaction ID and wrong submitter reject | PLANNED | Not implemented or executed |
| LEDGER-OPS-001 | 6 | Peer/orderer outage and restart preserve state and fail closed | PLANNED | Not implemented or executed |
| LEDGER-COMMIT-001 | 8 | Endorsement/orderer acknowledgement/invalid commit never authorize output | PLANNED | Not implemented or executed |
| MEM-ATOMIC-009 | 8 | Symlink, existing output, disk full and partial-write failures handled safely | PLANNED | Not implemented or executed |
| CLIENT-CONC-001 | 8 | Concurrent workflows cannot exchange state, receipts or frozen output bytes | PLANNED | Not implemented or executed |
| FORENSIC-013 | 10 | Transplanted valid watermark cannot obtain exact-hash VERIFIED_ATTRIBUTION | PLANNED | Not implemented or executed |
| FORENSIC-014 | 10 | Changed output with surviving frame is at most LOW_CONFIDENCE | PLANNED | Not implemented or executed |
| GUI-001 | 11 | UI cannot request plaintext; errors, cancellation and repeated operations fail safely | PLANNED | Not implemented or executed |
| FUZZ-001 | 12 | Fuzz CADP, certificate, receipt, watermark, PDF and chaincode parsers | PLANNED | Not implemented or executed |
| OFFLINE-001 | 13 | Clean disconnected build, provisioning, encryption, release and forensics | PLANNED | Not implemented or executed |
