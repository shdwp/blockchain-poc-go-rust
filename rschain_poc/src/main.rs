mod rsc_core;

mod rsc_blockdata;
mod rsc_crypto;
mod rsc_util;

mod rsc_miner;

use rand::{self, Rng};

use openssl::pkey::Private;
use openssl::{rsa::Rsa, pkey::PKey};
use rsc_core::shard::Shard;

use crate::rsc_blockdata::BlockData;
use crate::rsc_blockdata::block_data::{WalletData, TransactionData};
use crate::rsc_core::block::Block;
use crate::rsc_util::hash::{Hash, Hashable};

fn transaction_block(shard: &Shard, private_key: &PKey<Private>, prev_hash: Hash, from: Hash) -> Block {
    let receiver = rand::thread_rng().gen::<[u8; 32]>();

    let data = BlockData::Transaction(TransactionData {
        from: from.into(),
        to: hex::encode(receiver),
        currency: 1,
        amount: 1.0,
    });

    let data_vec: Vec<u8> = (&data).into();
    let signature = rsc_crypto::signature::sign(private_key, &data_vec).unwrap();

    let mut block3a = Block::new(prev_hash, data);
    block3a.signature = hex::encode(signature);

    rsc_miner::mine_block(&shard, block3a).unwrap()
}

/*
fn shardless_test(private_key: &PKey<Private>, public_pem: &Vec<u8>) {
    let mut chaina = Blockchain::new();

    let block = Block::new(Hash::empty(), BlockData::Empty);
    let block = rsc_miner::mine_block(&chaina, block).unwrap();
    chaina.append(&block).unwrap();

    let wallet_data = WalletData { pubkey: String::from_utf8(public_pem.clone()).unwrap() };
    let wallet_hash = wallet_data.hash();
    let block2 = Block::new(block.hash, BlockData::Wallet(wallet_data));
    let block2 = rsc_miner::mine_block(&chaina, block2).unwrap();
    chaina.append(&block2).unwrap();

    let mut chainb = chaina.clone();

    let block3a = transaction_block(&chaina, &private_key, &block2, wallet_hash);
    chaina.append(&block3a).unwrap();

    let block3b = transaction_block(&chainb, &private_key, &block2, wallet_hash);
    chainb.append(&block3b).unwrap();

    match chaina.fork_if_needed(&block3b) {
        Err(err) => {println!("Error: {}", err)},

        Ok(Some(mut chain)) => { 
            println!("forked:\n{}", chain);
            println!("appending: {:?}", chain.append(&block3b));
            println!("{}", chain);
        },

        Ok(None) => { println!("not forked, appending: {:?}", chaina.append(&block3b)); },
    };

    println!("chain a {}\n{}", chaina.into_iter().count(), chaina);
    println!("chain b {}\n{}", chainb.into_iter().count(), chainb);
}
*/

fn main() {
    let keypair = Rsa::generate(1024).unwrap();
    let private_pem = keypair.private_key_to_pem().unwrap();
    let public_pem = keypair.public_key_to_pem().unwrap();

    let private_key = PKey::private_key_from_pem(&private_pem).unwrap();
    let _public_key = PKey::public_key_from_pem(&public_pem).unwrap();

    let mut shard = Shard::new();

    let block = Block::new(Hash::new(), BlockData::Empty);
    let block1 = rsc_miner::mine_block(&shard, block).unwrap();
    let block1_hash = block1.hash;
    println!("PUSH {:?}", shard.push(block1));

    let wallet_data = WalletData { pubkey: String::from_utf8(public_pem.clone()).unwrap() };
    let wallet_hash = wallet_data.hash();
    let block2 = Block::new(block1_hash, BlockData::Wallet(wallet_data));
    let block2 = rsc_miner::mine_block(&shard, block2).unwrap();
    let block2_hash = block2.hash;
    println!("PUSH 2 {:?}", shard.push(block2));

    let block3a = transaction_block(&shard, &private_key, block2_hash, wallet_hash);
    let block3a_hash = block3a.hash;
    println!("PUSH 3a {:?}", shard.push(block3a));

    let block4a = transaction_block(&shard, &private_key, block3a_hash, wallet_hash);
    let block4a_hash = block4a.hash;
    println!("PUSH 4a {:?}", shard.push(block4a));

    let block5a = transaction_block(&shard, &private_key, block4a_hash, wallet_hash);
    let block5a_hash = block5a.hash;
    println!("PUSH 5a {:?}", shard.push(block5a));

    /*
    let block6a = transaction_block(&shard, &private_key, block5a_hash, wallet_hash);
    let block6a_hash = block6a.hash;
    println!("PUSH 6a {:?}", shard.push(block6a));

    let block7a = transaction_block(&shard, &private_key, block6a_hash, wallet_hash);
    let block7a_hash = block7a.hash;
    println!("PUSH 7a {:?}", shard.push(block7a));
    */

    let block3b = transaction_block(&shard, &private_key, block2_hash, wallet_hash);
    let block3b_hash = block3b.hash;
    println!("PUSH 3b {:?}", shard.push(block3b));

    /*
    let block4b = transaction_block(&shard, &private_key, block3b_hash, wallet_hash);
    let block4b_hash = block4b.hash;
    println!("PUSH 4b {:?}", shard.push(block4b));

    let block5b = transaction_block(&shard, &private_key, block4b_hash, wallet_hash);
    let block5b_hash = block5b.hash;
    println!("PUSH 5b {} {:?}", block5b_hash, shard.push(block5b));

    let block_6 = transaction_block(&shard, &private_key, Hash::new(), wallet_hash);
    println!("PUSH 6 {:?}", shard.push(block_6));
    */

    println!("{}", shard);
}
