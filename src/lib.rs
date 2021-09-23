mod hash;

pub use hash::Hash;

use openssl::error::ErrorStack;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    SignerCreate(ErrorStack),
    SignerUpdate(ErrorStack),
    SignerExec(ErrorStack),
    KeyGen(ErrorStack),
}
