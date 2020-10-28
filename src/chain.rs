use ibc::Height;
use std::sync::RwLock;
use tendermint_testgen::light_block::TMLightBlock;
use tendermint_testgen::{Generator, LightBlock};

pub struct Chain {
    blocks: RwLock<Vec<LightBlock>>,
}

impl Chain {
    pub fn new() -> Self {
        Chain {
            blocks: RwLock::new(vec![LightBlock::new_default(1)]),
        }
    }

    pub fn get_height(&self) -> Height {
        let blocks = self.blocks.read().unwrap();
        let height = blocks
            .last()
            .expect("[Internal] Chain should be initialized with a block.")
            .height();
        Height::new(1, height)
    }

    pub fn get_block(&self, height: u64) -> Option<TMLightBlock> {
        let blocks = self.blocks.read().unwrap();
        let block = Chain::get_block_at_height(height, &blocks)?;
        block.generate().ok()
    }

    /// Returns the store at a given height, where 0 means latest.
    fn get_block_at_height(height: u64, blocks: &Vec<LightBlock>) -> Option<&LightBlock> {
        if height == 0 {
            blocks.last()
        } else {
            blocks.get((height - 1) as usize)
        }
    }
}
