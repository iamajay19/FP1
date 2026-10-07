//! Standardized algorithms with fixed roles and a private native boundary.

mod error;
#[allow(unsafe_code)]
mod ffi;
pub mod kem;
mod secret;
pub mod signature;

pub use error::Error;

/// Public backend metadata, checked once against the linked native implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendInfo {
    pub version: &'static str,
    pub source_commit: &'static str,
    pub kem_algorithm: &'static str,
    pub signature_algorithm: &'static str,
    /// KEM public/secret/ciphertext/shared, signature public/secret/signature sizes.
    pub sizes: [usize; 7],
    pub empty_context_supported: bool,
}

/// Initialize/check the process-lifetime backend, or return its cached failure.
/// Never destroys global state while another caller may be using it.
pub fn backend_info() -> Result<&'static BackendInfo, Error> {
    ffi::backend_info()
}

#[cfg(test)]
mod tests;
