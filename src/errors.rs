use std::num::ParseIntError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    BencodeParseError(String),
    InvalidKRPC,
    InvalidValue,
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Self::BencodeParseError(e.to_string())
    }
}
