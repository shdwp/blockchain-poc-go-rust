use openssl::{pkey::{PKey, Public, Private}, sign::{Verifier, Signer}, hash::MessageDigest};

use crate::rsc_util::error::BlockchainError;

pub fn sign(key: &PKey<Private>, data: &[u8]) -> Result<Vec<u8>, BlockchainError> {
    match Signer::new(MessageDigest::sha256(), key) {
        Ok(mut s) => {
            s.update(data).unwrap();
            s.sign_to_vec().map_err(|err| err.to_string().into())
        }

        Err(err) => Err(err.to_string().into()),
    }
}

pub fn check(key: &PKey<Public>, data: &[u8], signature: &[u8]) -> Result<bool, BlockchainError> {
    let mut v = Verifier::new(MessageDigest::sha256(), key).unwrap();
    v.update(data).unwrap();

    Result::Ok(v.verify(signature).unwrap_or(false))
}
