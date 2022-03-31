use core::fmt;
use serde::{Deserialize, Serialize};
use std::{ops::Index};

use sha2::digest::generic_array::{GenericArray, ArrayLength};

use super::error::BlockchainError;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Hash {
    data: [u8; 32]
}

impl Hash {
    pub fn empty() -> Hash {
        Hash { data: [0; 32] }
    }

    pub fn to_ne_bytes(&self) -> [u8; 32] {
        self.data
    }
}

impl TryFrom<Vec<u8>> for Hash {
    type Error = BlockchainError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        value.try_into().map_or_else(|_| Err("invalid input size".into()), |data| Ok(Self { data }))
    }
}

impl<T: ArrayLength<u8>> From<GenericArray<u8, T>> for Hash {
    fn from(array: GenericArray<u8, T>) -> Self {
        Hash { data: array.as_slice().try_into().unwrap() }
    }
}

impl TryFrom<&String> for Hash {
    type Error = BlockchainError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        hex::decode(value).map_or_else(|e| Err(e.to_string().into()), |d| Hash::try_from(d))
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
