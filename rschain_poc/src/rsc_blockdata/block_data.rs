use std::fmt::Display;

use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use substring::Substring;

use crate::rsc_util::hash::{Hashable, Hash};

#[derive(Clone, Serialize, Deserialize)]
pub struct WalletData {
    pub pubkey: String,
}

impl Hashable for WalletData {
    fn hash(&self) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update(&self.pubkey);

        return hasher.finalize().try_into().expect("hasher/Hash incompat");
    }
}

impl Display for WalletData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hash_string = self.hash().to_string();

        f.write_fmt(format_args!("WALLET {}", hash_string.substring(0, 8)))
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TransactionData {
    pub from: String,
    pub to: String,
    pub currency: u64,
    pub amount: f64,
}

impl Hashable for TransactionData {
    fn hash(&self) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update(&self.from);
        hasher.update(&self.to);
        hasher.update(self.currency.to_ne_bytes());
        hasher.update(self.amount.to_ne_bytes());

        return hasher.finalize().try_into().expect("hasher/Hash incompat");
    }
}

impl Display for TransactionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let from = self.from.substring(0, 8);
        let to = self.to.substring(0, 8);

        return f.write_fmt(format_args!("TRAN of {} ({}) {} => {}", self.currency, self.amount, from, to));
    }
}
