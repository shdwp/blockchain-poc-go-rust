use std::borrow::Borrow;

use super::{chain::Blockchain, block::Block};

pub struct BlockchainIterator<'a> {
    idx: usize,
    first_call: bool,
    chain: &'a Blockchain,
}

impl<'a> BlockchainIterator<'a> {
    pub fn new(chain: &'a Blockchain) -> BlockchainIterator {
        BlockchainIterator {
            chain,
            idx: 0,
            first_call: true,
        }
    }
}

impl<'a> Iterator for BlockchainIterator<'a> {
    type Item = &'a Block;

    fn next(&mut self) -> Option<Self::Item> {
        if self.chain.blocks.len() <= self.idx {
            return Option::None;
        } else {
            self.idx += 1;
            return Option::Some(&self.chain.blocks[self.idx - 1])
        }
    }

    fn count(self) -> usize
    {
        self.chain.blocks.len()
    }

    fn last(self) -> Option<Self::Item>
    {
        self.chain.blocks.last().and_then(|b| Some(b.borrow()))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.chain.blocks.get(n).and_then(|b| Some(b.borrow()))
    }
}

impl<'a> DoubleEndedIterator for BlockchainIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.first_call {
            self.idx = self.chain.blocks.len();
            self.first_call = false;
        }

        if self.idx == 0 {
            return Option::None;
        } else {
            self.idx -= 1;
            return Option::Some(&self.chain.blocks[self.idx])
        }
    }
}

