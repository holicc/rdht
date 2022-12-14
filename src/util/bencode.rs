use std::{collections::BTreeMap, iter::Peekable};

use crate::errors::{Error, Result};

#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Integer(u64),
    List(Vec<Value>),
    Dict(BTreeMap<String, Value>),
}

impl Value {
    pub fn encode(self) -> Result<String> {
        match self {
            Value::String(s) => Self::encode_str(s),
            Value::Integer(i) => Ok(format!("i{}e", i)),
            Value::List(l) => Ok(format!(
                "l{}e",
                l.into_iter()
                    .filter_map(|v| v.encode().ok())
                    .collect::<Vec<String>>()
                    .join("")
            )),
            Value::Dict(d) => {
                let mut list = vec![];
                for (k, v) in d {
                    list.push(Self::encode_str(k)?);
                    list.push(v.encode()?);
                }
                Ok(format!("d{}e", list.join("")))
            }
        }
    }

    fn encode_str(s: String) -> Result<String> {
        if s.len() <= 0 {
            return Err(Error::InvalidValue);
        }
        Ok(format!("{}:{}", s.len(), s))
    }
}

impl TryFrom<&str> for Value {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Ok(Value::String(value.into()))
    }
}

impl TryInto<String> for Value {
    type Error = Error;

    fn try_into(self) -> Result<String> {
        if let Value::String(v) = self {
            return Ok(v);
        }
        Err(Error::InvalidValue)
    }
}

impl TryInto<u8> for Value {
    type Error = Error;

    fn try_into(self) -> Result<u8> {
        if let Value::Integer(v) = self {
            return Ok(v as u8);
        }
        Err(Error::InvalidValue)
    }
}

impl TryInto<u64> for Value {
    type Error = Error;

    fn try_into(self) -> Result<u64> {
        if let Value::Integer(v) = self {
            return Ok(v);
        }
        Err(Error::InvalidValue)
    }
}

impl TryInto<Vec<String>> for Value {
    type Error = Error;

    fn try_into(self) -> Result<Vec<String>> {
        if let Value::List(v) = self {
            return Ok(v.into_iter().filter_map(|x| x.try_into().ok()).collect());
        }
        Err(Error::InvalidValue)
    }
}

pub fn decode<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> Result<Value> {
    match chars.peek() {
        Some('i') => parse_int(chars),
        Some(c) if c.is_digit(10) => parse_str(chars),
        Some('d') => parse_dict(chars),
        Some('l') => parse_list(chars),
        c => Err(Error::BencodeParseError(format!(
            "invalid bencode content: {}",
            chars.collect::<String>()
        ))),
    }
}

fn parse_int<I: Iterator<Item = char>>(peek: &mut Peekable<I>) -> Result<Value> {
    let mut value = String::new();
    if peek.next_if_eq(&'i').is_none() {
        return Err(Error::BencodeParseError(format!("invalid bencode format")));
    }
    while let Some(c) = peek.next_if(|c| c.is_digit(10)) {
        value.push(c);
    }
    if let Some(_) = peek.next_if_eq(&'e') {
        return Ok(Value::Integer(value.parse::<u64>()?));
    }
    Err(Error::BencodeParseError(format!("invalid bencode format")))
}

fn parse_str<I: Iterator<Item = char>>(peek: &mut Peekable<I>) -> Result<Value> {
    let mut value = String::new();
    while let Some(c) = peek.next_if(|c| c.is_digit(10)) {
        value.push(c);
    }
    let size = value.parse::<usize>()?;
    if peek.next_if_eq(&':').is_none() {
        return Err(Error::BencodeParseError(format!("invalid bencode format")));
    }
    let value = peek.take(size).collect::<String>();
    if value.len() != size {
        return Err(Error::BencodeParseError(format!(
            "invalid string len want {} got {}",
            size,
            value.len()
        )));
    }
    Ok(Value::String(value))
}

fn parse_list<I: Iterator<Item = char>>(peek: &mut Peekable<I>) -> Result<Value> {
    if peek.next_if_eq(&'l').is_none() {
        return Err(Error::BencodeParseError(format!(
            "invalid list format of start"
        )));
    }
    let mut list = Vec::new();
    loop {
        match peek.peek() {
            Some(c) if *c == 'e' => {
                peek.next();
                break;
            }
            Some(_) => list.push(decode(peek)?),
            None => {
                return Err(Error::BencodeParseError(format!(
                    "invalid list format of end"
                )))
            }
        }
    }
    Ok(Value::List(list))
}

fn parse_dict<I: Iterator<Item = char>>(peek: &mut Peekable<I>) -> Result<Value> {
    if peek.next_if_eq(&'d').is_none() {
        return Err(Error::BencodeParseError(format!(
            "invalid dict format of start"
        )));
    }
    let mut map = BTreeMap::new();

    loop {
        match peek.peek() {
            Some(c) if *c == 'e' => {
                peek.next();
                break;
            }
            Some(_) => {
                if let Value::String(key) = decode(peek)? {
                    map.insert(key, decode(peek)?);
                }
            }
            None => {
                return Err(Error::BencodeParseError(format!(
                    "invalid dict format of end"
                )))
            }
        }
    }
    Ok(Value::Dict(map))
}
