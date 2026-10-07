# Test Matrix

Phase 0 evidence is recorded in [PHASE_0_REVIEW.md](PHASE_0_REVIEW.md).
Phase 0 TESTED statuses retain their historical evidence. Phase 1 TESTED statuses
now refer to actual primitive checks in PHASE_1_VALIDATION.txt and PHASE_1_REVIEW.md.
The authorized H-01/L-01 reruns and new regressions have separate evidence in
[PHASE_1_CORRECTIONS_VALIDATION.txt](PHASE_1_CORRECTIONS_VALIDATION.txt).
The original 19 Phase 2 PKI tests and later tests remain PLANNED; Phase 2A foundation evidence is separate below. The implementation
plan supplies the detailed expected outcomes for its mandatory IDs. Additional cases
close design-review gaps. Phase is the intended implementation/validation phase.

Phase 2 was explicitly authorized on 2026-10-07 after the independent Astra re-audit
approved Phase 1. The final dependency gate is READY: the refreshed ten-pin graph,
including CBOR/HMAC/HKDF, passed integrity and independent locked offline compilation.
Earlier failed probes remain historical evidence. No Phase 2 runtime test executed. All 19 existing Phase 2
IDs below remain PLANNED. Evidence: [PHASE_2_VALIDATION.txt](PHASE_2_VALIDATION.txt);
recovery: [PHASE_2_REVIEW.md](PHASE_2_REVIEW.md). No later-phase status changed.

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
| ML-KEM-768 | 1 | key generation | TESTED | cargo test; PHASE_1_VALIDATION.txt |
| PQC-KEM-001 | 1 | valid encapsulation/decapsulation | TESTED | cargo test; PHASE_1_VALIDATION.txt |
| PQC-KEM-002 | 1 | wrong private key | TESTED | cargo test; PHASE_1_VALIDATION.txt |
| PQC-KEM-003 | 1 | modified KEM ciphertext | TESTED | cargo test; PHASE_1_VALIDATION.txt |
| PQC-KEM-004 | 1 | truncated KEM ciphertext | TESTED | cargo test; PHASE_1_VALIDATION.txt |
| PQC-SIG-001 | 1 | valid sign/verify | TESTED | cargo test; PHASE_1_VALIDATION.txt |
| PQC-SIG-002 | 1 | wrong public key | TESTED | cargo test; PHASE_1_VALIDATION.txt |
| PQC-SIG-003 | 1 | modified message | TESTED | cargo test; PHASE_1_VALIDATION.txt |
| PQC-SIG-004 | 1 | modified signature | TESTED | cargo test; PHASE_1_VALIDATION.txt |
| PQC-SIG-005 | 1 | truncated signature | TESTED | cargo test; PHASE_1_VALIDATION.txt |
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

## Additional Phase 1 checks

All IDs below have actual evidence in [PHASE_1_VALIDATION.txt](PHASE_1_VALIDATION.txt).
Review scope/limitations are in [PHASE_1_REVIEW.md](PHASE_1_REVIEW.md).

| ID | Phase | Scenario / required assertion | Status | Evidence |
|---|---|---|---|---|
| PQC-KEM-005 | 1 | Noncanonical correctly sized public key rejects; failed shared-secret output cleansed | TESTED | pqc_kem_005; cargo test |
| PQC-BACKEND-001 | 1 | Runtime 0.16.0, exact compiled source pin, precisely the two required enabled algorithms | TESTED | pqc_backend_001; C initialization enumeration |
| PQC-SIZE-001 | 1 | Backend sizes equal 1184/2400/1088/32 and 1952/4032/3309 | TESTED | pqc_size_001 |
| PQC-BOUNDARY-001 | 1 | Empty/short/long KEM public key and ciphertext reject before native operation | TESTED | pqc_boundary_001; thread-local native-call counter |
| PQC-BOUNDARY-002 | 1 | Internal KEM secret/output length guards reject before native operation | TESTED | pqc_boundary_002; generated public secret type has no raw importer |
| PQC-BOUNDARY-003 | 1 | Empty/short/long signing public key/signature reject before native operation | TESTED | pqc_boundary_003 |
| PQC-BOUNDARY-004 | 1 | Internal signing secret/capacity guards reject before native operation | TESTED | pqc_boundary_004 |
| PQC-MSG-001 | 1 | Empty and borrowed 1 MiB messages sign/verify without wrapper-sized message allocation | TESTED | pqc_msg_001 |
| PQC-CTX-001 | 1 | Ordinary and NULL,0-context sign/verify interoperate in both directions | TESTED | pqc_ctx_001; no randomized signature equality assumption |
| PQC-SECRET-001 | 1 | Ordinary Debug prints only fixed redacted labels | TESTED | pqc_secret_001 |
| PQC-SECRET-002 | 1 | Drop cleanses all three secret buffer sizes before deallocation | TESTED | pqc_secret_002; live-buffer test observation |
| PQC-ALLOC-001 | 1 | Capacity-overflow allocation failure returns typed error without allocation/panic | TESTED | pqc_alloc_001 |
| PQC-LIFE-001 | 1 | Repeated operations initialize once and execute same-thread cleanup per call | TESTED | pqc_life_001 |
| PQC-CONC-001 | 1 | Four concurrent workers run KEM/signatures; fresh-process concurrent initialization | TESTED | pqc_conc_001; separate filtered process |
| PQC-KEY-001 | 1 | Raw getter, Clone and signature-to-KEM secret substitution fail compilation | TESTED | Three Rust compile-fail doctests |
| PQC-FFI-001 | 1 | Native null/length guards reject before handle allocation | TESTED | check_pqc_native.py; Clang ASan/UBSan |
| PQC-FFI-002 | 1 | Injected native handle allocation failure safely returns and balances handles | TESTED | check_pqc_native.py; disposable linker wrapping |
| PQC-FFI-003 | 1 | Injected native operation failures reject and free handles; cleanse works | TESTED | check_pqc_native.py; backend itself uninstrumented |
| PQC-FFI-004 | 1 | Injected null/inconsistent metadata rejects without null dereference or handle leak | TESTED | check_pqc_native.py |
| PQC-BUILD-001 | 1 | Missing/wrong/dirty/untracked/ignored/concealed-index source rejects; original untouched | TESTED | check_pqc_source_pin.py; six disposable-clone cases |
| PQC-SCOPE-001 | 1 | Audited PQC boundary preserved; scope validator admits explicitly approved Phase 2A foundation only; other components stay scaffold-only | TESTED | validate_phase1.py |
| PQC-BUILD-002 | 1 | Generated SHA3 AVX512VL must be undefined; reject enabled/zero/missing/ambiguous definitions and forbidden shim compilation | TESTED | check_pqc_native.py; build.rs test; fresh header evidence and separate ON-cache override in PHASE_1_CORRECTIONS_VALIDATION.txt |
| PQC-SECRET-003 | 1 | Four partial native failures pass through actual Rust error/Drop/cleanse/free; observe memory only before free | TESTED | check_pqc_native.py; isolated production rlib + support fixtures; four cases and omitted-cleanse negative control in PHASE_1_CORRECTIONS_VALIDATION.txt |


## Phase 2 pre-implementation design review — 2026-10-07

This is documentation/dependency evidence only, not new runtime test results.
Refreshed vendor integrity, cleanup-enabled compilation/API probe and SHA-2/Blake2
cleanup feature builds passed; three missing minicbor/HMAC/HKDF offline probes failed
as expected. See PHASE_2_VALIDATION.txt. ALL 19 existing Phase 2 rows remain PLANNED.

Implementing-phase coverage must include D-23 bounded canonical decoding (shortest
integers/lengths, exact fields, duplicate keys even with identical values, tags/floats/
indefinite forms, trailing bytes, encoded-key ordering, depth/item/size bounds), D-24
both proofs plus full transcript/ciphertext/key substitution, replay/expiry/restart and
concurrent consumption, and D-25 zero-skew validity boundaries, stale/future CRLs,
sequence rollback/conflicts/cumulative preservation and persistence failure. Add stable
IDs when implementing these regressions without replacing existing PKI IDs. No PoP,
CBOR, certificate, CRL, bundle, filesystem or time-policy runtime check is claimed here.


## Phase 2A foundation — 2026-10-08

Foundation implementation/test evidence ONLY: PHASE_2A_REVIEW.md and
PHASE_2A_VALIDATION.txt. Original 19 PKI rows are byte-identical and remain PLANNED.
Phase 2 is not complete; no Phase 2B/3 workflow implemented. Existing Phase 1 IDs
were rerun with native code/FFI unchanged; scope check now admits authorized utilities.

| ID | Phase | Scenario / required assertion | Status | Evidence |
|---|---|---|---|---|
| P2A-DEPS-001 | 2A | Exact pins/features, vendor/owner/Cargo checksum sets and offline compilation | TESTED | PHASE_2A_VALIDATION.txt; validate_phase2a.py |
| P2A-BUILD-001 | 2A | RNG cfg predicate accepts only one unambiguous Linux syscall selection | TESTED | PHASE_2A_VALIDATION.txt; build.rs harness |
| P2A-BUILD-002 | 2A | Actual Cargo missing/alternate/conflicting RNG cfg fails closed | TESTED | PHASE_2A_VALIDATION.txt; validate_phase2a.py |
| P2A-CBOR-001 | 2A | Canonical values, integer transitions, exact synthetic schema and stable map order | TESTED | PHASE_2A_VALIDATION.txt; cargo test debug/release |
| P2A-CBOR-002 | 2A | Overwide integers and byte/text/map lengths reject by full-byte comparison | TESTED | PHASE_2A_VALIDATION.txt; cargo test debug/release |
| P2A-CBOR-003 | 2A | Reject all indefinite forms, floats, tags, negative/simple/null/bool/break | TESTED | PHASE_2A_VALIDATION.txt; cargo test debug/release |
| P2A-CBOR-004 | 2A | Reject duplicates/alternate key spelling, unknown fields, wrong order and trailing bytes | TESTED | PHASE_2A_VALIDATION.txt; cargo test debug/release |
| P2A-CBOR-005 | 2A | Truncation, U64 oversized lengths, invalid UTF-8 and class byte limits | TESTED | PHASE_2A_VALIDATION.txt; cargo test debug/release |
| P2A-CBOR-006 | 2A | 256-item budget, depth overflow and embedded aggregate accounting | TESTED | PHASE_2A_VALIDATION.txt; cargo test debug/release |
| P2A-CBOR-007 | 2A | Exact approved domain labels and array/byte-string encoding | TESTED | PHASE_2A_VALIDATION.txt; cargo test debug/release |
| P2A-CBOR-008 | 2A | 256 deterministic typed round trips/mutations and 1280 malformed inputs | TESTED | PHASE_2A_VALIDATION.txt; cargo test debug/release |
| P2A-CBOR-009 | 2A | Accepted typed value requires exact whole-original re-encoding; overwide array rejects | TESTED | PHASE_2A_VALIDATION.txt; cargo test debug/release |
| P2A-CBOR-010 | 2A | Depth boundary, embedded depth, CRL item/entry ceilings and geometric bounded output growth | TESTED | PHASE_2A_VALIDATION.txt; cargo test debug/release |
| P2A-ID-001 | 2A | Distinct fixed key-ID types, exact key lengths, deterministic public golden vectors and changed keys | TESTED | PHASE_2A_VALIDATION.txt; cargo test debug/release |
| P2A-ID-002 | 2A | Identical synthetic bytes hash differently across KEM/signature labels | TESTED | PHASE_2A_VALIDATION.txt; cargo test debug/release |
| P2A-RNG-001 | 2A | OS-random output32 and independent calls | TESTED | PHASE_2A_VALIDATION.txt; cargo test debug/release |
| P2A-RNG-002 | 2A | Injected partial-fill failure returns typed error and no identifier/fallback | TESTED | PHASE_2A_VALIDATION.txt; cargo test debug/release |
| P2A-RNG-003 | 2A | Actual production RandomId rejects forced ENOSYS and EPERM syscalls | TESTED | PHASE_2A_VALIDATION.txt; validate_phase2a.py |
| P2A-HASH-001 | 2A | SHA-256 abc vector | TESTED | PHASE_2A_VALIDATION.txt; cargo test debug/release |
| P2A-MAC-001 | 2A | RFC4231 equivalent padded key, full tag; changed message/tag and short tags reject | TESTED | PHASE_2A_VALIDATION.txt; cargo test debug/release |
| P2A-KDF-001 | 2A | RFC5869 case1 exact32-byte extract+expand output | TESTED | PHASE_2A_VALIDATION.txt; cargo test debug/release |
| P2A-SECRET-001 | 2A | Redacted Debug and actual Drop observes wiped live heap before free | TESTED | PHASE_2A_VALIDATION.txt; cargo test debug/release |
| P2A-SECRET-002 | 2A | Disposable omission of only Drop wipe makes persistent cleanup test fail | TESTED | PHASE_2A_VALIDATION.txt; validate_phase2a.py |
