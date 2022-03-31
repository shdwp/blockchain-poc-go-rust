use std::time::Instant;

use crate::{Block, rsc_util::error::BlockchainError, rsc_core::chain::Blockchain};

pub fn mine_block(chain: &Blockchain, mut block: Block) -> Result<Block, BlockchainError> {
    let start_time = Instant::now();

    for attempt in 0.. {
        block.update_nonce(attempt);

        if chain.check_difficulty(&block) {
            return Ok(block);
        }

        if Instant::now().duration_since(start_time).as_secs() > 5 {
            return Err("max attempts exceeded".into());
        }
    }

    Err("max attempts exceeded".into())
}

