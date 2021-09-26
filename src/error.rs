use openssl::error::ErrorStack;
use std::fmt;

/// Error variants, describing possible errors which may occur within this crate
#[allow(missing_docs)]
#[derive(Debug)]
pub enum Error {
    Signer(SignerError),
    Verifier(VerifierError),
    KeyGen(ErrorStack),
    NoPreviousBlock,
    PublicConversion(ErrorStack),
    GenesisIsNotKey,
}

impl fmt::Display for Error {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
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
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
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
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
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
