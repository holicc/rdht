use rdht::errors::Error;
use rdht::hashmap;
use rdht::util::bencode;
use rdht::util::bencode::Value;
use std::borrow::BorrowMut;
use std::collections::HashMap;

#[test]
fn test_decode_int() {
    let r = bencode::decode("i54e".chars().peekable().borrow_mut());
    assert_eq!(r, Ok(Value::Integer(54)));

    let r = bencode::decode("i0e".chars().peekable().borrow_mut());
    assert_eq!(r, Ok(Value::Integer(0)));

    let r = bencode::decode("54e".chars().peekable().borrow_mut());
    assert_eq!(
        r,
        Err(Error::BencodeParseError(
            "invalid bencode format".to_string()
        ))
    );

    let r = bencode::decode("54".chars().peekable().borrow_mut());
    assert_eq!(
        r,
        Err(Error::BencodeParseError(
            "invalid bencode format".to_string()
        ))
    );

    let r = bencode::decode("i54".chars().peekable().borrow_mut());
    assert_eq!(
        r,
        Err(Error::BencodeParseError(
            "invalid bencode format".to_string()
        ))
    );

    let r = bencode::decode("ie".chars().peekable().borrow_mut());
    assert_eq!(
        r,
        Err(Error::BencodeParseError(
            "cannot parse integer from empty string".to_string()
        ))
    );
}

#[test]
fn test_decode_str() {
    let r = bencode::decode("5:hello".chars().peekable().borrow_mut());
    assert_eq!(r, Ok(Value::String(String::from("hello"))));

    let r = bencode::decode("5:hell".chars().peekable().borrow_mut());
    assert_eq!(
        r,
        Err(Error::BencodeParseError(
            "invalid string len want 5 got 4".to_string()
        ))
    );

    let r = bencode::decode("4:hello".chars().peekable().borrow_mut());
    assert_eq!(r, Ok(Value::String(String::from("hell"))));
}

#[test]
fn test_decode_list() {
    let r = bencode::decode("l5:hello5:worldi1234ee".chars().peekable().borrow_mut());
    assert_eq!(
        r,
        Ok(Value::List(vec![
            Value::String(String::from("hello")),
            Value::String(String::from("world")),
            Value::Integer(1234),
        ]))
    );

    let r = bencode::decode("l5:hello5:worldi1234e".chars().peekable().borrow_mut());
    assert_eq!(
        r,
        Err(Error::BencodeParseError(format!(
            "invalid list format of end"
        )))
    );

    let r = bencode::decode("l5:hell5:worldi1234e".chars().peekable().borrow_mut());
    assert_eq!(
        r,
        Err(Error::BencodeParseError(format!(
            "invalid bencode content: :worldi1234e"
        )))
    );
}

#[test]
fn test_decode_dict() {
    let r = bencode::decode(
        "d7:balancei1000e4:coin3:btc4:name5:jisene"
            .chars()
            .peekable()
            .borrow_mut(),
    );
    let mut w = HashMap::new();
    w.insert(String::from("name"), Value::String(String::from("jisen")));
    w.insert(String::from("coin"), Value::String(String::from("btc")));
    w.insert(String::from("balance"), Value::Integer(1000));

    assert_eq!(r, Ok(Value::Dict(w)));

    let r = bencode::decode(
        "d7:balancei1000e4:coin3:btc4:name5:jisen"
            .chars()
            .peekable()
            .borrow_mut(),
    );
    assert_eq!(
        r,
        Err(Error::BencodeParseError(format!(
            "invalid dict format of end"
        )))
    );

    let r = bencode::decode(
        "d1:ad2:id20:abcdefghij0123456789e1:q4:ping1:t2:aa1:y1:qe"
            .chars()
            .peekable()
            .borrow_mut(),
    );
    let mut w = HashMap::new();

    w.insert(String::from("y"), Value::String("q".into()));
    w.insert(String::from("q"), Value::String("ping".into()));
    w.insert(
        String::from("a"),
        Value::Dict(hashmap!["id".to_string() => Value::String("abcdefghij0123456789".into())]),
    );
    w.insert("t".into(), Value::String("aa".into()));
    assert_eq!(r, Ok(Value::Dict(w)));

    let r = bencode::decode(
        "d1:eli201e23:A Generic Error Ocurrede1:t2:aa1:y1:ee"
            .chars()
            .peekable()
            .borrow_mut(),
    );
    let mut w = HashMap::new();

    w.insert(String::from("y"), Value::String("e".into()));
    w.insert(
        String::from("e"),
        Value::List(vec![
            Value::Integer(201),
            Value::String("A Generic Error Ocurred".into()),
        ]),
    );
    w.insert("t".into(), Value::String("aa".into()));
    assert_eq!(r, Ok(Value::Dict(w)));
}
