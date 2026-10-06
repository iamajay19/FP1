# Watermark Design

Status: PLANNED initial design; no extraction or robustness experiment executed.

## Frame contract

The pre-ECC frame is a fixed 101 bytes:

| Field | Size / encoding |
|---|---|
| MAGIC | 4 ASCII bytes `CADW` |
| VERSION | 1 byte = 1 |
| RESERVATION_IDENTIFIER | 32 bytes, decoded Fabric reservation_txid |
| DOCUMENT_BINDING | 32 bytes: H(D(["CADP-WM-BIND-v1", document_id, manifest_sha256])) |
| CHECKSUM | 32 bytes: SHA-256 of preceding 69 frame bytes |

Checksum detects corruption, not malicious forgery. The receipt signs H(all 101
frame bytes) and H(final output). ECC wraps this frame; its profile must be fixed
with test vectors in Phase 7 before encoding any real artifact. Scheme identifier
selects the entire ECC/carrier/repetition profile; future changes require a new
scheme/version. No recipient name or private key is embedded.

## Text prototype (Phase 7)

Map encoded bits to U+200B/U+200C, with deterministic eligible insertion positions,
framing and repetition. Reject insufficient capacity and inputs whose existing
markers create ambiguous decoding. Select a maintained error-correction library;
record code parameters, interleaving, repetition count and maximum error/erasure
budget. Self-extract from the exact final byte buffer and require the intended frame.
Sanitization, some normalization pipelines, copy/paste and format conversion may
strip or alter carriers; each transformation gets an empirical result, not an
assumed guarantee. Invisible Unicode may still affect search/accessibility.

## PDF coordinate modulation (Phase 9)

Use selected text-run coordinates and a bounded displacement delta: bit zero shifts
negative, bit one positive. Compare observed positions to authenticated reference
geometry; differences within threshold tau are erasures. Do not modify every glyph.
Carrier selection must be reproducible before the reservation identifier is known:
use document-bound public selection metadata, not a seed obtainable only by decoding
the watermark. Select carriers before inserting recipient bits.

Reference geometry must be created by the publisher from its existing source,
authenticated and bound to document_id/manifest, then stored with restricted forensic
access. Recipient plaintext must not be exported to generate a reference. A compact
reference may still reveal layout/text relationships; minimize its contents. Exact
reference format, signature placement, carrier alignment after edits, delta/tau,
ECC capacity and supported PDF subset are Q-05 and must be settled before Phase 9.
No hidden promise of blind extraction exists in the initial design.

Initial intended support is text-bearing PDFs with sufficient stable carriers.
Scanned/encrypted PDFs, insufficient capacity, active content, malformed structures,
embedded files or unsupported font/layout features must reject before release.
Phase 9 must define the exact supported subset. Parsing/writing is in process without
temporary plaintext; saving incrementally must not retain original unwatermarked
revisions. Visual equivalence and accessibility effects require evaluation.

## Forensic evidence and confidence

Preserve and hash the candidate; record observations, erasures, decoding parameters,
frame checksum/binding, reference identity and all tool versions. Verify reservation,
receipt signature, historical certificate/authority and ledger evidence independently.
Compare candidate hash to final_output_sha256. For version 1, reserve
`VERIFIED_ATTRIBUTION` for an exact output match with complete valid evidence.
Changed-file candidates with a surviving frame are at most `LOW_CONFIDENCE` pending
a separately reviewed transformed-document attribution policy. Do not treat ECC
success as authentication or a probability of human guilt.

Public identifiers/checksums can be copied onto other documents; document binding
helps but is not a secret anti-transplant mechanism. Exact hash comparison detects
changed bytes. Distinguish WATERMARK_NOT_FOUND, WATERMARK_CORRUPTED,
UNKNOWN_RESERVATION, RECEIPT_NOT_FOUND, INVALID_SIGNATURE,
DOCUMENT_BINDING_FAILURE, INVALID_CERTIFICATE and UNSUPPORTED_DOCUMENT.
No robustness claim for print-to-PDF, rasterization, OCR, collusion or arbitrary edits
without recorded experiments, including a false-positive corpus.
