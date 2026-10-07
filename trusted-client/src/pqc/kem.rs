//! ML-KEM-768 only. Decapsulation success is not recipient authentication.

use super::{Error, error::check_length, ffi, secret::SecretBuffer};
use std::fmt;

pub const PUBLIC_KEY_LENGTH: usize = 1184;
pub const SECRET_KEY_LENGTH: usize = 2400;
pub const CIPHERTEXT_LENGTH: usize = 1088;
pub const SHARED_SECRET_LENGTH: usize = 32;

/// Generated KEM key material, distinct from signing keys; no public byte export.
///
/// ```compile_fail
/// use cadp_pqc::pqc::kem;
/// let (_, secret) = kem::generate_keypair().unwrap();
/// let bytes = secret.as_bytes();
/// ```
/// ```compile_fail
/// use cadp_pqc::pqc::kem;
/// let (_, secret) = kem::generate_keypair().unwrap();
/// let copy = secret.clone();
/// ```
pub struct SecretKey(pub(super) SecretBuffer);

/// Shared secret remains internal; future HKDF integration belongs to Phase 3.
pub struct SharedSecret(SecretBuffer);

impl fmt::Debug for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("KemSecretKey([REDACTED])")
    }
}

impl fmt::Debug for SharedSecret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SharedSecret([REDACTED])")
    }
}

impl PartialEq for SharedSecret {
    fn eq(&self, other: &Self) -> bool {
        ffi::secret_equal(self.0.as_slice(), other.0.as_slice())
    }
}
impl Eq for SharedSecret {}

/// Generate an independent ephemeral key pair using liboqs's OS CSPRNG.
pub fn generate_keypair() -> Result<([u8; PUBLIC_KEY_LENGTH], SecretKey), Error> {
    let mut public = [0u8; PUBLIC_KEY_LENGTH];
    let mut secret = SecretBuffer::zeroed(SECRET_KEY_LENGTH)?;
    ffi::kem_keypair(&mut public, secret.as_mut_slice())?;
    Ok((public, SecretKey(secret)))
}

pub fn encapsulate(public_key: &[u8]) -> Result<([u8; CIPHERTEXT_LENGTH], SharedSecret), Error> {
    check_length("KEM public key", public_key.len(), PUBLIC_KEY_LENGTH)?;
    let mut ciphertext = [0u8; CIPHERTEXT_LENGTH];
    let mut shared = SecretBuffer::zeroed(SHARED_SECRET_LENGTH)?;
    ffi::encapsulate(public_key, &mut ciphertext, shared.as_mut_slice())?;
    Ok((ciphertext, SharedSecret(shared)))
}

/// Correctly sized wrong-key/tampered inputs may return an unrelated secret due
/// to ML-KEM implicit rejection. Phase 3 CEK-wrap authentication must reject it.
pub fn decapsulate(secret_key: &SecretKey, ciphertext: &[u8]) -> Result<SharedSecret, Error> {
    check_length("KEM ciphertext", ciphertext.len(), CIPHERTEXT_LENGTH)?;
    let mut shared = SecretBuffer::zeroed(SHARED_SECRET_LENGTH)?;
    ffi::decapsulate(secret_key.0.as_slice(), ciphertext, shared.as_mut_slice())?;
    Ok(SharedSecret(shared))
}
