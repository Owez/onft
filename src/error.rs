use openssl::error::ErrorStack;

#[derive(Debug)]
pub enum Error {
    Signer(SignerError),
    Verifier(VerifierError),
    KeyGen(ErrorStack),
    NoPreviousBlock,
    PublicConversion(ErrorStack),
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