use openssl::{pkey::{PKey, Public, Private}, sign::{Verifier, Signer}, hash::MessageDigest, sha::sha256};

use super::chain::BlockchainError;


pub fn sign(key: &PKey<Private>, data: &[u8]) -> Result<Vec<u8>, BlockchainError> {
    let mut s = Signer::new(MessageDigest::sha256(), key).unwrap();
    s.update(data).unwrap();

    Result::Ok(s.sign_to_vec().unwrap())
}

pub fn check(key: &PKey<Public>, data: &[u8], signature: &[u8]) -> Result<bool, BlockchainError> {
    let mut v = Verifier::new(MessageDigest::sha256(), key).unwrap();
    v.update(data).unwrap();

    Result::Ok(v.verify(signature).unwrap_or(false))
}
