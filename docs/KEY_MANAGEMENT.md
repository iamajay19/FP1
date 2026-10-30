# Key Management

Status: PLANNED. Phase 2 will implement provisioning, private-key storage and PKI.

| Role | Material | Custody / permitted use |
|---|---|---|
| Offline CA | Independent ML-DSA-65 signing key | Restricted offline administration; signs certificates and CRLs only |
| Publisher | Independent ML-DSA-65 signing key | Approved publisher registry; signs package manifests only |
| Recipient KEM | ML-KEM-768 key pair | Encrypted local key bundle; recover CEK only |
| Recipient signature | Independent ML-DSA-65 key pair | Same protected bundle with distinct role/nonces; signs receipts and domain-separated enrollment PoP only |
| Fabric operator/client | Native MSP/TLS keys | Infrastructure plane; separate from application PQ keys |
| Per-document | Random CEK and nonce | One package encryption; erase after use |
| Per-recipient wrap | KEM shared secret, HKDF KEK and nonce | Scoped temporary values; erase immediately after wrap/recovery |

Provisioning must independently generate both recipient pairs using OS-backed
library randomness, recompute distinct fingerprints and verify possession without
logging secrets. Pin CA and publisher public trust anchors through an authenticated
out-of-band administrative process. Package-supplied trust anchors are never trusted.
Recipient labels such as Alice are display-only; cryptographic IDs are the bindings.

Store keys at rest using the [bundle protocol](CRYPTO_PROTOCOL.md); protect directory
permissions and avoid symlink-following writes. Do not put private keys, passwords,
source documents or real evidence in Git/test fixtures. Encrypted key material can
be backed up offline under controlled custody; never plaintext export as a debug
convenience. Backup encryption, password recovery policy and CA rotation procedures
must be finalized under Q-02 before real provisioning.

Rotate by issuing new IDs/certificates and preserving old public verification records.
Revoke either KEM or signing key through a signed cumulative CRL and ledger update;
reject new reservations/finalizations using revoked keys. Existing ciphertext can
still be decrypted by anyone retaining its usable KEM private key; republishing does
not erase old copies. Retain signed historical CRLs/certificates and ledger references
for forensics; disclose later compromise instead of hiding it.

A memory-hardened Rust process should handle unlocked material. Python orchestration
must not casually duplicate private-key bytes into immutable strings or subprocess
arguments. Zeroization is best-effort across library boundaries and is not proof of
physical erasure. Audit concrete allocation/lifetime paths in the implementing phases.

## Q-02 provisioning proof of possession — design decision D-24

Status: DESIGN RESOLVED for the Phase 2 provisioning transcript, 2026-10-07;
NOT IMPLEMENTED / NOT RUNTIME TESTED. This documents the protocol authorized by
this pre-implementation task. KDF benchmarking, backup/recovery and CA/publisher
rotation procedures remain separate open parts of Q-02. No production identities
may be provisioned until implementation, adversarial tests and review are complete.

### Preconditions and ownership

Use the approved Phase 1 safe ML-KEM-768 and ML-DSA-65 operations exclusively.
Provisioning is an authenticated administrative transaction over a controlled local
session or authenticated out-of-band offline delivery. An air gap or an arbitrary
USB file does not authenticate that session. Recipient and CA must agree on the
intended issuer, CA public-key fingerprint, recipient ID, both public keys and intended
certificate policy before acting on a challenge. Administrative identity/enrollment
authorization is separate from PoP. These proofs establish live access to keys, not
human identity, exclusive custody, trustworthy client execution or future secrecy.

The CA signing key remains limited to certificates/CRLs; this design does not add a
CA-signed challenge or general CA signing oracle. The authenticated administrative
path is therefore required, including to prevent a malicious party presenting a
self-generated encapsulation as a CA challenge. Forwarding a challenge to a willing
real key holder cannot be cryptographically prevented by this PoP protocol. A
compromised/dishonest CA can issue certificates and knows the KEM challenge secret.
The MAC is confirmation to that honest CA, not transferable evidence against it.

Client generates independent recipient keypairs and random recipient_id B[32]. CA
validates submitted algorithm names, lengths and recomputed IDs, and separately
approves enrollment policy. The CA constructs the complete final certificate_body
using the exact existing schema, including its own fresh random certificate_serial,
issuer and agreed validity interval. The client compares that body against its
intended keys/recipient/policy, allowing the CA-generated serial. A future-dated
certificate can be explicitly agreed; PoP does not bypass its later validity checks.

Keep one live pending transaction per CA provisioning instance in Phase 2. Generate
independent fresh OS-random transaction_id B[32] and ca_nonce B[32]. Encapsulate to the
submitted ML-KEM-768 public key exactly once, producing ct B[1088] and ss B[32].
Any entropy, allocation or backend failure aborts. No nonce, transaction or
encapsulation reuse on retry. No fallback entropy or decapsulation-status PoP.

### Exact canonical challenge

Notation D/H/B[n]/U64 and map ordering are unchanged from CRYPTO_PROTOCOL.md.
`ca_signature_key_id = H(D(["CADP-SIG-ID-v1", pinned_CA_public_key]))` is a public
fingerprint, not permission to use a recipient or publisher key as a CA key.
Challenge body C is the following EXACT six-field map; no optional/unknown fields:

| Field | Exact type / meaning |
|---|---|
| schema_version | U64 = 1 |
| transaction_id | Fresh CA OS-random B[32] |
| ca_nonce | Independent fresh CA OS-random B[32] |
| ca_signature_key_id | B[32]; matches authenticated pinned CA anchor |
| certificate_body | Exact existing certificate map, with all fields below |
| kem_ciphertext | B[1088], CA encapsulation to certificate_body.kem_public_key |

certificate_body contains EXACTLY: schema_version U64=1;
certificate_serial B[32]; recipient_id B[32]; kem_algorithm T="ML-KEM-768";
kem_key_id B[32]; kem_public_key B[1184]; signature_algorithm T="ML-DSA-65";
signature_key_id B[32]; signature_public_key B[1952]; issuer B[32];
valid_from U64; valid_until U64 with valid_from < valid_until.
Recompute both key IDs by the existing domain-separated rules, and require both
public keys, recipient ID, issuer and requested validity policy to match the locally
approved transaction. No labels/names replace cryptographic IDs.

```text
challenge_bytes = D(["CADP-POP-CHALLENGE-v1", C])
challenge_hash  = H(challenge_bytes)
info           = D(["CADP-POP-KEM-KEY-v1", challenge_hash])
K_pop          = HKDF-SHA-256.ExtractThenExpand(
                   IKM=ss, salt=C.ca_nonce, info=info, L=32)
mac_message    = D(["CADP-POP-KEM-v1", {challenge: challenge_bytes}])
kem_proof      = HMAC-SHA-256(K_pop, mac_message)  # full 32-byte tag
signature_body = D(["CADP-POP-SIG-v1",
                    {challenge: challenge_bytes, kem_proof: kem_proof}])
signature      = ML-DSA-65.Sign(recipient_sk_sig, signature_body,
                               pure_message_mode, empty_context)
response       = D({body: signature_body, signature: signature})
```

`challenge` and envelope `body` are CBOR BYTE STRINGS holding the exact indicated
canonical bytes, not nested values, hex text, JSON or hashes in their place.
`kem_proof` is B[32]; signature is B[3309]. The response uses the existing signed
object convention; signature_body is the S("CADP-POP-SIG-v1", body) byte string.
The CA sends only challenge_bytes, never ss, K_pop or an expected MAC. Both sides
use BOTH HKDF stages; no from_prk shortcut, SHA(secret||message), ad-hoc MAC,
HMAC truncation, external-mu/prehash signature or non-empty context is permitted.
HKDF provides a purpose-bound confirmation key, HMAC authenticates the complete
challenge, and the recipient signature binds the KEM response to the same signing
key and proposed certificate. Distinct labels separate CA challenge, KDF, MAC and
signature roles from each other and from certificates, receipts and CEK wrapping.

The recipient validates/authenticates the complete challenge before decapsulation
or signing, decapsulates using its matching KEM secret key, derives K_pop and produces
the response above. A returned implicit-rejection secret is not success evidence.
Do not reveal that secret; an honest CA's MAC verification will fail if it differs.
All PoP challenge/response objects are bounded to 16 KiB, depth 8 and 256 total decoded
items including embedded canonical objects. Reject malformed lengths before PQC/KDF
calls. These are enrollment bounds, not changes to certificate/bundle/CRL formats.

### CA verification, consumption and failure behavior

CA retains the exact challenge_bytes, approved certificate_body and a cleanup-owned
K_pop in process memory. Erase ss and derived/intermediate copies promptly; avoid
logging them. Expire the challenge after 300 seconds of process-local monotonic
elapsed time; reject unavailable/invalid timer state. This timeout is resource/replay
hygiene, not a trusted wall-clock or persistence guarantee. Suspend/root manipulation
and transport authorization remain operational assumptions.

On the authenticated session, atomically move PENDING to VERIFYING on the first
response attempt. A malformed response consumes that attempt too. Recheck deadline,
strictly parse/canonical-validate the envelope, signed body and embedded challenge,
and require embedded challenge BYTES to equal the exact live CA-owned bytes.
Never reconstruct expected issuance state from attacker-supplied response fields.
Verify the pure/empty-context signature with the submitted signing public key from
CA-owned state, and verify the full 32-byte HMAC with a library constant-time
`verify_slice`-equivalent operation under retained K_pop. No ordinary tag `==`,
early-exit byte loop or decapsulation-status check substitutes for MAC verification.
Either false result/error rejects with generic provisioning failure, cleanses pending
secret state and enters terminal FAILED. Success enters CONSUMED; issue exactly the
stored approved certificate_body once only after BOTH verifications succeed.

Use a fresh transaction after timeout, cancellation, malformed response, failed
proof, issuance error or retry. There is no same-challenge retry or partial-proof
success state. On process death/restart all pending/verification transactions are
invalid; do not reload/export their K_pop or resurrect a transcript file as pending
state. A completed response alone cannot create a transaction. Concurrent duplicate
responses cannot both consume the same pending state. A crash after consumption
requires new provisioning, not recovery by replaying the response.

### Threat analysis and planned checks

| Case | Required rejection / boundary |
|---|---|
| Replay or challenge reuse | Fresh transaction/nonce/encapsulation plus one live one-shot state; old/consumed/expired/unknown response rejects, including after restart |
| Cross-recipient or cross-KEM substitution | Full certificate including recipient_id, algorithms, IDs and keys is inside exact challenge, HKDF info hash, MAC and signature; CA compares to its own stored bytes |
| Cross-signing-key substitution / combined independent proofs | Correct recipient signature covers both exact challenge and full kem_proof; selecting verification keys from response is forbidden |
| Reflection | CA challenge contains no response MAC/signature; recipient-to-CA labels are distinct; no CA MAC response oracle or reverse-direction proof exists |
| Modified ciphertext / wrong KEM private key | Exact challenge mismatch or unrelated implicit-rejection secret; MAC rejection, never decapsulation return status |
| Transcript/header/signature/tag tampering | Strict canonicality/schema/length checks, CA exact-byte match, ML-DSA verification and HMAC verification |
| CA/key-role confusion | issuer and CA fingerprint pinned to CA role; fixed recipient algorithm roles, distinct IDs and labels; recipient certificate confers no CA/publisher authority |
| Relay to cooperating key holder | Not prevented; authenticated administrative path and honest enrollment/custody assumptions required |

Before issuance implementation is accepted, add explicit PoP positive, signature/KEM
failure, altered transcript/ciphertext, swapped recipient/keys, replay/timeout/restart
and concurrent-consumption cases under stable Phase 2 IDs. None ran in this design
review; all existing Phase 2 runtime IDs remain PLANNED.

No public secret getter is authorized for this design. The current Phase 1 SharedSecret
is opaque; future Phase 2 integration must add narrowly scoped INTERNAL safe
HKDF/PoP operations inside its ownership boundary, using borrowed secret material
only during the call. It must not export raw ss to an application crate or bypass
Phase 1 via direct C calls. Stored/decrypted private-key use also remains a separately
reviewed internal storage boundary. No such API change is made by this task.

## Q-03 Phase 2 offline time boundary — design decision D-25

Status: Phase 2 validation semantics DESIGN RESOLVED, NOT IMPLEMENTED/TESTED.
Cryptographically trustworthy offline time/host anti-rollback remains OPEN (A-06).
No timestamp server, online service, secure clock or new host control is introduced.

### Certificate checks

Use one validated U64 Unix-seconds `now` from an explicitly configured caller/local clock
at each validation boundary; test injection is explicit and never taken from the
certificate/package/CRL. Reject unavailable, pre-epoch, overflowing or malformed time.
Require valid_from < valid_until and valid_from <= now < valid_until, with ZERO
permitted skew/grace. At the upper boundary the certificate is expired. Do not cache
this result across later security boundaries; those phases revalidate with fresh time.
This is only validity relative to the supplied/local clock, not secure current time.
An incorrect but well-formed clock can accept an actually expired certificate.

### CRL freshness and sequence

Validate issuer role, full signature/canonical schema/bounds before importing state.
Require generated_at < next_update and generated_at <= now < next_update with zero
skew; revoked_at <= generated_at. Missing, future or stale CRL rejects new operations.
There is no permissive offline grace period. Operators must import a valid cumulative
replacement BEFORE next_update to maintain availability; maximum issuance/import
cadence is a deployment policy still to be fixed, not a fabricated freshness guarantee.
A stale CRL must not be applied as current authorization state or reset the watermark;
retain previously accepted sequence/history for rollback rejection and forensics.

Per pinned issuer/CA anchor, retain highest accepted sequence, canonical CRL hash,
generated_at and cumulative revocations in controlled local persistent state. First
bootstrap is an explicit authenticated administrative action with a verified current
CRL; never interpret missing/corrupt/unreadable state as an automatic sequence reset.
New updates require a STRICTLY greater sequence, nondecreasing generated_at, and
preservation of previously accepted revocation entries and their recipient/key binding.
Reject removal or modification of those entries in this cumulative version-1 profile.
Reject lower sequence. Equal sequence is allowed ONLY as an idempotent reload of the
identical canonical CRL hash/bytes, still subject to current freshness validation;
a different CRL at the same sequence rejects as conflicting state. Sequence overflow
rejects; no wrap/reset is allowed. Anchor rotation must not silently reset issuer state.

Commit ordinary local state atomically and sync it before treating the update as
accepted; a persistence failure rejects authorization. Serialize imports/checks so a
concurrent older update cannot replace a newer one. Validation must check whether
EITHER certificate KEM key ID OR signature key ID is revoked, with matching recipient
binding; recipient_id alone is insufficient. Phase 2 may implement/test these rules
against ordinary file persistence and controlled restart without claiming more.

### Actual anti-rollback limits

Sequence persistence rejects replay only relative to the retained trusted watermark.
Restoring old files, a VM/disk snapshot or the whole host can restore both clock and
sequence state. File permissions, atomic rename and signatures do NOT prove freshness
against that restoration, malicious administrators or privileged host modification.
A first-install bootstrap can be given an old still-locally-valid CRL if administrative
freshness assumptions fail. No implementation claim may call this host rollback
protection or trusted offline time. Detectable state errors fail closed; undetectable
whole-host rollback is explicitly residual. Independently retained authenticated
checkpoints, restore procedures and protected clock/counter facilities are later
operational/deployment decisions under Q-03/Q-04, not implemented in this task.


## Phase 2B approved storage format — D-33

Explicit owner approval resolved the 2026-10-23 preflight blocker. The exact
HeaderV1/ProtectedSecret/AAD bytes in CRYPTO_PROTOCOL.md are authoritative, not
the earlier draft in PHASE_2B_REVIEW.md. The approved header omits public keys and
certificate hash. Storage callers must supply the expected public keys/recipient
binding to recompute IDs; this does not issue or validate a certificate.

Only local password-protected storage is authorized here. Benchmark/profile selection,
implementation and adversarial/cleanup/filesystem tests must be evidenced separately.
Original preflight and Phase 1/2A audit evidence remain historical and intact.
