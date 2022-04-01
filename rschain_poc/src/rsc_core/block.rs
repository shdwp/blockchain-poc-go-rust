use core::fmt;
use substring::Substring;

use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

use crate::rsc_util::hash::{Hash, Hashable};
use crate::rsc_blockdata::BlockData;

pub type Nonce = u64;

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    pub hash: Hash,
    pub previous_hash: Hash,
    pub nonce: Nonce,
    pub signature: String,

    pub data: BlockData,
}

impl Block {
    pub fn new(previous_hash: Hash, data: BlockData) -> Block {
        Block {
            hash: Hash::new(),
            nonce: 0,
            previous_hash,
            data,
            signature: String::new(),
        }
    }

    pub fn update_nonce(&mut self, nonce: Nonce) {
        self.nonce = nonce;
        self.hash = self.hash();
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hash_string = self.hash.to_string();
        let data_string = self.data.to_string();

        let _ = f.write_fmt(format_args!(
                "{} {}",
                hash_string.substring(0, 8),
                data_string.substring(0, 200)
        ));

        Result::Ok(())
    }
}

impl Hashable for Block {
    fn hash(&self) -> Hash {
        let mut hasher = Sha256::new();
        let mut data = Vec::<u8>::new();

        data.extend(self.previous_hash.to_ne_bytes().iter());
        data.extend(self.nonce.to_ne_bytes());
        data.extend::<Vec<u8>>((&self.data).into());

        hasher.update(self.previous_hash.to_ne_bytes());
        hasher.update(self.nonce.to_ne_bytes());

        let data_bytes: Vec<u8> = (&self.data).into();
        hasher.update(data_bytes);

        hasher.finalize().try_into().expect("hasher/Hash incompat")
    }
}

