# CADP Package Format

Status: PLANNED version-1 design. No parser or encoder exists. Phase 3 implements
bounded encoding/basic decoding; Phase 4 adds systematic hardening/fuzzing. Basic
safety is required from the first parser, not postponed until Phase 4.

## Container

Choose a small length-prefixed container, not ZIP (no filenames, traversal,
compression or multiple ambiguous members). Integers below are unsigned big-endian.

```text
magic                 8 bytes: ASCII "CADP" followed by 00 00 00 01
manifest_length       4 bytes
content_length        8 bytes (includes GCM tag)
signature_length      4 bytes (must equal 3309)
manifest              manifest_length bytes: S("CADP-MANIFEST-v1", body)
encrypted_content     content_length bytes: ciphertext || 16-byte tag
publisher_signature   signature_length bytes: ML-DSA-65 over manifest bytes
EOF                   no trailing bytes
```

Header fixed size is 24 bytes. Validate lengths with checked arithmetic and exact
file size before allocation. Header lengths are transport framing, not the signed
identity; compare content length to the signed manifest field. Unknown magic/version,
truncation, zero/missing sections, duplicate fields or trailing data reject the package.
No compressed container is supported. Limits: total file 80 MiB; manifest 8 MiB;
content 64 MiB + 16 bytes; plaintext input at most 64 MiB; recipients 1..1000.
Enforce all limits together, nesting depth <= 8 and aggregate decoded items <= 50,000.
Limits are conservative baseline policies to benchmark, not tested performance claims.

## Manifest schema

All fields required, no unknown keys. `D`, `S`, IDs and algorithms follow
[CRYPTO_PROTOCOL.md](CRYPTO_PROTOCOL.md).

| Field | Value |
|---|---|
| schema_version | 1 |
| document_id | B[32] |
| media_type | `application/pdf` or `text/plain;charset=utf-8` |
| content_algorithm | `AES-256-GCM` |
| content_nonce | B[12] |
| encrypted_content_sha256 | B[32], H(exact ciphertext plus tag) |
| encrypted_content_length | U64, exact content length |
| publisher_id, publisher_signature_key_id | B[32] each; must match trusted publisher registry |
| recipient_entries | Array of exact entry maps below |
| creation_metadata | Exact map `{created_at: U64}`; publisher claim, not trusted time |

Entry fields: `recipient_id:B[32]`, `kem_key_id:B[32]`,
`signature_key_id:B[32]`, `recipient_certificate_sha256:B[32]`,
`kem_ciphertext:B[1088]`, `cek_wrap_nonce:B[12]`, `wrapped_cek:B[48]`.
Certificate hash is H(exact signed certificate envelope bytes). Certificates are
provisioned separately. Entries are sorted lexicographically by raw recipient_id;
reject duplicate recipient IDs and repeated KEM key IDs. No adding alternate
signing identities for one recipient. Certificates must pass current validation.

The signature covers every field and every recipient entry, including wrapped CEKs
and KEM ciphertexts; H(C) binds the single content object. `manifest_sha256` is
H(exact S manifest bytes), used in registration/reservations/receipts. Signature and
ciphertext hash verification happen before recipient decapsulation.

Parsing must not interpret package metadata as a filesystem path. Hold a stable
input snapshot/file handle and verify the bytes actually used, preventing TOCTOU
between verification and decryption. Read limits apply independently of declared
sizes; no huge allocations from attacker-controlled counts. JSON is inspection-only
and must never replace signed CBOR bytes in verification.
