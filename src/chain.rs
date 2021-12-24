//! Contains [Chain] and implementations

use crate::{error::Result, Block, Hash, Ownership};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Representation of an Onft blockchain
///
/// # Using
///
/// You can, in high level terms, do the following directly to a blockchain:
///
/// - Create an initial blockchain: [Chain::default]
/// - Add some data inside a new block: [Chain::push]
/// - Extend multiple new pieces of data inside new blocks: [Chain::extend]
/// - Verify entire blockchain one-by-one: [Chain::verify]
///
/// # Example
///
/// ```rust
/// use onft::prelude::*;
///
/// // create
/// let mut chain = Chain::default();
/// println!("Chain: {:?}", chain);
///
/// // add block
/// chain.push("Hello, world!").unwrap();
/// println!("Chain: {:?}", chain);
///
/// // verify
/// if let Ok(true) = chain.verify() {
///     println!("Verified")
/// } else {
///     eprintln!("Not verified")
/// }
/// ```
///
/// # Comparison to the [Vec] interface
///
/// This structure is based loosely of of the standard library's implementation
/// of [Vec] as this blockchain relies on it underneath. The two main differences
/// between this and the vector interface are:
///
/// - More item-level documentation; everything must be comprehensive
/// - Less methods here compared to vectors; unwise idea to [Vec::truncate] a
/// blockchain
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Chain(Vec<Block>);

impl Chain {
    /// Verifies entire chain block-by-block from the first index.
    ///
    /// # Example
    ///
    /// ```rust
    /// use onft::prelude::*;
    ///
    /// // create
    /// let mut chain = Chain::default();
    /// println!("Chain: {:?}", chain);
    ///
    /// // add block
    /// chain.push("Hello, world!").unwrap();
    /// println!("Chain: {:?}", chain);
    ///
    /// // verify
    /// if let Ok(true) = chain.verify() {
    ///     println!("Verified")
    /// } else {
    ///     eprintln!("Not verified")
    /// }
    /// ```
    ///
    /// # Performance
    ///
    /// This is a computationally heavy single-threaded task and ideally should
    /// just be done when needed block-by-block by verifying a [Block] manually
    /// using the [Block::verify] method if at all possible as the method simply
    /// links to this one.
    pub fn verify(&self) -> Result<bool> {
        let mut previous_hash = &self.0[0].hash;
        for block in self.0[1..].iter() {
            if !block.verify(previous_hash)? {
                return Ok(false);
            }
            previous_hash = &block.hash
        }
        Ok(true)
    }

    /// Adds a new single block to the chain via new data; chainable method.
    ///
    /// # Example
    ///
    /// ```rust
    /// use onft::prelude::*;
    ///
    /// let mut chain = Chain::default();
    /// chain.push("Hello, world!").unwrap();
    ///
    /// println!("Chain: {:?}", chain);
    /// ```
    pub fn push(&mut self, data: impl Into<Vec<u8>>) -> Result<&mut Self> {
        let previous_block = self.0.last().unwrap();
        let new_block = Block::new(&previous_block.hash, data)?;
        self.0.push(new_block);
        Ok(self)
    }

    /// Adds multiple blocks to the chain via an iterator of all the needed
    /// data; chainable method.
    ///
    /// # Example
    ///
    /// ```rust
    /// use onft::prelude::*;
    ///
    /// let data_vec = vec![
    ///     "Hello".as_bytes(), "world".as_bytes(),
    ///     "multiple".as_bytes(), "data".as_bytes()
    /// ];
    ///
    /// let mut chain = Chain::default();
    /// chain.extend(data_vec).unwrap();
    ///
    /// println!("Chain: {:?}", chain);
    /// ```
    pub fn extend(
        &mut self,
        data_iter: impl IntoIterator<Item = impl Into<Vec<u8>>>,
    ) -> Result<&mut Self> {
        for data in data_iter.into_iter() {
            Self::push(self, data)?;
        }
        Ok(self)
    }

    /// TODO: document
    ///
    /// # Example
    ///
    /// ```none
    /// TODO: example
    /// ```
    pub fn push_ext(&mut self, block: impl Into<Block>) -> &mut Self {
        self.0.push(block.into());
        self
    }

    /// TODO: document
    ///
    /// # Example
    ///
    /// ```none
    /// TODO: example
    /// ```
    pub fn extend_ext(&mut self, blocks: impl IntoIterator<Item = impl Into<Block>>) -> &mut Self {
        self.0.extend(blocks.into_iter().map(|block| block.into()));
        self
    }

    /// TODO: document
    ///
    /// # Example
    ///
    /// ```none
    /// TODO: example
    /// ```
    pub fn find(&self, query: ChainQuery) -> Option<&Block> {
        match query {
            ChainQuery::Hash(_) => todo!("query for hash"),
            ChainQuery::Signature(_) => todo!("query for signature"),
            ChainQuery::Owner(_) => todo!("query for owner"),
        }
    }

    /// Clears the blockchain, removing all values. This method has no effect on
    /// the allocated capacity of the block storage vector contained within.
    ///
    /// # Example
    ///
    /// ```none
    /// TODO: example
    /// ```
    pub fn clear(&mut self) {
        self.0.truncate(0)
    }
}

impl Default for Chain {
    fn default() -> Self {
        Self(vec![Block::default()])
    }
}

/// TODO: document
pub enum ChainQuery {
    /// Queries for a block's hash, combining the `blockchain + data + ownership` values
    Hash(Hash),
    /// Queries for a block's signature combining the `data + ownership` values
    Signature([u8; Hash::SIG_LEN]),
    /// Queries for the ownership key, allowing for the `ownership` value
    Owner(Ownership),
}
