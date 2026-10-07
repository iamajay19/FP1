use super::{
    Error,
    cbor::{self, Bytes, Class, Domain, Separated},
    crypto::sha256,
};
use crate::pqc::{kem, signature};
use zeroize::Zeroize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KemKeyId([u8; 32]);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignatureKeyId([u8; 32]);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RandomId([u8; 32]);
impl KemKeyId {
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}
impl SignatureKeyId {
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}
impl RandomId {
    /// OS CSPRNG only. Partial failure output is discarded; there is no fallback.
    pub fn generate() -> Result<Self, Error> {
        random_with(getrandom::fill)
    }
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}
// Private injection point shared with tests, never selectable by API callers.
fn random_with(
    fill: impl FnOnce(&mut [u8]) -> Result<(), getrandom::Error>,
) -> Result<RandomId, Error> {
    let mut bytes = [0u8; 32];
    if fill(&mut bytes).is_err() {
        bytes.zeroize();
        return Err(Error::RandomnessUnavailable);
    }
    Ok(RandomId(bytes))
}
pub fn kem_key_id(public_key: &[u8]) -> Result<KemKeyId, Error> {
    if public_key.len() != kem::PUBLIC_KEY_LENGTH {
        return Err(Error::IncorrectKeyLength);
    }
    Ok(KemKeyId(identifier(Domain::KemId, public_key)?))
}
pub fn signature_key_id(public_key: &[u8]) -> Result<SignatureKeyId, Error> {
    if public_key.len() != signature::PUBLIC_KEY_LENGTH {
        return Err(Error::IncorrectKeyLength);
    }
    Ok(SignatureKeyId(identifier(Domain::SignatureId, public_key)?))
}
fn identifier(domain: Domain, public_key: &[u8]) -> Result<[u8; 32], Error> {
    let bytes = cbor::deterministic(
        &Separated {
            domain,
            body: &Bytes(public_key),
        },
        Class::Small,
    )?;
    Ok(sha256(&bytes))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p2a_rng_002_partial_failure_no_fallback() {
        let result = random_with(|out| {
            out.fill(0x5a);
            Err(getrandom::Error::UNSUPPORTED)
        });
        assert_eq!(result, Err(Error::RandomnessUnavailable));
    }
    #[test]
    fn p2a_id_002_domain_separation() {
        assert_ne!(
            identifier(Domain::KemId, b"public test").unwrap(),
            identifier(Domain::SignatureId, b"public test").unwrap()
        );
    }
}
