//! Fixed SHA-256/HKDF/HMAC utilities, internal until protocol-specific callers exist.
use super::{Error, secret::Secret32};
use hkdf::Hkdf;
use hmac::{Hmac, KeyInit, Mac};
use sha2::{Digest, Sha256};
use zeroize::Zeroize;

pub(crate) fn sha256(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}

/// Caller supplies protocol-defined salt/info. Both RFC 5869 stages, always 32 bytes.
/// Explicit PRK is wiped; upstream expand's temporary output copies remain a limitation.
pub(crate) fn hkdf_sha256(ikm: &[u8], salt: &[u8], info: &[u8]) -> Result<Secret32, Error> {
    let mut output = Secret32::zeroed()?;
    let (mut prk, hk) = Hkdf::<Sha256>::extract(Some(salt), ikm);
    prk.as_mut_slice().zeroize();
    hk.expand(info, output.borrow_mut())
        .map_err(|_| Error::DerivationFailure)?;
    Ok(output)
}

/// Full public confirmation tag; no equality trait used for authentication.
pub(crate) struct Tag32([u8; 32]);
impl Tag32 {
    pub(crate) fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}
fn mac(key: &[u8], message: &[u8]) -> Result<Hmac<Sha256>, Error> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key).map_err(|_| Error::DerivationFailure)?;
    mac.update(message);
    Ok(mac)
}
pub(crate) fn hmac_sha256(key: &Secret32, message: &[u8]) -> Result<Tag32, Error> {
    Ok(Tag32(
        mac(key.borrow(), message)?.finalize().into_bytes().into(),
    ))
}
pub(crate) fn verify_hmac_sha256(key: &Secret32, message: &[u8], tag: &[u8]) -> Result<(), Error> {
    if tag.len() != 32 {
        return Err(Error::AuthenticationFailure);
    }
    mac(key.borrow(), message)?
        .verify_slice(tag)
        .map_err(|_| Error::AuthenticationFailure)
}
