mod rsc_core;

mod rsc_blockdata;
mod rsc_crypto;
mod rsc_util;

mod rsc_miner;

use openssl::{rsa::Rsa, pkey::PKey};

use crate::rsc_blockdata::BlockData;
use crate::rsc_blockdata::block_data::{WalletData, TransactionData};
use crate::rsc_core::block::Block;
use crate::rsc_core::chain::Blockchain;
use crate::rsc_util::hash::{Hash, Hashable};


fn main() {
    let keypair = Rsa::generate(1024).unwrap();
    let private_pem = keypair.private_key_to_pem().unwrap();
    let public_pem = keypair.public_key_to_pem().unwrap();

    let private_key = PKey::private_key_from_pem(&private_pem).unwrap();
    let _public_key = PKey::public_key_from_pem(&public_pem).unwrap();

    let mut chain = Blockchain::new();

    let block = Block::new(Hash::empty(), BlockData::Empty);
    let block = rsc_miner::mine_block(&chain, block).unwrap();
    chain.append(&block).unwrap();

    let wallet_data = WalletData { pubkey: String::from_utf8(public_pem).unwrap() };
    let wallet_hash = wallet_data.hash();
    let block2 = Block::new(block.hash, BlockData::Wallet(wallet_data));
    let block2 = rsc_miner::mine_block(&chain, block2).unwrap();
    chain.append(&block2).unwrap();

    let block3_data = BlockData::Transaction(TransactionData {
        from: wallet_hash.into(),
        to: String::from("deadbeef"),
        currency: 1,
        amount: 1,
    });

    let data_vec: Vec<u8> = (&block3_data).into();
    let signature = rsc_crypto::signature::sign(&private_key, &data_vec).unwrap();

    let mut block3 = Block::new(block2.hash, block3_data);
    block3.signature = hex::encode(signature);
    let block3 = rsc_miner::mine_block(&chain, block3).unwrap();
    chain.append(&block3).unwrap();
    println!("{}", chain);
}
