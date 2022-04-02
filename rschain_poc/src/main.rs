mod rsc_core;

mod rsc_blockdata;
mod rsc_crypto;
mod rsc_util;

mod rsc_miner;
mod rsc_bank;

use rand::{self, Rng};

use openssl::pkey::Private;
use openssl::{rsa::Rsa, pkey::PKey};
use rsc_core::shard::Shard;

use crate::rsc_blockdata::BlockData;
use crate::rsc_blockdata::block_data::{WalletData, TransactionData};
use crate::rsc_core::block::Block;
use crate::rsc_util::hash::{ByteHash, Hashable};

fn transaction_block(shard: &Shard, prev_hash: ByteHash, from: ByteHash, private_key: &PKey<Private>, to: ByteHash) -> Block {
    let data = BlockData::Transaction(TransactionData {
        from: from.into(),
        to: to.into(),
        currency: 1,
        amount: 1.0,
    });

    let data_vec: Vec<u8> = (&data).into();
    let signature = rsc_crypto::signature::sign(private_key, &data_vec).unwrap();

    let mut block3a = Block::new(prev_hash, data);
    block3a.signature = hex::encode(signature);

    rsc_miner::mine_block(&shard, block3a).unwrap()
}

fn wallet_block(shard: &Shard, prev_hash: ByteHash) -> (ByteHash, PKey<Private>, Block) {
    let keypair = Rsa::generate(1024).unwrap();
    let private_pem = keypair.private_key_to_pem().unwrap();
    let public_pem = keypair.public_key_to_pem().unwrap();

    let private_key = PKey::private_key_from_pem(&private_pem).unwrap();
    let _public_key = PKey::public_key_from_pem(&public_pem).unwrap();

    let data = WalletData { pubkey: String::from_utf8(public_pem.clone()).unwrap() };
    let wallet_hash = data.hash();
    let block = rsc_miner::mine_block(shard, Block::new(prev_hash, BlockData::Wallet(data))).unwrap();

    (wallet_hash, private_key, block)
}


fn push_block(shard: &mut Shard, block: Block) -> ByteHash {
    let block = rsc_miner::mine_block(&shard, block).unwrap();
    let block_hash = block.hash;
    println!("PUSH {:?}", shard.push(block));

    return block_hash;
}

fn main() {
    let mut shard = Shard::new();

    let bhash1 = push_block(&mut shard, Block::new(ByteHash::new(), BlockData::Empty));

    let (w1, w1pk, b2) = wallet_block(&shard, bhash1);
    let bhash2 = push_block(&mut shard, b2);

    let (w2, w2pk, b3) = wallet_block(&shard, bhash2);
    let bhash3 = push_block(&mut shard, b3);

    let block3 = transaction_block(&shard, bhash2, w1, &w1pk, w2);
    let bhash3 = push_block(&mut shard, block3);

    println!("{}", shard);
}
