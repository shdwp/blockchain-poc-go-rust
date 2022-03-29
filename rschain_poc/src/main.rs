mod rsccore;

use openssl::{rsa::Rsa, pkey::{PKey, Private, Public}, sign::Signer, hash::MessageDigest, x509::X509};

use crate::rsccore::{block::Block, hash::Hash, block_data::{BlockData, WalletData}, chain::Blockchain, signature};


fn main() {
    let keypair = Rsa::generate(1024).unwrap();
    let private_pem = keypair.private_key_to_pem().unwrap();
    let public_pem = keypair.public_key_to_pem().unwrap();

    let private_key = PKey::private_key_from_pem(&private_pem).unwrap();
    let public_key = PKey::public_key_from_pem(&public_pem).unwrap();

    let block = Block::new(Hash::empty(), 0, BlockData::Empty);

    let mut chain = Blockchain::new();
    chain.append(&block).unwrap();

    let block2 = Block::new(block.hash, 0, BlockData::Wallet(WalletData { pubkey: String::from_utf8(public_pem).unwrap() }));
    chain.append(&block2).unwrap();

    println!("{}", chain);
}
