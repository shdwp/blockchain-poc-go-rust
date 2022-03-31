use std::{fmt, error::Error};

#[derive(Debug)]
pub struct BlockchainError {
    description: String
}

impl fmt::Display for BlockchainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.description)
    }
}

impl From<&str> for BlockchainError {
    fn from(s: &str) -> Self {
        BlockchainError { description: s.into() }
    }
}

impl From<String> for BlockchainError {
    fn from(s: String) -> Self {
        BlockchainError { description: s }
    }
}

impl Error for BlockchainError {
}
