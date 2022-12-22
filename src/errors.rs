use std::{array::TryFromSliceError, net::AddrParseError, num::ParseIntError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    BencodeParseError(String),
    InvalidKRPC,
    InvalidKey(String),
    InvalidValue,
    InvalidNetAddr(String),
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Self::BencodeParseError(e.to_string())
    }
}

impl From<AddrParseError> for Error {
    fn from(e: AddrParseError) -> Self {
        Self::InvalidNetAddr(e.to_string())
    }
}

impl From<TryFromSliceError> for Error {
    fn from(e: TryFromSliceError) -> Self {
        Self::InvalidKey(e.to_string())
    }
}
