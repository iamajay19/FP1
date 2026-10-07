//! Phase 2A utilities. No PKI, provisioning, storage or trusted-time workflow.
mod error;
// Internal foundations intentionally await typed protocol callers in Phase 2B.
// Keeping them private avoids exporting raw secrets or a permissive codec API.
#[allow(dead_code)]
pub(crate) mod cbor;
#[allow(dead_code)]
pub(crate) mod crypto;
mod identifiers;
#[allow(dead_code)]
pub(crate) mod secret;

pub use error::Error;
pub use identifiers::{KemKeyId, RandomId, SignatureKeyId, kem_key_id, signature_key_id};

#[cfg(test)]
mod tests;
