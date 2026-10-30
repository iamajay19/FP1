//! Pure FIPS 204 ML-DSA-65 with empty context; no prehash or external-mu API.

use super::{Error, ffi, secret::SecretBuffer};
use std::fmt;

pub const PUBLIC_KEY_LENGTH: usize = 1952;
pub const SECRET_KEY_LENGTH: usize = 4032;
pub const MAX_SIGNATURE_LENGTH: usize = 3309;

/// Generated signing key; cannot be used as an ML-KEM key or exported publicly.
///
/// ```compile_fail
/// use cadp_pqc::pqc::{kem, signature};
/// let (_, signing_key) = signature::generate_keypair().unwrap();
/// let result = kem::decapsulate(&signing_key, &[0u8; 1088]);
/// ```
pub struct SecretKey(pub(super) SecretBuffer);

impl fmt::Debug for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SignatureSecretKey([REDACTED])")
    }
}

pub fn generate_keypair() -> Result<([u8; PUBLIC_KEY_LENGTH], SecretKey), Error> {
    let mut public = [0u8; PUBLIC_KEY_LENGTH];
    let mut secret = SecretBuffer::zeroed(SECRET_KEY_LENGTH)?;
    ffi::signature_keypair(&mut public, secret.as_mut_slice())?;
    Ok((public, SecretKey(secret)))
}

/// Message is borrowed without copying or attacker-sized wrapper allocations.
pub fn sign(secret_key: &SecretKey, message: &[u8]) -> Result<Vec<u8>, Error> {
    let mut signature = Vec::new();
    signature
        .try_reserve_exact(MAX_SIGNATURE_LENGTH)
        .map_err(|_| Error::AllocationFailure)?;
    signature.resize(MAX_SIGNATURE_LENGTH, 0);
    let written = ffi::sign(secret_key.0.as_slice(), message, &mut signature)?;
    signature.truncate(written);
    Ok(signature)
}

/// Ok(true) means valid. Ok(false) means cryptographic rejection; malformed
/// lengths return typed errors before FFI. Backend/setup errors also return Err.
/// Callers must authorize only on Ok(true), never on an error or Ok(false).
pub fn verify(public_key: &[u8], message: &[u8], signature: &[u8]) -> Result<bool, Error> {
    ffi::verify(public_key, message, signature)
}

// Crate-internal storage boundary only; no public raw getter/importer.
impl SecretKey {
    pub(crate) fn with_storage_bytes<R>(&self, f: impl FnOnce(&[u8]) -> R) -> R {
        f(self.0.as_slice())
    }
    pub(crate) fn from_storage(bytes: Vec<u8>) -> Result<Self, Error> {
        let owner = SecretBuffer::adopt_storage(bytes);
        super::error::check_length(
            "stored signature secret key",
            owner.as_slice().len(),
            SECRET_KEY_LENGTH,
        )?;
        Ok(Self(owner))
    }
}
