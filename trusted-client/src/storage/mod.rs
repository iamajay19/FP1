//! Phase 2B password-protected private-key storage. No PKI or document workflow.
mod error;
mod kdf;
mod secret;
pub use error::Error;
pub use kdf::{KdfParameters, benchmark_argon2_profile};
pub use secret::Password;
mod aead;
mod persistence;
mod schema;
pub use kdf::creation_parameters;
pub use persistence::{Persistence, persist, unlock_file};

use crate::{
    foundation::{KemKeyId, SignatureKeyId, kem_key_id, signature_key_id},
    pqc::{kem, signature},
};
use schema::{Bundle, Header, Kdf, Protected, Role};
use std::fmt;
use zeroize::Zeroize;

/// Expected public binding supplied by the trusted caller, never inferred from
/// the untrusted bundle. No certificates or provisioning authority are involved.
pub struct Identity<'a> {
    recipient_id: [u8; 32],
    kem_public_key: &'a [u8],
    signature_public_key: &'a [u8],
    kem_id: KemKeyId,
    signature_id: SignatureKeyId,
}
impl<'a> Identity<'a> {
    pub fn new(
        recipient_id: [u8; 32],
        kem_public_key: &'a [u8],
        signature_public_key: &'a [u8],
    ) -> Result<Self, Error> {
        Ok(Self {
            recipient_id,
            kem_id: kem_key_id(kem_public_key)?,
            signature_id: signature_key_id(signature_public_key)?,
            kem_public_key,
            signature_public_key,
        })
    }
}
/// Contains ciphertext only; constructed by successful private-key protection.
pub struct EncryptedBundle {
    bytes: Vec<u8>,
}
impl EncryptedBundle {
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}
impl fmt::Debug for EncryptedBundle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("EncryptedBundle([CIPHERTEXT])")
    }
}
/// Distinct audited key owners; no raw exports, Clone or combined plaintext blob.
#[derive(Debug)]
pub struct UnlockedKeys {
    pub kem: kem::SecretKey,
    pub signature: signature::SecretKey,
}

fn check_keys(
    identity: &Identity<'_>,
    kem: &kem::SecretKey,
    sig: &signature::SecretKey,
) -> Result<(), Error> {
    let (ct, expected) =
        kem::encapsulate(identity.kem_public_key).map_err(|_| Error::PqcFailure)?;
    let recovered = kem::decapsulate(kem, &ct).map_err(|_| Error::PqcFailure)?;
    if expected != recovered {
        return Err(Error::AuthenticationFailure);
    }
    // Local consistency test only: random public message, never a protocol object.
    let mut challenge = [0u8; 32];
    random(&mut challenge)?;
    let proof = signature::sign(sig, &challenge).map_err(|_| Error::PqcFailure)?;
    if !signature::verify(identity.signature_public_key, &challenge, &proof)
        .map_err(|_| Error::PqcFailure)?
    {
        return Err(Error::AuthenticationFailure);
    }
    Ok(())
}
fn random(out: &mut [u8]) -> Result<(), Error> {
    if getrandom::fill(out).is_err() {
        out.zeroize();
        return Err(Error::RandomnessFailure);
    }
    Ok(())
}
pub fn create_bundle(
    password: &Password,
    identity: &Identity<'_>,
    kem: &kem::SecretKey,
    signature: &signature::SecretKey,
) -> Result<EncryptedBundle, Error> {
    check_keys(identity, kem, signature)?;
    create_with_rng(
        password,
        identity,
        kem,
        signature,
        creation_parameters(),
        &mut random,
    )
}
fn create_with_rng(
    password: &Password,
    identity: &Identity<'_>,
    kem: &kem::SecretKey,
    signature: &signature::SecretKey,
    costs: KdfParameters,
    fill: &mut impl FnMut(&mut [u8]) -> Result<(), Error>,
) -> Result<EncryptedBundle, Error> {
    let mut salt = [0; 16];
    let mut kn = [0; 12];
    let mut sn = [0; 12];
    fill(&mut salt)?;
    fill(&mut kn)?;
    fill(&mut sn)?;
    if kn == sn {
        return Err(Error::RandomnessFailure);
    }
    let header = Header {
        schema_version: 1,
        recipient_id: &identity.recipient_id,
        kem_algorithm: "ML-KEM-768",
        kem_key_id: identity.kem_id.as_bytes(),
        signature_algorithm: "ML-DSA-65",
        signature_key_id: identity.signature_id.as_bytes(),
        aead_algorithm: "AES-256-GCM",
        kdf: Kdf {
            algorithm: "Argon2id",
            version: 19,
            memory_kib: u64::from(costs.memory_kib),
            time_cost: u64::from(costs.time_cost),
            parallelism: u64::from(costs.parallelism),
            output_len: 32,
            salt: &salt,
        },
    };
    header.validate()?;
    let kek = kdf::derive(password.borrow(), &salt, costs)?;
    let kc = kem.with_storage_bytes(|bytes| aead::protect(&kek, &kn, &header, Role::Kem, bytes))?;
    let sc = signature
        .with_storage_bytes(|bytes| aead::protect(&kek, &sn, &header, Role::Signature, bytes))?;
    #[cfg(test)]
    tests::serialization_fault()?;
    let bytes = schema::encode(&Bundle {
        header,
        kem: Protected {
            nonce: &kn,
            ciphertext_and_tag: &kc,
        },
        signature: Protected {
            nonce: &sn,
            ciphertext_and_tag: &sc,
        },
    })?;
    Ok(EncryptedBundle { bytes })
}
/// Strict canonical parsing and bounded KDF precede AEAD. Authorization requires
/// BOTH authenticated entries plus the trusted recipient/public-key binding.
pub fn unlock_bundle(
    bytes: &[u8],
    password: &Password,
    identity: &Identity<'_>,
) -> Result<UnlockedKeys, Error> {
    let bundle = schema::parse(bytes)?;
    let costs = bundle.header.validate()?;
    let kek = kdf::derive(password.borrow(), bundle.header.kdf.salt, costs)?;
    let kem_plain = aead::open(&kek, bundle.kem, &bundle.header, Role::Kem)?;
    let signature_plain = aead::open(&kek, bundle.signature, &bundle.header, Role::Signature)?;
    // Both live plaintext owners clean up if any later binding check fails.
    if bundle.header.recipient_id != identity.recipient_id
        || bundle.header.kem_key_id != identity.kem_id.as_bytes()
        || bundle.header.signature_key_id != identity.signature_id.as_bytes()
    {
        return Err(Error::AuthenticationFailure);
    }
    let keys = UnlockedKeys {
        kem: kem::SecretKey::from_storage(kem_plain.into_vec())
            .map_err(|_| Error::AuthenticationFailure)?,
        signature: signature::SecretKey::from_storage(signature_plain.into_vec())
            .map_err(|_| Error::AuthenticationFailure)?,
    };
    check_keys(identity, &keys.kem, &keys.signature).map_err(|e| match e {
        Error::RandomnessFailure => e,
        _ => Error::AuthenticationFailure,
    })?;
    Ok(keys)
}
#[cfg(test)]
mod tests;
