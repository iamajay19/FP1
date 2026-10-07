use super::{Error, ffi};
use std::fmt;

/// One heap owner; no Clone/Deref/AsRef/Display or public byte export.
/// Fixed-size allocations are reserved before zero initialization and never resized.
pub(super) struct SecretBuffer {
    bytes: Vec<u8>,
}

impl SecretBuffer {
    pub(super) fn zeroed(length: usize) -> Result<Self, Error> {
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(length)
            .map_err(|_| Error::AllocationFailure)?;
        bytes.resize(length, 0);
        Ok(Self { bytes })
    }

    pub(super) fn as_slice(&self) -> &[u8] {
        &self.bytes
    }
    pub(super) fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.bytes
    }
}

impl fmt::Debug for SecretBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecretBuffer([REDACTED])")
    }
}

impl Drop for SecretBuffer {
    fn drop(&mut self) {
        // Cleanse the live allocation before Vec deallocates it, including partial
        // native outputs on failure. This cannot erase backend stack/register copies,
        // swap/dumps or buffers already copied by a privileged observer.
        ffi::cleanse(&mut self.bytes);
    }
}
