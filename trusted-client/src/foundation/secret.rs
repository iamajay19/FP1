//! Normal-Drop cleanup only; stack/register/library copies and fatal exits remain.
use super::Error;
use std::fmt;
use zeroize::Zeroize;

/// Fixed-size heap output owner. No Clone, Display, Deref or public raw getter.
#[cfg(test)]
type Observer = Box<dyn Fn(&[u8])>;

pub(crate) struct Secret32 {
    bytes: Box<[u8; 32]>,
    #[cfg(test)]
    pub(super) observer: Option<Observer>,
}
impl Secret32 {
    pub(crate) fn zeroed() -> Result<Self, Error> {
        let mut v = Vec::new();
        v.try_reserve_exact(32)
            .map_err(|_| Error::AllocationFailure)?;
        v.resize(32, 0);
        // Any shrink/copy occurs before sensitive bytes exist.
        let bytes = v
            .into_boxed_slice()
            .try_into()
            .map_err(|_| Error::AllocationFailure)?;
        Ok(Self {
            bytes,
            #[cfg(test)]
            observer: None,
        })
    }
    pub(crate) fn borrow(&self) -> &[u8] {
        &self.bytes[..]
    }
    pub(crate) fn borrow_mut(&mut self) -> &mut [u8] {
        &mut self.bytes[..]
    }
}
impl fmt::Debug for Secret32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Secret32([REDACTED])")
    }
}
impl Drop for Secret32 {
    fn drop(&mut self) {
        self.bytes.as_mut().zeroize();
        #[cfg(test)]
        if let Some(observe) = &self.observer {
            observe(&self.bytes[..]);
        }
    }
}
