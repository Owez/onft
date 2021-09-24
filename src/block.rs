use crate::{Error, Hash, Result};
use openssl::pkey::{Id, PKey, Private, Public};

pub struct Block<'a> {
    pub previous_block: Option<&'a Block<'a>>,
    pub hash: Hash,
    pub ownership: Ownership,
    pub signature: Vec<u8>,
    pub data: Vec<u8>,
}

impl<'a> Block<'a> {
    pub fn new(
        previous_block: impl Into<Option<&'a Block<'a>>>,
        data: impl Into<Vec<u8>>,
    ) -> Result<Self> {
        let previous_block = previous_block.into();
        let previous_hash = match previous_block {
            Some(previous_block) => previous_block.hash.clone(),
            None => Hash::GENESIS,
        };
        let data = data.into();
        let (hash, signature, pkey) = Hash::new(&previous_hash, data.as_slice())?;
        Ok(Self {
            previous_block,
            hash,
            ownership: pkey.into(),
            signature,
            data,
        })
    }

    pub fn verify(&self) -> Result<bool> {
        match self.previous_block {
            Some(previous_block) => self.hash.verify(
                &previous_block.hash,
                self.signature.as_slice(),
                self.data.as_slice(),
                self.ownership.clone().to_public()?,
            ),
            None => Err(Error::NoPreviousBlock),
        }
    }
}

#[derive(Clone)]
pub enum Ownership {
    Them(PKey<Public>),
    Us(PKey<Private>),
}

impl Ownership {
    pub fn to_public(self) -> Result<PKey<Public>> {
        // TODO: check if this is right
        let public_conversion = |err| Error::PublicConversion(err);
        match self {
            Self::Them(pkey) => Ok(pkey),
            Self::Us(pkey) => PKey::public_key_from_raw_bytes(
                pkey.raw_public_key().map_err(public_conversion)?.as_slice(),
                Id::RSA,
            )
            .map_err(public_conversion),
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
