use std::{collections::HashMap, fmt::Display};

use openssl::pkey::PKey;

use crate::{rsc_util::hash::{ByteHash, Hashable}, rsc_blockdata::{block_data::{TransactionData, WalletData}, BlockData}, rsc_crypto, rsc_core::block::Block};

#[derive(thiserror::Error, Debug)]
pub enum BankError {
    #[error("WalletNotFound")]
    WalletNotFound,

    #[error("WalletDuplicate")]
    WalletDuplicate,

    #[error("WalletNotFound")]
    SignatureInvalid,

    #[error("InsufficientCurrency")]
    InsufficientCurrency,
}

#[derive(Clone)]
pub struct Wallet {
    pub hash: ByteHash,
    pub pubkey: Vec<u8>,
    pub accounts: HashMap<u64, f64>
}

impl Wallet {
    pub fn new(hash: ByteHash, pubkey: Vec<u8>) -> Wallet {
        Wallet {
            hash,
            pubkey,
            accounts: HashMap::new(),
        }
    }

    pub fn deduct(&mut self, currency: u64, amount: f64) -> anyhow::Result<()> {
        let account_amount = self.accounts.get_mut(&currency).ok_or(BankError::InsufficientCurrency)?;
        if *account_amount < amount {
            return Err(BankError::InsufficientCurrency.into());
        }

        *account_amount -= amount;
        Ok(())
    }

    pub fn add(&mut self, currency: u64, amount: f64) {
        self.accounts.entry(currency).and_modify(|a| *a += amount ).or_insert(amount);
    }
}

#[derive(Clone)]
pub struct Bank {
    pub wallets: HashMap<ByteHash, Wallet>
}

impl Bank {
    pub fn new() -> Bank {
        Bank { wallets: HashMap::new() }
    }

    pub fn get_wallet(&mut self, hash: ByteHash) -> anyhow::Result<&mut Wallet> {
        self.wallets.get_mut(&hash).ok_or(BankError::WalletNotFound.into())
    }

    pub fn do_block(&mut self, block: &Block) -> anyhow::Result<()> {
        self.process_block(block, false)
    }

    pub fn undo_block(&mut self, block: &Block) -> anyhow::Result<()> {
        self.process_block(block, true)
    }

    fn process_block(&mut self, block: &Block, invert: bool) -> anyhow::Result<()> {
        match &block.data {
            BlockData::Transaction(data) => self.process_transaction_block(block, data, invert),
            BlockData::Wallet(data) => self.process_wallet_block(data, invert),

            _ => Ok(()),
        }
    }

    fn process_wallet_block(&mut self, data: &WalletData, invert: bool) -> anyhow::Result<()> {
        let hash = data.hash();

        if !invert {
            if self.wallets.contains_key(&hash) {
                Err(BankError::WalletDuplicate)?;
            }

            let pubkey: Vec<u8> = data.pubkey.clone().try_into()?;
            let mut wallet = Wallet::new(hash, pubkey);
            wallet.add(1, 100.0);
            self.wallets.insert(hash, wallet);
            Ok(())
        } else {
            if !self.wallets.contains_key(&hash) {
                Err(BankError::WalletNotFound)?;
            }

            self.wallets.remove_entry(&hash).map_or(Err(BankError::WalletNotFound.into()), |_| Ok(()))
        }
    }

    fn process_transaction_block(&mut self, block: &Block, data: &TransactionData, invert: bool) -> anyhow::Result<()>{
        let from_hash: ByteHash = (&data.from).try_into()?;
        let to_hash: ByteHash = (&data.to).try_into()?;

        if !self.wallets.contains_key(&to_hash) {
            return Err(BankError::WalletNotFound.into());
        }

        let from = self.wallets.get_mut(&from_hash).ok_or(BankError::WalletNotFound)?;

        let data_vec: Vec<u8> = (&block.data).into();
        let signature_bytes = hex::decode(&block.signature).unwrap();
        let public_key = PKey::public_key_from_pem(&from.pubkey).unwrap();

        if !rsc_crypto::signature::check(&public_key, data_vec.as_slice(), &signature_bytes)? {
            Err(BankError::SignatureInvalid)?;
        }

        if !invert {
            {
                let from_mut = self.wallets.get_mut(&from_hash).ok_or(BankError::WalletNotFound)?;
                from_mut.deduct(data.currency, data.amount)?;
            }

            {
                let to_mut = self.wallets.get_mut(&to_hash).expect("precheck");
                to_mut.add(data.currency, data.amount);
            }
        } else {
            {
                let to_mut = self.wallets.get_mut(&to_hash).expect("precheck");
                to_mut.deduct(data.currency, data.amount)?;
            }

            {
                let from_mut = self.wallets.get_mut(&from_hash).ok_or(BankError::WalletNotFound)?;
                from_mut.add(data.currency, data.amount);
            }
        }

        Ok(())
    }
}

impl Display for Wallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (c, a) in &self.accounts {
            f.write_fmt(format_args!("{}={},", c, a))?;
        }

        Ok(())
    }
}

impl Display for Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (hash, wallet) in &self.wallets {
            f.write_fmt(format_args!("{} ({})", hash, wallet))?;
        }

        Ok(())
    }
}
