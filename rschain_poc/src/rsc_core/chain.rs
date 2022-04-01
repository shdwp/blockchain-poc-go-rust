use core::fmt;
use std::borrow::Borrow;
use std::rc::Rc;
use std::fmt::Debug;

use openssl::pkey::PKey;

use crate::rsc_blockdata::BlockData;
use crate::rsc_blockdata::block_data::WalletData;
use crate::rsc_crypto;
use crate::rsc_util::hash::{Hash, Hashable};

use super::block::Block;

#[derive(thiserror::Error, Debug)]
pub enum BlockchainError {
    #[error("no attach point")]
    NoAttachPoint,

    #[error("wallet not found")]
    WalletNotFound,

    #[error("signature")]
    SignatureInvalid,
}

#[derive(Clone)]
pub struct Blockchain {
    blocks: Vec<Rc<Block>>,
    difficulty: usize,
}

pub struct BlockchainIterator<'a> {
    idx: usize,
    first_call: bool,
    chain: &'a Blockchain,
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

impl Blockchain {
    pub fn append(&mut self, block: &Block) -> anyhow::Result<()> {
        match &block.data {
            BlockData::Transaction(data) => {
                let wallet_data = self.find_wallet_str(&data.from)?;
                let public_key = PKey::public_key_from_pem(wallet_data.pubkey.as_bytes()).unwrap();
                let data_vec: Vec<u8> = (&block.data).into();
                let signature_bytes = hex::decode(&block.signature).unwrap();

                if !rsc_crypto::signature::check(&public_key, data_vec.as_slice(), &signature_bytes)? {
                    Err(BlockchainError::SignatureInvalid)?;
                }
            },

            _ => {}
        };

        self.blocks.push(block.clone().into());
        Ok(())
    }

    pub fn find_wallet(&self, data_hash: Hash) -> Option<&WalletData> {
        self.into_iter().find_map(|b| match &b.data {
             BlockData::Wallet(data) => if data.hash() == data_hash { Some(data) } else { None },
            _ => None,
        })
    }

    pub fn find_wallet_str(&self, string: &String) -> anyhow::Result<&WalletData> {
        self.find_wallet(hex::decode(string)?.try_into()?).ok_or(BlockchainError::WalletNotFound.into())
    }

    pub fn fork(&self, last_hash: Hash) -> anyhow::Result<Blockchain> {
        let position = 1 + (&self.blocks)
            .into_iter()
            .position(|b| b.hash == last_hash)
            .ok_or(BlockchainError::NoAttachPoint)?;

        Ok(Blockchain {
            blocks: (&self.blocks).into_iter().take(position).map(|rc| rc.clone()).collect(),
            difficulty: self.difficulty,
        })
    }

    pub fn fork_if_needed(&self, block: &Block) -> anyhow::Result<Option<Blockchain>> {
        if self.into_iter().count() == 0 {
            return Ok(None);
        }

        let last_block = self.into_iter().last().expect("check was there");
        let fork_point = self.into_iter().rev().find(|b| b.hash == block.previous_hash);

        match fork_point {
            None => return Err(BlockchainError::NoAttachPoint.into()),
            Some(b) => {
                if b.hash == last_block.hash {
                    Ok(None)
                } else {
                    Ok(Some(self.fork(b.hash)?))
                }
            },
        }
    }

    pub fn new() -> Blockchain {
        Blockchain{
            blocks: vec![],
            difficulty: 1,
        }
    }
}

impl<'a> IntoIterator for &'a Blockchain {
    type Item = &'a Block;
    type IntoIter = BlockchainIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BlockchainIterator {
            chain: self,
            idx: 0,
            first_call: true,
        }
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

        Ok(())
    }
}

impl Debug for Blockchain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Blockchain").field("blocks", &self.blocks.len()).field("difficulty", &self.difficulty).finish()
    }
}
