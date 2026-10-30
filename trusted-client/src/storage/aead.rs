use super::{
    Error,
    schema::{self, Header, Protected, Role},
    secret::SecretBytes,
};
use aes_gcm::{Aes256Gcm, KeyInit, Nonce, Tag, aead::AeadInOut};

pub(super) fn protect(
    key: &SecretBytes,
    nonce: &[u8; 12],
    header: &Header<'_>,
    role: Role,
    plaintext: &[u8],
) -> Result<Vec<u8>, Error> {
    if plaintext.len() > 4032 || key.borrow().len() != 32 {
        return Err(Error::InvalidBundle);
    }
    let aad = schema::aad(header, role)?;
    let length = plaintext
        .len()
        .checked_add(16)
        .ok_or(Error::InvalidBundle)?;
    let mut buffer = SecretBytes::zeroed(length, "encrypt-staging")?;
    let (message, tag_output) = buffer.borrow_mut().split_at_mut(plaintext.len());
    message.copy_from_slice(plaintext);
    let cipher =
        Aes256Gcm::new_from_slice(key.borrow()).map_err(|_| Error::AuthenticationFailure)?;
    let tag = cipher
        .encrypt_inout_detached(&Nonce::from(*nonce), &aad, message.into())
        .map_err(|_| Error::AuthenticationFailure)?;
    tag_output.copy_from_slice(&tag);
    // Ownership may leave the secret guard only after complete encryption.
    Ok(buffer.into_vec())
}
pub(super) fn open(
    key: &SecretBytes,
    entry: Protected<'_>,
    header: &Header<'_>,
    role: Role,
) -> Result<SecretBytes, Error> {
    if entry.nonce.len() != 12 || ![2416, 4048].contains(&entry.ciphertext_and_tag.len()) {
        return Err(Error::InvalidBundle);
    }
    let nonce: [u8; 12] = entry.nonce.try_into().map_err(|_| Error::InvalidBundle)?;
    let n = entry
        .ciphertext_and_tag
        .len()
        .checked_sub(16)
        .ok_or(Error::InvalidBundle)?;
    let (ciphertext, tag) = entry.ciphertext_and_tag.split_at(n);
    let tag: [u8; 16] = tag.try_into().map_err(|_| Error::InvalidBundle)?;
    let aad = schema::aad(header, role)?;
    let mut output = SecretBytes::zeroed(
        n,
        match role {
            Role::Kem => "kem-plaintext",
            Role::Signature => "signature-plaintext",
        },
    )?;
    output.borrow_mut().copy_from_slice(ciphertext);
    let cipher =
        Aes256Gcm::new_from_slice(key.borrow()).map_err(|_| Error::AuthenticationFailure)?;
    cipher
        .decrypt_inout_detached(
            &Nonce::from(nonce),
            &aad,
            output.borrow_mut().into(),
            &Tag::from(tag),
        )
        .map_err(|_| Error::AuthenticationFailure)?;
    let expected = match role {
        Role::Kem => 2400,
        Role::Signature => 4032,
    };
    if output.borrow().len() != expected {
        return Err(Error::AuthenticationFailure);
    }
    Ok(output)
}
