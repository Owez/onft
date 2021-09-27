//! Contains crate-level errors and implementations
//!
//! # Structure
//!
//! The structure of this erroring system is simple and reflects implementations such as [std::io]'s structure but less complex. It is structured in a high-level sense as such:
//!
//! - Abstract library error: [Error]
//!     - Whilst signing a block: [SignerError]
//!     - Whilst verifying a block: [VerifierError]
//! - Module result wrapper type: [Result]

#[cfg(feature = "serde")]
use crate::PROTO_VERSION;
use openssl::error::ErrorStack;
use std::fmt;

/// Error variants, describing possible errors which may occur within this crate
#[allow(missing_docs)]
#[derive(Debug)]
pub enum Error {
    Signer(SignerError),
    Verifier(VerifierError),
    KeyGen(ErrorStack),
    GenesisIsNotKey,
    #[cfg(feature = "serde")]
    IncompatibleVersion(u8),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Signer(err) => write!(f, "{}", err),
            Error::Verifier(err) => write!(f, "{}", err),
            Error::KeyGen(err) => write!(f, "Couldn't generate new ED25519 keypair ({})", err),
            Error::GenesisIsNotKey => write!(
                f,
                "Genesis block's don't contain pkeys but it was queried for"
            ),
            #[cfg(feature = "serde")]
            Error::IncompatibleVersion(found) => write!(
                f,
                "Inputted protocol version (v{}) doesn't match ours (v{})",
                found, PROTO_VERSION
            ),
        }
    }
}

impl From<Error> for () {
    fn from(_: Error) -> Self {}
}

/// Errors related to the creation of block signatures contained within hashes
#[allow(missing_docs)]
#[derive(Debug)]
pub enum SignerError {
    Create(ErrorStack),
    Update(ErrorStack),
    Execute(ErrorStack),
}

impl From<SignerError> for Error {
    fn from(err: SignerError) -> Self {
        Self::Signer(err)
    }
}

impl fmt::Display for SignerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignerError::Create(err) => {
                write!(f, "Couldn't create signer to create a new hash ({})", err)
            }
            SignerError::Update(err) => write!(
                f,
                "Couldn't update signer with data to create a new hash ({})",
                err
            ),
            SignerError::Execute(err) => {
                write!(f, "Couldn't execute signer to create a new hash ({})", err)
            }
        }
    }
}

impl From<SignerError> for () {
    fn from(_: SignerError) -> Self {}
}

/// Errors related to verification of hashes and block signatures
#[allow(missing_docs)]
#[derive(Debug)]
pub enum VerifierError {
    Create(ErrorStack),
    Update(ErrorStack),
    Execute(ErrorStack),
}

impl From<VerifierError> for Error {
    fn from(err: VerifierError) -> Self {
        Self::Verifier(err)
    }
}

impl fmt::Display for VerifierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VerifierError::Create(err) => write!(f, "Couldn't create block verifier ({})", err),
            VerifierError::Update(err) => {
                write!(f, "Couldn't update block verifier with hash ({})", err)
            }
            VerifierError::Execute(err) => write!(f, "Couldn't execute block verifier ({})", err),
        }
    }
}

impl From<VerifierError> for () {
    fn from(_: VerifierError) -> Self {}
}

/// Type alias for results containing crate-based errors
pub type Result<T> = std::result::Result<T, Error>;

impl<T> From<SignerError> for Result<T> {
    fn from(err: SignerError) -> Self {
        Result::Err(err.into())
    }
}

impl<T> From<VerifierError> for Result<T> {
    fn from(err: VerifierError) -> Self {
        Result::Err(err.into())
    }
}

/// Serde testing
#[cfg(feature = "serde")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incompatible_version() {
        assert_eq!(
            format!("{}", Error::IncompatibleVersion(20)),
            format!(
                "Inputted protocol version (v20) doesn't match ours (v{})",
                PROTO_VERSION
            )
        )
    }
}
