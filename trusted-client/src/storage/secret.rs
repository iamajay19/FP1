//! Fixed allocation ownership after secrets exist. Normal Drop only; no physical
//! erasure, terminal-copy, process-abort or compiler/register-copy guarantee.
use super::Error;
use std::fmt;
use zeroize::Zeroize;

pub(super) struct SecretBytes {
    bytes: Vec<u8>,
    #[cfg(test)]
    kind: &'static str,
}
impl SecretBytes {
    pub(super) fn zeroed(length: usize, _kind: &'static str) -> Result<Self, Error> {
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(length)
            .map_err(|_| Error::AllocationFailure)?;
        bytes.resize(length, 0);
        Ok(Self {
            bytes,
            #[cfg(test)]
            kind: _kind,
        })
    }
    fn password(value: String) -> Self {
        // String::into_bytes transfers the allocation; it does not clone it.
        Self {
            bytes: value.into_bytes(),
            #[cfg(test)]
            kind: "password",
        }
    }
    pub(super) fn borrow(&self) -> &[u8] {
        &self.bytes
    }
    pub(super) fn borrow_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }
    /// Internal ownership transfer only: encrypted output or immediate adoption
    /// by the existing PQC secret owner. Never a public secret-export API.
    pub(super) fn into_vec(mut self) -> Vec<u8> {
        std::mem::take(&mut self.bytes)
    }
}
impl fmt::Debug for SecretBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecretBytes([REDACTED])")
    }
}
impl Drop for SecretBytes {
    fn drop(&mut self) {
        self.bytes.as_mut_slice().zeroize();
        #[cfg(test)]
        super::tests::observe_drop(self.kind, &self.bytes);
    }
}

/// Passwords have no Clone, Display, dereference or public byte constructor/getter.
/// The production source is rpassword's protected /dev/tty input only.
/// ```compile_fail
/// use cadp_pqc::storage::Password;
/// let password = Password::read_from_terminal().unwrap();
/// let copied = password.clone();
/// ```
/// ```compile_fail
/// use cadp_pqc::storage::Password;
/// let password = Password::read_from_terminal().unwrap();
/// let bytes = password.as_bytes();
/// ```
pub struct Password(SecretBytes);
impl Password {
    pub fn read_from_terminal() -> Result<Self, Error> {
        let input =
            rpassword::prompt_password("Bundle password: ").map_err(|_| Error::IoFailure)?;
        Self::take_string(input)
    }
    fn take_string(input: String) -> Result<Self, Error> {
        let owner = SecretBytes::password(input);
        if owner.borrow().is_empty() || owner.borrow().len() > 4096 {
            return Err(Error::InvalidPassword);
        }
        Ok(Self(owner))
    }
    pub(super) fn borrow(&self) -> &[u8] {
        self.0.borrow()
    }
    #[cfg(test)]
    pub(super) fn take_string_for_test(input: String) -> Result<Self, Error> {
        Self::take_string(input)
    }
    #[cfg(test)]
    pub(super) fn fixture(input: &str) -> Self {
        Self::take_string(input.to_owned()).unwrap()
    }
}
impl fmt::Debug for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Password([REDACTED])")
    }
}
