use std::fmt;

/// Fixed diagnostics only; never includes input bytes or secret material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    MalformedCbor,
    ForbiddenCbor,
    NonCanonical,
    Schema,
    DuplicateField,
    UnknownField,
    TrailingBytes,
    SizeLimit,
    DepthLimit,
    ItemLimit,
    AllocationFailure,
    IncorrectKeyLength,
    RandomnessUnavailable,
    DerivationFailure,
    AuthenticationFailure,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::MalformedCbor => "malformed CBOR",
            Self::ForbiddenCbor => "forbidden CBOR type",
            Self::NonCanonical => "noncanonical CBOR",
            Self::Schema => "invalid schema",
            Self::DuplicateField => "duplicate field",
            Self::UnknownField => "unknown field",
            Self::TrailingBytes => "trailing bytes",
            Self::SizeLimit => "size limit exceeded",
            Self::DepthLimit => "nesting limit exceeded",
            Self::ItemLimit => "item limit exceeded",
            Self::AllocationFailure => "allocation failed",
            Self::IncorrectKeyLength => "incorrect public key length",
            Self::RandomnessUnavailable => "OS randomness unavailable",
            Self::DerivationFailure => "key derivation failed",
            Self::AuthenticationFailure => "authentication failed",
        };
        f.write_str(message)
    }
}
impl std::error::Error for Error {}
