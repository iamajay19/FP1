use std::fmt;

/// Fixed public diagnostics. Never retain password, key, ciphertext or path bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidBundle,
    UnsupportedVersion,
    UnsupportedAlgorithm,
    InvalidKdfParameters,
    InvalidPassword,
    AuthenticationFailure,
    RandomnessFailure,
    AllocationFailure,
    SerializationFailure,
    PqcFailure,
    UnsafePath,
    AlreadyExists,
    IoFailure,
    /// Publication succeeded, but directory synchronization failed. Ciphertext
    /// may exist; callers must not assume the old file is still installed.
    PersistenceUncertain,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidBundle => "invalid key bundle",
            Self::UnsupportedVersion => "unsupported key bundle version",
            Self::UnsupportedAlgorithm => "unsupported key bundle algorithm",
            Self::InvalidKdfParameters => "invalid bounded Argon2id parameters",
            Self::InvalidPassword => "password must contain 1 to 4096 UTF-8 bytes",
            Self::AuthenticationFailure => "authentication failed",
            Self::RandomnessFailure => "OS randomness unavailable",
            Self::AllocationFailure => "allocation failed",
            Self::SerializationFailure => "key bundle serialization failed",
            Self::PqcFailure => "PQC operation failed",
            Self::UnsafePath => "unsafe key bundle path or permissions",
            Self::AlreadyExists => "key bundle already exists",
            Self::IoFailure => "key bundle I/O failed",
            Self::PersistenceUncertain => "encrypted bundle published; durability uncertain",
        })
    }
}
impl std::error::Error for Error {}
impl From<crate::foundation::Error> for Error {
    fn from(e: crate::foundation::Error) -> Self {
        match e {
            crate::foundation::Error::AllocationFailure => Self::AllocationFailure,
            _ => Self::InvalidBundle,
        }
    }
}
