//! Contains [Block], [Ownership] and implementations

use crate::{error::Error, Hash, Result, DEFAULT_GENESIS};
use openssl::pkey::{Id, PKey, Private, Public};
use openssl::sha::Sha256;
#[cfg(feature = "serde")]
use serde::{ser::SerializeStruct, Serialize};
use serde::{Deserialize, Deserializer}; // TODO: merge with `#[cfg(feature = "serde")]` item

/// Single block within a larger blockchain, providing access to a block of data
///
/// # Using
///
/// You can, in high level terms, do the following directly to a block:
///
/// - Create a genesis block: [Block::default]
/// - Create a block containing data: [Block::new]
/// - Verify a block: [Block::verify]
///
/// # Example
///
/// ```rust
/// use onft::prelude::*;
///
/// fn main() -> onft::Result<()> {
///     let genesis_block = Block::default();
///
///     let data = "Hello, world!";
///     let new_block = Block::new(&genesis_block, data)?;
///     let verified = new_block.verify(&genesis_block)?;
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
    /// Underlying data contained for this block.
    pub data: BlockData,
}

impl<'a> Block {
    /// Creates a new block from the previous block in a chain alongside the data
    /// contained within this block.
    ///
    /// # Example
    ///
    /// ```rust
    /// use onft::prelude::*;
    ///
    /// fn main() -> onft::Result<()> {
    ///     let genesis_block = Block::default();
    ///
    ///     let data = "Hello, world!";
    ///     let block = Block::new(&genesis_block, data)?;
    ///
    ///     println!("Block:\n{:?}", block);
    ///     Ok(())
    /// }
    /// ```
    pub fn new(previous_hash: impl Into<&'a Hash>, data: impl Into<Vec<u8>>) -> Result<Self> {
        let data = BlockData::new(data.into())?;
        let (hash, signature, pkey) = Hash::new(previous_hash, data.hash)?;
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
    /// use onft::prelude::*;
    ///
    /// fn main() -> onft::Result<()> {
    ///     let genesis_block = Block::default();
    ///
    ///     let data = "Hello, world!";
    ///     let new_block = Block::new(&genesis_block, data)?;
    ///     let verified = new_block.verify(&genesis_block)?;
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
        let data_hash = self.data.hash;

        match &self.ownership {
            Ownership::Them(pkey) => {
                self.hash
                    .verify(previous_hash, self.signature, data_hash, pkey)
            }
            Ownership::Us(pkey) => self
                .hash
                .verify(previous_hash, self.signature, data_hash, pkey),
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
            data: BlockData::default(),
        }
    }
}

#[cfg(feature = "serde")]
impl Serialize for Block {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Block", 4 + 1)?;
        state.serialize_field("pver", &PROTO_VERSION)?; // custom protocol version
        state.serialize_field("hash", &self.hash)?;
        state.serialize_field("ownership", &self.ownership)?;
        state.serialize_field("data", &self.data.inner)?;
        state.serialize_field("data_hash", &self.data.hash)?;
        state.end()
    }
}

// TODO: deserialize

/// Data contained within a block along with it's hash to be used downstream
///
/// # Example
///
/// TODO: example
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockData {
    /// Actual data in a byte vector.
    pub inner: Vec<u8>,
    /// Computed hash of data to use for constructing/verifying block hashes.
    pub hash: [u8; 32],
}

impl BlockData {
    /// Creates new instance from data, hashing automatically.
    pub fn new(data: impl Into<Vec<u8>>) -> Result<Self> {
        let mut hasher = Sha256::new();
        let data = data.into();
        hasher.update(data.as_slice());
        Ok(Self {
            inner: data,
            hash: hasher.finish(),
        })
    }
}

impl Default for BlockData {
    fn default() -> Self {
        Self {
            inner: vec![],
            hash: DEFAULT_GENESIS,
        }
    }
}

impl From<BlockData> for Vec<u8> {
    fn from(block_data: BlockData) -> Self {
        block_data.inner
    }
}

impl<'a> From<&'a BlockData> for &'a [u8] {
    fn from(block_data: &'a BlockData) -> Self {
        &block_data.inner[..]
    }
}

impl From<BlockData> for [u8; 32] {
    fn from(block_data: BlockData) -> Self {
        block_data.hash
    }
}

impl From<&BlockData> for [u8; 32] {
    fn from(block_data: &BlockData) -> Self {
        block_data.hash
    }
}

// TODO: try_into

/// Contains ownership keys and information for a given block
#[derive(Deserialize, Debug, Clone)]
pub enum Ownership {
    /// Special genesis ownership type as the genesis block is owned by nobody.
    Genesis,
    /// Owned by an external source as we have a general public key.
    // todo: #[cfg_attr(feature = "serde", serde(deserialize_with = "Ownership::from_public_raw"))]
    #[serde(deserialize_with = "de_pkey_pub")]
    Them(PKey<Public>),
    /// Owned by us as we have a private key.
    // todo: #[cfg_attr(feature = "serde", serde(skip_deserializing))]
    #[serde(skip_deserializing)]
    Us(PKey<Private>),
}

// TODO: finish
/// Produces serde-orientated data into a new pkey instance
// #[cfg(feature = "serde")]
fn de_pkey_pub<'de, D>(_data: D) -> std::result::Result<PKey<Public>, D::Error>
where
    D: Deserializer<'de>,
{
    // let pkey = PKey::public_key_from_raw_bytes(bytes.as_ref(), Id::ED25519)
    //     .map_err(Error::KeyPublic)?;
    todo!()
}

impl Ownership {
    /// Converts ownership to a public key, used primarily for serialization if enabled.
    pub fn to_raw_public(&self) -> Result<Vec<u8>> {
        match self {
            Self::Genesis => Err(Error::GenesisIsNotKey),
            Self::Them(pkey) => pkey.raw_public_key().map_err(Error::KeyPublic),
            Self::Us(pkey) => pkey.raw_public_key().map_err(Error::KeyPublic),
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

#[cfg(feature = "serde")]
impl Serialize for Ownership {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        const NAME: &str = "Ownership";
        match self {
            Ownership::Genesis => serializer.serialize_unit_variant(NAME, 0, "Genesis"),
            _ => serializer.serialize_newtype_variant(
                NAME,
                1,
                "Them",
                &self
                    .to_raw_public()
                    .map_err(|err| serde::ser::Error::custom(&format!("{}", err)))?[..],
            ),
        }
    }
}
