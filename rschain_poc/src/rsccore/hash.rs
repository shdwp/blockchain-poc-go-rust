use core::fmt;
use serde::{Deserialize, Serialize};
use std::{ops::Index, array};

use sha2::digest::generic_array::{GenericArray, ArrayLength};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Hash {
    data: [u8; 32]
}

impl Hash {
    pub fn empty() -> Hash {
        Hash { data: [0; 32] }
    }

    pub fn from_array<T: ArrayLength<u8>>(array: GenericArray<u8, T>) -> Hash {
        Hash { data: array.as_slice().try_into().unwrap() }
    }

    pub fn check_difficulty(&self, difficulty: usize) -> bool {
        for i in 0..difficulty {
            if self[i] != 0 {
                return false
            }
        }

        return true
    }

    pub fn to_ne_bytes(&self) -> [u8; 32] {
        self.data
    }
}

impl From<Vec<u8>> for Hash {
    fn from(vec: Vec<u8>) -> Self {
        Self {
            data: vec.try_into().unwrap()
        }
    }
}

impl From<&String> for Hash {
    fn from(s: &String) -> Self {
        let data = hex::decode(s).unwrap();
        data.into()
    }
}

pub trait Hashable {
    fn hash(&self) -> Hash;
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f.write_fmt(format_args!("{}", hex_fmt::HexFmt(self.data)));
        Result::Ok(())
    }
}

impl Index<usize> for Hash {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
