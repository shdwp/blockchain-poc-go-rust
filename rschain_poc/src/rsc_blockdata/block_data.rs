use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

use crate::rsc_util::hash::{Hashable, Hash};

#[derive(Clone, Serialize, Deserialize)]
pub struct WalletData {
    pub pubkey: String,
}

impl Hashable for WalletData {
    fn hash(&self) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update(&self.pubkey);

        hasher.finalize().into()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TransactionData {
    pub from: String,
    pub to: String,
    pub currency: u64,
    pub amount: u64,
}

impl Hashable for TransactionData {
    fn hash(&self) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update(&self.from);
        hasher.update(&self.to);
        hasher.update(self.currency.to_ne_bytes());
        hasher.update(self.amount.to_ne_bytes());

        hasher.finalize().into()
    }
}

