use core::fmt;

use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

use super::{hash::{Hash, Hashable}, block_data::BlockData};

pub type Nonce = u64;

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    pub hash: Hash,
    pub previous_hash: Hash,
    pub nonce: Nonce,

    pub data: BlockData,
}

impl Block {
    pub fn new(previous_hash: Hash, nonce: Nonce, data: BlockData) -> Block {
        let mut block = Block {
            hash: Hash::empty(),
            nonce: 0,
            previous_hash,
            data,
        };
        block.hash = block.hash();

        block
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f.write_fmt(format_args!("{} {}", self.hash, self.data));

        Result::Ok(())
    }
}

impl Hashable for Block {
    fn hash(&self) -> Hash {
        let mut hasher = Sha256::new();
        let mut data = Vec::<u8>::new();

        data.extend(self.previous_hash.to_ne_bytes().iter());
        data.extend(self.nonce.to_ne_bytes());

        let dataJson = serde_json::to_vec(&self.data);
        data.extend(dataJson.unwrap());

        hasher.update(data.as_slice());
        Hash::from_array(hasher.finalize())
    }
}

