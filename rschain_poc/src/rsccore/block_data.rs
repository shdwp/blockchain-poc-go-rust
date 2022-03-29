use core::fmt;

use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

use super::hash::{Hashable, Hash};

#[derive(Clone, Serialize, Deserialize)]
pub struct WalletData {
    pub pubkey: String,
}

impl Hashable for WalletData {
    fn hash(&self) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update(&self.pubkey);

        Hash::from_array(hasher.finalize())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TransactionData {
    pub from: String,
    pub to: String,
    pub currency: u64,
    pub amount: u64,
    pub signature: String,
}

impl Hashable for TransactionData {
    fn hash(&self) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update(&self.from);
        hasher.update(&self.to);
        hasher.update(self.currency.to_ne_bytes());
        hasher.update(self.amount.to_ne_bytes());
        hasher.update(&self.signature);

        Hash::from_array(hasher.finalize())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum BlockData {
    Empty,
    Wallet(WalletData),
    Transaction(TransactionData),
}

impl fmt::Display for BlockData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string(&self);
        let _ = f.write_str(&json.unwrap());

        Result::Ok(())
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
