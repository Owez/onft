use crate::{Error, Hash, Result};
use openssl::pkey::{Id, PKey, Private, Public};

pub struct Block {
    pub hash: Hash,
    pub ownership: Ownership,
    pub signature: Vec<u8>,
    pub data: Vec<u8>,
}

impl<'a> Block {
    pub fn new(previous_hash: impl Into<&'a Hash>, data: impl Into<Vec<u8>>) -> Result<Self> {
        let data = data.into();
        let (hash, signature, pkey) = Hash::new(previous_hash, data.as_slice())?;
        Ok(Self {
            hash,
            ownership: pkey.into(),
            signature,
            data,
        })
    }

    pub fn verify(&self, previous_hash: impl Into<&'a Hash>) -> Result<bool> {
        self.hash.verify(
            previous_hash.into(),
            self.signature.as_slice(),
            self.data.as_slice(),
            self.ownership.clone().into_public()?,
        )
    }
}

impl Default for Block {
    fn default() -> Self {
        todo!()
    }
}

#[derive(Clone)]
pub enum Ownership {
    Them(PKey<Public>),
    Us(PKey<Private>),
}

impl Ownership {
    pub fn into_public(self) -> Result<PKey<Public>> {
        // TODO: check if this is right
        match self {
            Self::Them(pkey) => Ok(pkey),
            Self::Us(pkey) => PKey::public_key_from_raw_bytes(
                pkey.raw_public_key()
                    .map_err(Error::PublicConversion)?
                    .as_slice(),
                Id::RSA,
            )
            .map_err(Error::PublicConversion),
        }
    }
}

impl From<PKey<Public>> for Ownership {
    fn from(pkey: PKey<Public>) -> Self {
        Self::Them(pkey)
    }
}

impl From<PKey<Private>> for Ownership {
    fn from(pkey: PKey<Private>) -> Self {
        Self::Us(pkey)
    }
}
