pub mod block_data;

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::rsc_util::hash::{Hash, Hashable};
use crate::rsc_blockdata::block_data::TransactionData;
use crate::rsc_blockdata::block_data::WalletData;

#[derive(Clone, Serialize, Deserialize)]
pub enum BlockData {
    Empty,
    Wallet(WalletData),
    Transaction(TransactionData),
}

impl fmt::Display for BlockData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        serde_json::to_string(&self)
            .map_err(|_| fmt::Error::default())
            .and_then(|js| f.write_str(&js))
    }
}

impl From<&BlockData> for Vec<u8> {
    fn from(d: &BlockData) -> Self {
        serde_json::to_vec(d).unwrap()
    }
}

impl Hashable for BlockData {
    fn hash(&self) -> Hash {
        match self {
            BlockData::Empty => Hash::empty(),
            BlockData::Wallet(data) => data.hash(),
            BlockData::Transaction(data) => data.hash(),
        }
    }
}

