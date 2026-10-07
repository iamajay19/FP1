use std::fmt;

/// Errors contain public sizes/operation labels only, never buffer contents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    BackendUnavailable,
    UnexpectedBackendMetadata,
    IncorrectInputLength {
        input: &'static str,
        expected: usize,
        actual: usize,
    },
    AllocationFailure,
    KeyGenerationFailure {
        algorithm: &'static str,
    },
    EncapsulationFailure,
    DecapsulationFailure,
    SigningFailure,
    MalformedSignature {
        actual: usize,
    },
    UnexpectedSignatureLength {
        actual: usize,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BackendUnavailable => f.write_str("required liboqs backend unavailable"),
            Self::UnexpectedBackendMetadata => {
                f.write_str("unexpected liboqs metadata or contract")
            }
            Self::IncorrectInputLength {
                input,
                expected,
                actual,
            } => write!(
                f,
                "incorrect {input} length: expected {expected}, received {actual}"
            ),
            Self::AllocationFailure => f.write_str("PQC buffer or native allocation failed"),
            Self::KeyGenerationFailure { algorithm } => {
                write!(f, "{algorithm} key generation failed")
            }
            Self::EncapsulationFailure => f.write_str("ML-KEM-768 encapsulation failed"),
            Self::DecapsulationFailure => f.write_str("ML-KEM-768 backend decapsulation failed"),
            Self::SigningFailure => f.write_str("ML-DSA-65 signing failed"),
            Self::MalformedSignature { actual } => {
                write!(f, "malformed ML-DSA-65 signature length: {actual}")
            }
            Self::UnexpectedSignatureLength { actual } => {
                write!(f, "unexpected backend signature length: {actual}")
            }
        }
    }
}

impl std::error::Error for Error {}

pub(super) fn check_length(
    input: &'static str,
    actual: usize,
    expected: usize,
) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(Error::IncorrectInputLength {
            input,
            expected,
            actual,
        })
    }
}
