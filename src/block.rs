//! Contains [Block], [Ownership] and implementations

use crate::{error::Error, Hash, Result};
use openssl::pkey::{PKey, Private, Public};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Single block within a larger blockchain, providing access to a block of data
///
/// # Using
///
/// You can in high level terms do the following directly to a block:
///
/// - Create a genesis block: [Block::default]
/// - Create a block containing data: [Block::new]
/// - Verify a block: [Block::verify]
///
/// # Example
///
/// ```rust
/// use onft::Block;
///
/// fn main() -> onft::Result<()> {
///     let genesis_block = Block::default();
///
///     let data = "Hello, world!";
///     let new_block = Block::new(&genesis_block.hash, data)?;
///     let verified = new_block.verify(&genesis_block.hash)?;
///
///     if verified {
///         println!("Verified")
///     } else {
///         eprintln!("Not verified")
///     }
///     Ok(())
/// }
/// ```
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
    /// ```rust
    /// use onft::Block;
    ///
    /// fn main() -> onft::Result<()> {
    ///     let genesis_block = Block::default();
    ///
    ///     let data = "Hello, world!";
    ///     let block = Block::new(&genesis_block.hash, data)?;
    ///
    ///     println!("Block:\n{:?}", block);
    ///     Ok(())
    /// }
    /// ```
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
    /// ```rust
    /// use onft::Block;
    ///
    /// fn main() -> onft::Result<()> {
    ///     let genesis_block = Block::default();
    ///
    ///     let data = "Hello, world!";
    ///     let new_block = Block::new(&genesis_block.hash, data)?;
    ///     let verified = new_block.verify(&genesis_block.hash)?;
    ///
    ///     if verified {
    ///         println!("Verified")
    ///     } else {
    ///         eprintln!("Not verified")
    ///     }
    ///     Ok(())
    /// }
    /// ```
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
    /// Special genesis ownership type as the genesis block is owned by nobody.
    Genesis,
    /// Owned by an external source as we have a general public key.
    Them(PKey<Public>),
    /// Owned by us as we have a private key.
    Us(PKey<Private>),
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

#[cfg(feature = "serde")]
impl Serialize for Ownership {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        const NAME: &str = "Ownership";
        let ser_err = |msg| serde::ser::Error::custom(msg);
        match self {
            Ownership::Genesis => serializer.serialize_unit_variant(NAME, 0, "Genesis"),
            Ownership::Them(pkey) => serializer.serialize_newtype_variant(
                NAME,
                1,
                "Them",
                &pkey
                    .raw_public_key()
                    .map_err(|_| ser_err("Couldn't convert pkey to raw public key"))?,
            ),
            Ownership::Us(pkey) => serializer.serialize_newtype_variant(
                NAME,
                1,
                "Us",
                &pkey
                    .raw_private_key()
                    .map_err(|_| ser_err("Couldn't convert pkey to raw private key"))?,
            ),
        }
    }
}
