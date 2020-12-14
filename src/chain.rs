//! # Chain
//!
//! This modules defines the tendermock chain. The chain is a vector of light blocks, which are
//! stripped down versions of full blown tendermint blocks.
use ibc::Height;
use std::sync::RwLock;
use tendermint::Block as TMBlock;
use tendermint_testgen::light_block::TMLightBlock;
use tendermint_testgen::{Generator, LightBlock};

pub struct Chain {
    blocks: RwLock<Blocks>,
}

struct Blocks {
    /// The chain of validated blocks.
    chain: Vec<LightBlock>,
    /// The next block candidate, it will be considered valid once another block is added.
    pending_block: Option<LightBlock>,
}

impl Chain {
    pub fn new() -> Self {
        Chain {
            blocks: RwLock::new(Blocks {
                chain: vec![LightBlock::new_default(1)],
                pending_block: None,
            }),
        }
    }

    /// Returns the height of the chain.
    ///
    /// The height is defined as the height of the latest validated blocks.
    pub fn get_height(&self) -> Height {
        let blocks = &self.blocks.read().unwrap().chain;
        let height = blocks
            .last()
            .expect("[Internal] Chain should be initialized with a block.")
            .height();
        Height::new(1, height)
    }

    /// Returns a Tendermint Light Block or None if no block exist at that height.
    pub fn get_block(&self, height: u64) -> Option<TMLightBlock> {
        let blocks = &self.blocks.read().unwrap().chain;
        let block = Chain::get_block_at_height(height, &blocks)?;
        block.generate().ok()
    }

    /// Grow the chain by adding a new block.
    pub fn grow(&self) -> Option<()> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.grow_at(now)
    }

    /// Grow the chain by adding a new block, the block will appear as if it had beend added at
    /// `time`.
    ///
    /// `time` must be an Unix timestamp.
    pub fn grow_at(&self, time: u64) -> Option<()> {
        let mut blocks = self.blocks.write().unwrap();
        if let Some(block) = blocks.pending_block.take() {
            blocks.chain.push(block);
        }
        let last_block = blocks
            .chain
            .last()
            .expect("[Internal] Chain should be initialized with a block.");
        let mut next_block = last_block.next();
        let mut header_ref = next_block.header.as_mut().unwrap();
        header_ref.time = Some(time);
        blocks.pending_block = Some(next_block);
        Some(())
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

/// Build a Tendermint block from a Tendermint loght block.
pub fn to_full_block(light_block: TMLightBlock) -> TMBlock {
    let signed_header = light_block.signed_header;
    let block = tendermint::Block::new(
        signed_header.header,
        tendermint::abci::transaction::Data::default(), // TODO: should we include transaction data?
        tendermint::evidence::Data::new(vec![]),
        Some(signed_header.commit),
    )
    .unwrap();
    block
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn chain() {
        let chain = Chain::new();
        let height = chain.get_height();

        // Chain is expected to start at height 1 (same as Storage)
        assert_eq!(height.version_height, 1);
        chain.grow();
        let height = chain.get_height();
        assert_eq!(height.version_height, 1); // The now block is not yet validated
        let block = chain.get_block(2);
        assert!(block.is_none()); // Should we be able to retrieve invalid block? For now it's not possible.
        chain.grow();
        let height = chain.get_height();
        assert_eq!(height.version_height, 2); // Now the second block is valid
    }
}
