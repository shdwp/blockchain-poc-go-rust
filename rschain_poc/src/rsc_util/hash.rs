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

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ByteHash {
    data: [u8; 32]
}

impl ByteHash {
    pub fn new() -> ByteHash {
        ByteHash { data: [0; 32] }
    }

    pub fn to_ne_bytes(&self) -> [u8; 32] {
        self.data
    }
}

impl TryFrom<Vec<u8>> for ByteHash {
    type Error = anyhow::Error;

    fn try_from(value: Vec<u8>) -> anyhow::Result<Self> {
        Ok(value
            .try_into()
            .map(|data| ByteHash { data })
            .map_err(|_| HashError::InvalidSize)?)
    }
}

impl<T: ArrayLength<u8>> TryFrom<GenericArray<u8, T>> for ByteHash {
    type Error = HashError;

    fn try_from(value: GenericArray<u8, T>) -> Result<Self, Self::Error> {
        Ok(value
            .as_slice()
            .try_into()
            .map(|data| ByteHash { data })
            .map_err(|_| HashError::InvalidSize)?)
    }
}

impl TryFrom<&String> for ByteHash {
    type Error = anyhow::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        hex::decode(value).and_then(|data| Ok(data.try_into()))?
    }
}

impl From<ByteHash> for String {
    fn from(h: ByteHash) -> Self {
        hex::encode(h.data)
    }
}

pub trait Hashable {
    fn hash(&self) -> ByteHash;
}

impl fmt::Display for ByteHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", hex_fmt::HexFmt(self.data)))
    }
}

impl fmt::Debug for ByteHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", hex_fmt::HexFmt(self.data)))
    }
}

impl Index<usize> for ByteHash {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
