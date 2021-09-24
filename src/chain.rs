use crate::{Block, Result};

pub struct Chain(Vec<Block>);

impl Chain {
    pub fn add_block(&mut self, data: impl Into<Vec<u8>>) -> Result<&mut Self> {
        let previous_block = self.0.last().unwrap();
        let new_block = Block::new(&previous_block.hash, data)?;
        self.0.push(new_block);
        Ok(self)
    }

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
