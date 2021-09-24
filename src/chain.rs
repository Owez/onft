use crate::{Block, Result};

pub struct Chain(Vec<Block>);

impl Chain {
    /// Verifies entire chain block-by-block from the first index.
    ///
    /// # Example
    ///
    /// TODO: example
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
    /// TODO: example
    pub fn add_block(&mut self, data: impl Into<Vec<u8>>) -> Result<&mut Self> {
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
    /// TODO: example
    pub fn add_blocks(
        &mut self,
        data_iter: impl IntoIterator<Item = Vec<u8>>,
    ) -> Result<&mut Self> {
        for data in data_iter.into_iter() {
            Self::add_block(self, data)?;
        }
        Ok(self)
    }
}

impl Default for Chain {
    fn default() -> Self {
        Self(vec![Block::default()])
    }
}
