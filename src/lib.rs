mod hash;

pub use hash::Hash;

use openssl::error::ErrorStack;

#[derive(Debug)]
pub enum Error {
    Signer(SignerError),
    KeyGen(ErrorStack),
}

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

pub type Result<T> = std::result::Result<T, Error>;

impl<T> From<SignerError> for Result<T> {
    fn from(err: SignerError) -> Self {
        Result::Err(err.into())
    }
}
