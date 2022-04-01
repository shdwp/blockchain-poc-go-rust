use std::time::Instant;

use crate::{Block, rsc_core::shard::Shard};

#[derive(thiserror::Error, Debug)]
pub enum MiningError {
    #[error("max time exceeded")]
    MaxTimeExceeded,
}

pub fn mine_block(shard: &Shard, mut block: Block) -> anyhow::Result<Block> {
    let start_time = Instant::now();

    for attempt in 0.. {
        block.update_nonce(attempt);

        if shard.check_difficulty(&block) {
            return Ok(block);
        }

        if Instant::now().duration_since(start_time).as_secs() > 5 {
            return Err(MiningError::MaxTimeExceeded.into());
        }
    }

    panic!("infinit. loop exit")
}

