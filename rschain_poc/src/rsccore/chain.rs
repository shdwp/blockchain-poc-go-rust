use core::fmt;
use std::{error::Error};

use openssl::pkey::PKey;

use super::{block::Block, signature, block_data::{WalletData, BlockData}, hash::{Hashable, Hash}};

pub struct Blockchain {
    blocks: Vec<Block>,
    difficulty: usize,
}

#[derive(Debug)]
pub struct BlockchainError {
    description: String
}

impl fmt::Display for BlockchainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.description);
        Result::Ok(())
    }
}

impl From<&str> for BlockchainError {
    fn from(s: &str) -> Self {
        BlockchainError { description: s.into() }
    }
}

impl From<String> for BlockchainError {
    fn from(s: String) -> Self {
        BlockchainError { description: s }
    }
}

impl Error for BlockchainError {
}

pub struct BlockchainIterator<'a> {
    idx: usize,
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
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain{
            blocks: vec![],
            difficulty: 0,
        }
    }

    pub fn last_block(&self) -> &Block {
        self.blocks.last().unwrap()
    }

    pub fn find<'a, R, T: Fn(&Block) -> Option<&'a R>>(&'a self, pred: T) -> Option<&'a R> {
        for x in self.into_iter() {
            match pred(x) {
                Some(data) => return Some(data),
                _ => {},
            };
        }

        None
    }

    pub fn find_wallet<'a>(&'a self, data_hash: Hash) -> Option<&'a WalletData> {
        let x = self.find(|block| match &block.data {
             BlockData::Wallet(data) => if data.hash() == data_hash { Some(&data) } else { None },
            _ => None,
        });

        None
    }

    pub fn append(&mut self, block: &Block) -> Result<(), BlockchainError> {
        if self.blocks.len() > 0 {
            let last_block = self.last_block();
            if last_block.hash != block.previous_hash {
                return Result::Err("previous hash mismatch".into())
            }
        }

        if !block.hash.check_difficulty(self.difficulty) {
            return Result::Err("difficulty mismatch".into());
        }

        match &block.data {
            BlockData::Transaction(data) => {
                let wallet = self.find_wallet((&data.from).into());
                match wallet {
                    None => return Result::Err("wallet found".into()),
                    Some(wallet_data) => {
                        let public_key = PKey::public_key_from_pem(wallet_data.pubkey.as_bytes()).unwrap();
                        let data_vec: Vec<u8> = (&block.data).into();
                        let check_result = signature::check(&public_key, data_vec.as_slice(), data.signature.as_bytes());

                        match check_result {
                            Err(err) => return Result::Err(format!("signature check failed - {}", err).into()),
                            Ok(false) => return Result::Err("signature check failed".into()),
                            Ok(true) => {},
                        };
                    }
                }
            },

            _ => {}
        };

        self.blocks.push(block.clone());
        Result::Ok(())
    }
}

impl<'a> IntoIterator for &'a Blockchain {
    type Item = &'a Block;
    type IntoIter = BlockchainIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BlockchainIterator {
            chain: self,
            idx: 0,
        }
    }
}

impl fmt::Display for Blockchain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, block) in self.blocks.iter().enumerate() {
            let _ = f.write_fmt(format_args!("{} {}\n", i, block));
        }

        Result::Ok(())
    }
}
