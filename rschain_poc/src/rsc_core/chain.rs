use core::fmt;
use std::iter::repeat;

use openssl::pkey::PKey;

use crate::rsc_blockdata::BlockData;
use crate::rsc_blockdata::block_data::WalletData;
use crate::rsc_crypto;
use crate::rsc_util::error::BlockchainError;
use crate::rsc_util::hash::{Hash, Hashable};

use super::block::Block;

pub struct Blockchain {
    blocks: Vec<Block>,
    difficulty: usize,
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
            difficulty: 1,
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

    pub fn find_wallet(&self, data_hash: Hash) -> Option<&WalletData> {
        self.into_iter().find_map(|b| match &b.data {
             BlockData::Wallet(data) => if data.hash() == data_hash { Some(data) } else { None },
            _ => None,
        })
    }

    pub fn append(&mut self, block: &Block) -> Result<(), BlockchainError> {
        if !self.into_iter().last().map_or(true, |b| b.hash == block.previous_hash) {
            return Result::Err("previous hash mismatch".into())
        }

        if !self.check_difficulty(block) {
            return Result::Err("difficulty mismatch".into());
        }

        match &block.data {
            BlockData::Transaction(data) => {
                let wallet_hash: Hash = hex::decode(&data.from).unwrap().try_into().unwrap();
                let wallet = self.find_wallet(wallet_hash);

                match wallet {
                    None => return Result::Err("wallet not found".into()),
                    Some(wallet_data) => {
                        let public_key = PKey::public_key_from_pem(wallet_data.pubkey.as_bytes()).unwrap();
                        let data_vec: Vec<u8> = (&block.data).into();

                        let signature_bytes = hex::decode(&block.signature).unwrap();
                        let check_result = rsc_crypto::signature::check(&public_key, data_vec.as_slice(), &signature_bytes);

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
            let result = f.write_fmt(format_args!("{} {}\n", i, block));
            if result.is_err() {
                return result;
            }
        }

        Ok(())
    }
}
