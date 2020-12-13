use ibc::Height;
use std::sync::RwLock;
use tendermint::Block as TMBlock;
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

    /// Returns the height of the chain.
    ///
    /// The height is defined as the height of the latest validated blocks, that is not the last
    /// block but the one just before.
    pub fn get_height(&self) -> Height {
        let blocks = self.blocks.read().unwrap();
        let height = blocks
            .last()
            .expect("[Internal] Chain should be initialized with a block.")
            .height();
        Height::new(1, std::cmp::max(height -1, 1))
    }

    /// Returns a Tendermint Light Block or None if no block exist at that height.
    pub fn get_block(&self, height: u64) -> Option<TMLightBlock> {
        let blocks = self.blocks.read().unwrap();
        let block = Chain::get_block_at_height(height, &blocks)?;
        block.generate().ok()
    }

    /// Grow the chain by adding a new block.
    pub fn grow(&self) -> Option<()> {
        let mut blocks = self.blocks.write().unwrap();
        let last_block = blocks
            .last()
            .expect("[Internal] Chain should be initialized with a block.");
        let mut next_block = last_block.next();
        let mut header_ref = next_block.header.as_mut().unwrap();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        header_ref.time = Some(now);
        blocks.push(next_block);
        Some(())
    }

    /// Grow the chain by adding a new block, the block will appear as if it had beend added at
    /// `time`.
    ///
    /// `time` must be an Unix timestamp.
    pub fn grow_at(&self, time: u64) -> Option<()> {
        let mut blocks = self.blocks.write().unwrap();
        let last_block = blocks
            .last()
            .expect("[Internal] Chain should be initialized with a block.");
        let mut next_block = last_block.next();
        let mut header_ref = next_block.header.as_mut().unwrap();
        header_ref.time = Some(time);
        blocks.push(next_block);
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
        assert!(block.is_some()); // But still, it should exist.
        chain.grow();
        let height = chain.get_height();
        assert_eq!(height.version_height, 2); // Now the second block is valid
    }
}
