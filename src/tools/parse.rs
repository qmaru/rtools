use base64_light::{base64_decode_str, base64_encode};
use sqids::Sqids;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// `Base64` base64 tools
pub struct Base64 {}

#[wasm_bindgen]
impl Base64 {
    /// `encode` encode content to base64
    pub fn encode(content: &str) -> String {
        base64_encode(content)
    }

    /// `decode` decode base64 string
    pub fn decode(content: &str) -> String {
        base64_decode_str(content)
    }
}

#[wasm_bindgen]
/// `SqIDs` Generate unique IDs from numbers
pub struct SqIDs {}

const MAX_SAFE_INTEGER: u64 = (1 << 53) - 1;

#[wasm_bindgen]
impl SqIDs {
    /// `encode` encode number to string
    pub fn encode(number: i64, length: u8) -> Result<String, JsValue> {
        if number < 0 {
            let errmsg =
                format!("Value {number} is not a valid integer, between 0 and {MAX_SAFE_INTEGER}");
            return Err(JsValue::from_str(&errmsg));
        }

        if number > MAX_SAFE_INTEGER as i64 {
            let errmsg =
                format!("Value {number} is not a valid integer, between 0 and {MAX_SAFE_INTEGER}");
            return Err(JsValue::from_str(&errmsg));
        }

        let mut numbers: Vec<u64> = Vec::new();
        for c in number.to_string().chars() {
            if let Some(number) = c.to_digit(10) {
                numbers.push(number as u64)
            }
        }

        let sqids = Sqids::builder().min_length(length).build();
        match sqids {
            Ok(sqids) => {
                let result = sqids.encode(&numbers);
                match result {
                    Ok(id) => Ok(id),
                    Err(err) => Err(JsValue::from_str(&err.to_string())),
                }
            }
            Err(err) => Err(JsValue::from_str(&err.to_string())),
        }
    }

    /// `decode` decode string to number
    pub fn decode(content: &str, length: u8) -> Option<String> {
        let sqids = Sqids::builder().min_length(length).build();
        match sqids {
            Ok(sqids) => {
                let numbers = sqids.decode(content);
                let string_numbers: Vec<String> =
                    numbers.iter().map(|&num| num.to_string()).collect();

                if string_numbers.len() != 0 {
                    return Some(string_numbers.join(""));
                }
                return None;
            }
            Err(_) => None,
        }
    }
}

#[test]
fn b64_encode_test() {
    let result = Base64::encode("你好, world");
    println!("base64 encode: {:?}", result);
    assert_eq!("5L2g5aW9LCB3b3JsZA==", result)
}

#[test]
fn b64_decode_test() {
    let result: String = Base64::decode("5L2g5aW9LCB3b3JsZA==");
    println!("base64 decode: {:?}", result);
    assert_eq!("你好, world", result)
}

#[test]
fn sqids_encode_test() {
    let result = SqIDs::encode(123, 0);
    match result {
        Ok(result) => {
            println!("sqids encode: {:?}", result);
            assert_eq!("86Rf07", result);
        }
        Err(err) => eprintln!("sqids error: {:?}", err),
    }
}

#[test]
fn sqids_decode_test() {
    let result = SqIDs::decode("86Rf07", 0);
    println!("sqids decode: {:?}", result);
    assert!(result.is_some());
    assert_eq!("123", result.unwrap());
}
