# trusted-client

Rust PQ integration (Phase 1), trusted decryption (Phase 5), atomic release (Phase 8).

Phase 1 primitive wrappers IMPLEMENTED. The full trusted decryption workflow remains
PLANNED. Follow the [implementation plan](../docs/IMPLEMENTATION_PLAN.md) and phase gates.

The dependency-free `cadp-pqc` Rust library uses pinned liboqs 0.16.0 through an
internal FFI module and C shim. It supports only ML-KEM-768 and pure ML-DSA-65 with
empty context. Backend metadata is checked before operations. No real identities,
private-key storage, document encryption, ledger or watermarking exist yet.

From the repository root:

```sh
make build
make check
```

Supply the clean source at `.deps/liboqs-0.16.0` (ignored), exact commit
`5a1a854b0dc9f2141bdc771c555ee60c37950183`, tag `0.16.0`.
An explicit `CADP_LIBOQS_SOURCE` may point to another clean checkout of that same
revision; it cannot select an arbitrary library. Cargo never fetches the source or
uses system liboqs. Build output and expanded commands stay under `target/`.

Safe APIs: `pqc::kem::{generate_keypair,encapsulate,decapsulate}` and
`pqc::signature::{generate_keypair,sign,verify}`. KEM and signature secret keys are
distinct, non-cloneable types with redacted Debug and no public byte accessor or
import operation. A private `SecretBuffer` cleanses its live heap buffer on Drop.
Secret import/export for protected storage needs a later reviewed design; no
unrestricted secret export is introduced for future convenience.

KEM implicit rejection may yield an unrelated secret for a wrong key or modified
ciphertext. Decapsulation success does not establish authorization. Phase 3 must
authenticate the CEK wrap. Signature verification authorizes only on `Ok(true)`;
invalid signatures yield `Ok(false)`, malformed lengths/setup failures yield `Err`.
Messages are borrowed; callers must bound processing time for large messages.

All unsafe Rust stays in `src/pqc/ffi.rs`; the crate denies it elsewhere. Native
handles never leave C and are freed after each call. Rust OnceLock initializes the
backend once; each operation calls thread cleanup on the same thread. Global
`OQS_destroy` is deliberately unused for process-lifetime state. Concurrent safe
operations are supported on the tested native Linux/pthread configuration.
The build explicitly excludes AVX-512 SHA3 dispatch (`OQS_USE_SHA3_AVX512VL=OFF`),
checks generated/installed headers and rejects the forbidden macro in the shim.
See the [Phase 1 review](../docs/PHASE_1_REVIEW.md) for the corrective history.

This is a research prototype using FIPS-standard algorithms, **not a FIPS-validated
module**. Zeroization is best effort and cannot erase library/register copies,
swap/dumps or privileged observations. Some upstream fatal RNG/allocation failures
terminate the process and bypass Rust Drop. See the
[Phase 1 review](../docs/PHASE_1_REVIEW.md) and
[limitations](../docs/KNOWN_LIMITATIONS.md).
