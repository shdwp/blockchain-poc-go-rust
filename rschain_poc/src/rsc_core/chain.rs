use core::fmt;
use std::rc::Rc;
use std::fmt::Debug;

use crate::rsc_bank::Bank;
use crate::rsc_blockdata::BlockData;
use crate::rsc_util::hash::{ByteHash, Hashable};

use super::block::Block;
use super::chain_iter::BlockchainIterator;

#[derive(thiserror::Error, Debug)]
pub enum BlockchainError {
    #[error("no attach point")]
    NoAttachPoint,
}

#[derive(Clone)]
pub struct Blockchain {
    pub blocks: Vec<Rc<Block>>,
    bank: Bank,
    difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain{
            blocks: vec![],
            bank: Bank::new(),
            difficulty: 1,
        }
    }

    pub fn append(&mut self, block: &Block) -> anyhow::Result<()> {
        self.bank.do_block(block)?;
        self.blocks.push(block.clone().into());

        Ok(())
    }

    pub fn fork(&self, last_hash: ByteHash) -> anyhow::Result<Blockchain> {
        let position = 1 + (&self.blocks)
            .into_iter()
            .position(|b| b.hash == last_hash)
            .ok_or(BlockchainError::NoAttachPoint)?;

        let mut bank = self.bank.clone();
        let mut blocks = self.blocks.clone();

        for undo_block_idx in self.blocks.len()..position {
            let undo_block = blocks.swap_remove(undo_block_idx);
            bank.undo_block(&undo_block)?;
        }

        Ok(Blockchain {
            blocks,
            bank,
            difficulty: self.difficulty,
        })
    }

    pub fn fork_if_needed(&self, block: &Block) -> anyhow::Result<Option<Blockchain>> {
        if self.into_iter().count() == 0 {
            return Ok(None);
        }

        let last_block = self.into_iter().last().expect("check was there");
        let fork_point = self.into_iter().rev().find(|b| b.hash == block.previous_hash);

        fork_point
            .ok_or(BlockchainError::NoAttachPoint.into())
            .and_then(|b| if b.hash == last_block.hash { Ok(None) } else { Ok(Some(self.fork(b.hash)?)) })
    }

}

impl<'a> IntoIterator for &'a Blockchain {
    type Item = &'a Block;
    type IntoIter = BlockchainIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BlockchainIterator::new(&self)
    }
}

impl fmt::Display for Blockchain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, block) in self.blocks.iter().enumerate() {
            let result = f.write_fmt(format_args!("{} {}\n", i, block));
            if result.is_err() {
                return result;
            }
        }

        f.write_fmt(format_args!("\n{}", self.bank))
    }
}

impl Debug for Blockchain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Blockchain").field("blocks", &self.blocks.len()).field("difficulty", &self.difficulty).finish()
    }
}
