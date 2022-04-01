use std::{fmt::{Debug, Display}, ptr};

use super::{chain::Blockchain, block::Block};
use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum ShardError { 
    #[error("duplicate")]
    Duplicate,

    #[error("rejected")]
    Rejected,

    #[error("difficulty")]
    Difficulty,
}

pub struct Shard {
    lead_idx: Option<usize>,
    chains: Vec<Blockchain>,
    difficulty: usize,
    cleanup_threshold: usize,
}

impl Shard {
    pub fn new() -> Self {
        let chain = Blockchain::new();

        Self {
            chains: vec!(chain),
            lead_idx: None,
            difficulty: 0,
            cleanup_threshold: 3,
        }
    }

    pub fn check_difficulty(&self, block: &Block) -> bool {
        for i in 0..self.difficulty {
            if block.hash[i] != 0u8 {
                return false;
            }
        }

        true
    }

    pub fn push(&mut self, block: Block) -> anyhow::Result<()> {
        let is_duplicate = (&self.chains)
            .into_iter()
            .any(|chain| chain.into_iter()
                .any(|b| b.hash == (&block).hash));

        if is_duplicate {
            return Err(ShardError::Duplicate.into());
        }

        if !self.check_difficulty(&block) {
            return Err(ShardError::Difficulty.into());
        }

        self.push_impl(block)?;
        self.update_longest_chain_idx();
        self.cleanup();

        return Ok(());
    }

    fn push_impl(&mut self, block: Block) -> anyhow::Result<()> {
        let mut new_chains = Vec::<Blockchain>::new();

        let mut added = false;
        for chain in &mut self.chains {
            if !chain.into_iter().any(|b| b.hash == (&block).hash) {

                match chain.fork_if_needed(&block) {
                    Err(_) => {},

                    Ok(None) => {
                        if chain.append(&block).is_ok() {
                            added = true;
                        }
                    },

                    Ok(Some(mut chain)) => {
                        if chain.append(&block).is_ok() {
                            added = true;
                        }

                        new_chains.push(chain);
                    }
                };

            }
        }

        self.chains.append(&mut new_chains);
        return match added {
            true => Ok(()),
            false => Err(ShardError::Rejected.into()),
        };
    }

    fn update_longest_chain_idx(&mut self) {
        let chain = (&self.chains).into_iter().max_by_key(|c| c.into_iter().count());
        self.lead_idx = chain.and_then(|c| (&self.chains).into_iter().position(|a| ptr::eq(a, c)));
    }

    fn cleanup(&mut self) {
        let leader = self.lead_idx.map(|i| &self.chains[i]);
        if leader.is_none() {
            return;
        }

        let leader = leader.expect("precheck");

        let mut removed_keys = Vec::<usize>::new();
        for (i, chain) in (&self.chains).into_iter().enumerate() {
            let diff = leader.into_iter().count().checked_sub(chain.into_iter().count()).unwrap_or(0);
            if diff > self.cleanup_threshold {
                removed_keys.push(i);
            }
        }

        for key in removed_keys.into_iter().rev() {
            self.chains.remove(key);
        }
    }
}

impl Debug for Shard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Shard").field("longest_chain", &self.lead_idx).field("chains", &self.chains.len()).finish()
    }
}

impl Display for Shard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Longest: {:?} ", self.lead_idx))?;
        for (i, chain) in (&self.chains).into_iter().enumerate() {
            f.write_fmt(format_args!("chain {}:\n{}", i, chain))?;
        }

        Ok(())
    }
}
