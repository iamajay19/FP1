# Phase 2B — Private-Key Storage Preflight

Date: 2026-10-23. Status: **BLOCKED BEFORE PRODUCTION IMPLEMENTATION**.
Phase 2B is authorized; Phases 2C, 2D and 3 have not begun.

## Repository and approval gate

Inspected HEAD: `35d2164c15ce5d6888d01c65dd88ee690f814c4e`
(`Record final Phase 2A Astra approval`). The staged diff is empty.
[PHASE_2A_ASTRA_CONFIRMATION.md](PHASE_2A_ASTRA_CONFIRMATION.md) ends with
`PHASE 2A SECURITY FINAL CONFIRMATION: APPROVED`. Compared with its reviewed
implementation commit `ce67343cff345dd74052b2103c8dc3f725b32d9c`, HEAD adds only
the two final confirmation documents. The approved implementation is unchanged.

Existing owner changes to CRYPTO_PROTOCOL.md and KEY_MANAGEMENT.md and untracked
history/original audit files were present at entry and are preserved. No Git
initialization, staging, commit or metadata repair was performed. No network access,
dependency changes, production secrets or recipient identities were involved.

The required documentation was read and the current manifests/lock, foundation,
PQC wrappers/tests, build configuration and specified validators were inspected.
The existing locked offline build passed. Actual commands/results and preservation
hashes are in [PHASE_2B_VALIDATION.txt](PHASE_2B_VALIDATION.txt).

## Blocking protocol decision

The user's Phase 2B section 11 requires:

> If exact storage AAD/domain labels have NOT been finalized in the authoritative
> documentation: STOP before committing a new protocol-level label.

Section 33 also requires stopping when an unapproved protocol decision is found.
This is the reason for the stop; it is not a missing dependency or a reopening of
the Phase 1/2A audits.

[CRYPTO_PROTOCOL.md](CRYPTO_PROTOCOL.md) lines 141–157 approve Argon2id, separate
AES-256-GCM encryption, full public-header authentication and the literal roles
`kem` / `signature`. They specify header contents, cost ranges and size bounds.
They do **not** finalize the storage domain label, exact AAD CBOR value, outer
bundle/header/KDF/entry field names or map structure. No later decision supplies
those exact bytes. Foundation `Domain` contains only key-ID, certificate, CRL and
PoP labels; there is no implemented storage domain or bundle schema.

Implementing a permanent storage format now would silently choose protocol bytes.
No storage label, schema, password helper, KDF wrapper, benchmark, AEAD operation,
file persistence or key import/export API has therefore been implemented.

## Smallest proposed decision — NOT APPROVED

The following is one concrete version-1 schema/AAD proposal for owner review.
It is **PROPOSED ONLY**, not an amendment to CRYPTO_PROTOCOL.md or an approved
decision-log entry. No code uses these proposed names or bytes.

Use the existing RFC 8949 core deterministic `D` profile, with exact text-key maps:

```text
bundle = {
    header: H,
    kem: {nonce: B[12], ciphertext: B[2416]},
    signature: {nonce: B[12], ciphertext: B[4048]}
}
```

`ciphertext` is encrypted expanded private-key bytes followed by the full B[16]
GCM tag. No plaintext CBOR object combines the private keys. AES-256-GCM is fixed
by schema version 1; the format offers no cipher selection.

H would have exactly these ten fields:

| Field | Required type / value |
|---|---|
| schema_version | U64 = 1 |
| recipient_id | B[32] |
| kem_algorithm | T = `ML-KEM-768` |
| kem_key_id | B[32], recomputed by the approved Phase 2A rule |
| kem_public_key | B[1184] |
| signature_algorithm | T = `ML-DSA-65` |
| signature_key_id | B[32], recomputed by the approved Phase 2A rule |
| signature_public_key | B[1952] |
| certificate_hash | Required caller-supplied B[32] binding; no optional/null/omitted form |
| kdf | Exact map below |

Phase 2B would bind the supplied certificate hash without issuing or validating
a certificate. Fixtures would use synthetic public metadata. Certificate processing
remains outside Phase 2B; this proposal does not define a substitute certificate.

The proposed exact KDF map is:

```text
{
    name: "Argon2id",
    version: 19,
    memory_kib: U64,
    time_cost: U64,
    parallelism: U64,
    output_length: 32,
    salt: B[16]
}
```

The proposed exact AAD for each independently encrypted object is:

```text
AAD_role = D([
    "CADP-KEY-BUNDLE-AAD-v1",
    {header: H, role: role}
])
```

Here H is the complete **nested map**, not a byte string, hash or JSON rendering.
The role is exactly text `kem` for the kem entry or `signature` for the signature
entry, selected by the trusted typed operation rather than an arbitrary caller
string. This uses one proposed new domain label and binds both keys/IDs, recipient,
certificate binding, KDF parameters and salt to both objects. Nonces remain the
actual GCM nonce inputs; nonce changes must fail authentication. Reject equal
nonces before KDF. No extra unsigned metadata is trusted.

All maps would require their exact fields and existing encoded-key bytewise ordering.
Retain the approved 32 KiB / depth 8 / 256-item limits and exact-byte canonical
validation before cryptography. Unsupported versions/algorithms/parameters reject.

Approval of this single schema/AAD decision, or provision of an alternative exact
approved schema/AAD, is needed to resume implementation. General Phase 2B approval
does not override the explicit missing-label stop condition in the same request.

## Existing bounds and unselected benchmark profile

Already approved: Argon2id v0x13, salt 16 bytes, output 32 bytes, memory 64–256 MiB
(65536–262144 KiB), time cost 3–6 and parallelism 1–4. Check all bounds and integer
conversions before workspace allocation/KDF. The approved defaults-off library
processes Argon2 lanes sequentially; lane count does not imply that many OS threads.
The 64 MiB / 3 / 4 point remains a **candidate**, not a measured/selected profile.
No new benchmark was executed and no duration is invented.

## Tests and implementation status

This is preflight evidence only. Existing-build and documentation checks are not
Phase 2B runtime tests. No Phase 2B test IDs were added or marked TESTED; all 19
original Phase 2 PKI rows remain PLANNED. Full implementation/regression validation,
adversarial bundle/filesystem testing and a separate implementation security
self-review are NOT EXECUTED because the explicit protocol gate stopped the task.
No Phase 2B completion or storage-security claim follows from a successful build.

## Preservation and limitations

No production code, dependency pin/feature, Phase 1 FFI/configuration, Phase 2A
CBOR/identifier/crypto/RNG helper or historical audit evidence was modified.
Only this new review and its new validation record were added; task-owned preflight
records/build outputs remain ignored under .deps/p2b-preflight. Preexisting owner
changes are not attributed to this task. No automatic commit was made.

Accepted limitations remain: M-01/L-19 fatal backend termination can bypass Drop;
software zeroization cannot cover all library/compiler/register/terminal copies,
swap/dumps or process/power failure. Password strength and offline guessing,
compromised-host exposure, trusted-client assumptions and filesystem permissions
are not solved by Phase 2B. No trusted time/host rollback guarantee is introduced.

## Recommendation

Resolve the proposed exact bundle schema/AAD decision before resuming Phase 2B.
Do not begin Phase 2C, 2D or 3. The authorized Phase 2B implementation is incomplete.

PHASE 2B BLOCKED


## Later explicit resolution — 2026-10-23

The owner subsequently approved an exact storage format; see D-33 and the normative
CRYPTO_PROTOCOL.md section. The historical BLOCKED result and unapproved draft above
are preserved. The approved format differs from that draft, including omission of
public keys/certificate hash and exact kdf/entry field names. Production implementation
uses D-33 exclusively. New implementation evidence is recorded separately.
