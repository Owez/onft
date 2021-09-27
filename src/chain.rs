//! Contains [Chain] and implementations

use crate::{error::Result, Block};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Representation of an Onft blockchain
///
/// # Using
///
/// You can in high level terms do the following directly to a blockchain:
///
/// - Create an initial blockchain: [Chain::default]
/// - Add some data inside a new block: [Chain::push_data]
/// - Extend multiple new pieces of data inside new blocks: [Chain::extend_data]
/// - Verify entire blockchain one-by-one: [Chain::verify]
///
/// # Example
///
/// ```rust
/// use onft::Chain;
///
/// // create
/// let mut chain = Chain::default();
/// println!("Chain: {:?}", chain);
///
/// // add block
/// chain.push_data("Hello, world!").unwrap();
/// println!("Chain: {:?}", chain);
///
/// // verify
/// if let Ok(true) = chain.verify() {
///     println!("Verified")
/// } else {
///     eprintln!("Not verified")
/// }
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Chain(Vec<Block>);

impl Chain {
    /// Verifies entire chain block-by-block from the first index.
    ///
    /// # Example
    ///
    /// ```rust
    /// use onft::Chain;
    ///
    /// // create
    /// let mut chain = Chain::default();
    /// println!("Chain: {:?}", chain);
    ///
    /// // add block
    /// chain.push_data("Hello, world!").unwrap();
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
    /// use onft::Chain;
    ///
    /// let mut chain = Chain::default();
    /// chain.push_data("Hello, world!").unwrap();
    ///
    /// println!("Chain: {:?}", chain);
    /// ```
    pub fn push_data(&mut self, data: impl Into<Vec<u8>>) -> Result<&mut Self> {
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
    /// use onft::Chain;
    ///
    /// let data_vec = vec![
    ///     "Hello".as_bytes(), "world".as_bytes(),
    ///     "multiple".as_bytes(), "data".as_bytes()
    /// ];
    ///
    /// let mut chain = Chain::default();
    /// chain.extend_data(data_vec).unwrap();
    ///
    /// println!("Chain: {:?}", chain);
    /// ```
    pub fn extend_data(
        &mut self,
        data_iter: impl IntoIterator<Item = impl Into<Vec<u8>>>,
    ) -> Result<&mut Self> {
        for data in data_iter.into_iter() {
            Self::push_data(self, data)?;
        }
        Ok(self)
    }

    // TODO: more vec-like interface
}

impl Default for Chain {
    fn default() -> Self {
        Self(vec![Block::default()])
    }
}
