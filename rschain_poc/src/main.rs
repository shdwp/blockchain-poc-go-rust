mod rsccore;

use openssl::{rsa::Rsa, pkey::{PKey, Private, Public}, sign::Signer, hash::MessageDigest, x509::X509};
use rsccore::block_data::TransactionData;

use crate::rsccore::{block::Block, hash::{Hash, Hashable}, block_data::{BlockData, WalletData}, chain::Blockchain, signature};


fn main() {
    let keypair = Rsa::generate(1024).unwrap();
    let private_pem = keypair.private_key_to_pem().unwrap();
    let public_pem = keypair.public_key_to_pem().unwrap();

    let private_key = PKey::private_key_from_pem(&private_pem).unwrap();
    let public_key = PKey::public_key_from_pem(&public_pem).unwrap();

    let block = Block::new(Hash::empty(), 0, BlockData::Empty);

    let mut chain = Blockchain::new();
    chain.append(&block).unwrap();

    let wallet_data = WalletData { pubkey: String::from_utf8(public_pem).unwrap() };
    let wallet_hash = wallet_data.hash();
    let block2 = Block::new(block.hash, 0, BlockData::Wallet(wallet_data));
    chain.append(&block2).unwrap();

    let block3_data = BlockData::Transaction(TransactionData {
        from: wallet_hash.into(),
        to: String::from("deadbeef"),
        currency: 1,
        amount: 1,
    });

    let data_vec: Vec<u8> = (&block3_data).into();
    let signature = signature::sign(&private_key, &data_vec).unwrap();

    let mut block3 = Block::new(block2.hash, 0, block3_data);
    block3.signature = hex::encode(signature);
    chain.append(&block3).unwrap();
    println!("{}", chain);

}
