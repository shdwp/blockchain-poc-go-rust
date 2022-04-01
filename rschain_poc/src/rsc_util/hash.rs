use core::fmt;
use serde::{Deserialize, Serialize};
use thiserror;
use std::ops::Index;

use sha2::digest::generic_array::{GenericArray, ArrayLength};

#[derive(thiserror::Error, Debug)]
pub enum HashError {
    #[error("Invalid size")]
    InvalidSize,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Hash {
    data: [u8; 32]
}

impl Hash {
    pub fn new() -> Hash {
        Hash { data: [0; 32] }
    }

    pub fn to_ne_bytes(&self) -> [u8; 32] {
        self.data
    }
}

impl TryFrom<Vec<u8>> for Hash {
    type Error = anyhow::Error;

    fn try_from(value: Vec<u8>) -> anyhow::Result<Self> {
        Ok(value
            .try_into()
            .map(|data| Hash { data })
            .map_err(|_| HashError::InvalidSize)?)
    }
}

impl<T: ArrayLength<u8>> TryFrom<GenericArray<u8, T>> for Hash {
    type Error = HashError;

    fn try_from(value: GenericArray<u8, T>) -> Result<Self, Self::Error> {
        Ok(value
            .as_slice()
            .try_into()
            .map(|data| Hash { data })
            .map_err(|_| HashError::InvalidSize)?)
    }
}

impl TryFrom<&String> for Hash {
    type Error = anyhow::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        hex::decode(value).and_then(|data| Ok(data.try_into()))?
    }
}

impl From<Hash> for String {
    fn from(h: Hash) -> Self {
        hex::encode(h.data)
    }
}

pub trait Hashable {
    fn hash(&self) -> Hash;
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", hex_fmt::HexFmt(self.data)))
    }
}

impl Index<usize> for Hash {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
