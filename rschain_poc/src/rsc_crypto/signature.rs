use openssl::{pkey::{PKey, Public, Private}, sign::{Verifier, Signer}, hash::MessageDigest};

pub fn sign(key: &PKey<Private>, data: &[u8]) -> anyhow::Result<Vec<u8>> {
    let mut signer = Signer::new(MessageDigest::sha256(), key)?;
    signer.update(data)?;
    return Ok(signer.sign_to_vec()?);
}

pub fn check(key: &PKey<Public>, data: &[u8], signature: &[u8]) -> anyhow::Result<bool> {
    let mut verifyer = Verifier::new(MessageDigest::sha256(), key)?;
    verifyer.update(data)?;
    return Ok(verifyer.verify(signature)?);
}
