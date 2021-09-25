use crate::{Error, Hash, Result};
use openssl::pkey::{Id, PKey, Private, Public};

#[derive(Debug, Clone)]
pub struct Block {
    /// The hash of this block.
    pub hash: Hash,
    /// Ownership identifier, represents if we own it or not.
    pub ownership: Ownership,
    /// Signature which wraps data into a key to verify ownership.
    pub signature: Vec<u8>,
    /// Underlying data contained within this block.
    pub data: Vec<u8>,
}

impl<'a> Block {
    /// Creates a new block from the previous block in a chain alongside the data
    /// contained within this block.
    ///
    /// # Example
    ///
    /// TODO: example
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

    /// Verifies this individual block based upon the known hash of the last block.
    ///
    /// # Example
    ///
    /// TODO: example
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
    /// Creates default genesis block.
    fn default() -> Self {
        Self {
            hash: Hash::default(),
            ownership: Ownership::Genesis,
            signature: vec![],
            data: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub enum Ownership {
    Them(PKey<Public>),
    Us(PKey<Private>),
    /// Special genesis ownership type as the genesis block is owned by nobody.
    Genesis,
}

impl Ownership {
    /// Attempts to convert this enumeration into a public key; ensure this
    /// isn't ran on the genesis block.
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
            Self::Genesis => Err(Error::GenesisIsNotKey),
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
