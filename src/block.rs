use crate::{Error, Hash, Result};
use openssl::pkey::{PKey, Private, Public};

/// Single block within a larger blockchain, providing access to a block of data
///
/// # Example
///
/// TODO: example
#[derive(Debug, Clone)]
pub struct Block {
    /// The hash of this block.
    pub hash: Hash,
    /// Ownership identifier, represents if we own it or not.
    pub ownership: Ownership,
    /// Signature which wraps data into a key to verify ownership.
    pub signature: [u8; Hash::SIG_LEN],
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
        let previous_hash = previous_hash.into();
        let data = self.data.as_slice();

        match &self.ownership {
            Ownership::Them(pkey) => self.hash.verify(previous_hash, self.signature, data, pkey),
            Ownership::Us(pkey) => self.hash.verify(previous_hash, self.signature, data, pkey),
            Ownership::Genesis => Err(Error::GenesisIsNotKey),
        }
    }
}

impl Default for Block {
    /// Creates default genesis block.
    fn default() -> Self {
        Self {
            hash: Hash::default(),
            ownership: Ownership::Genesis,
            signature: [0; Hash::SIG_LEN],
            data: vec![],
        }
    }
}

/// Contains ownership keys and information for a given block
#[derive(Debug, Clone)]
pub enum Ownership {
    /// Owned by an external source as we have a general public key.
    Them(PKey<Public>),
    /// Owned by us as we have a private key.
    Us(PKey<Private>),
    /// Special genesis ownership type as the genesis block is owned by nobody.
    Genesis,
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
