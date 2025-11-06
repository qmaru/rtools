use data_encoding::{BASE32, BASE32_NOPAD, BASE64, BASE64_NOPAD, HEXLOWER};
use idna::{domain_to_ascii, domain_to_unicode};
use sqids::Sqids;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// `DataEncoding` Data encoding/decoding tools
pub struct DataEncoding {}

#[wasm_bindgen]
impl DataEncoding {
    /// `encode64` encode string to base64
    pub fn encode64(content: &str) -> String {
        BASE64.encode(content.as_bytes())
    }

    /// `decode64` decode string from base64
    pub fn decode64(content: &str) -> String {
        let bytes = BASE64.decode(content.as_bytes()).unwrap_or_default();
        String::from_utf8(bytes).unwrap_or_default()
    }

    /// `encode64_nopad` encode string to base64 no-padding
    pub fn encode64_nopad(content: &str) -> String {
        BASE64_NOPAD.encode(content.as_bytes())
    }

    /// `decode64_nopad` decode string from base64 no-padding
    pub fn decode64_nopad(content: &str) -> String {
        let bytes = BASE64_NOPAD.decode(content.as_bytes()).unwrap_or_default();
        String::from_utf8(bytes).unwrap_or_default()
    }

    /// `encode64_bytes` encode bytes to base64
    pub fn encode64_bytes(content: &[u8]) -> String {
        BASE64.encode(content)
    }

    /// `decode64_bytes` decode bytes from base64
    pub fn decode64_bytes(content: &str) -> Vec<u8> {
        BASE64.decode(content.as_bytes()).unwrap_or_default()
    }

    /// `encode64_nopad_bytes` encode bytes to base64 no-padding
    pub fn encode64_nopad_bytes(content: &[u8]) -> String {
        BASE64_NOPAD.encode(content)
    }

    /// `decode64_nopad_bytes` decode bytes from base64 no-padding
    pub fn decode64_nopad_bytes(content: &str) -> Vec<u8> {
        BASE64_NOPAD.decode(content.as_bytes()).unwrap_or_default()
    }

    /// `encode32` encode string to base32
    pub fn encode32(content: &str) -> String {
        BASE32.encode(content.as_bytes())
    }

    /// `decode32` decode string from base32
    pub fn decode32(content: &str) -> String {
        let bytes = BASE32.decode(content.as_bytes()).unwrap_or_default();
        String::from_utf8(bytes).unwrap_or_default()
    }

    /// `encode32_nopad` encode string to base32 no-padding
    pub fn encode32_nopad(content: &str) -> String {
        BASE32_NOPAD.encode(content.as_bytes())
    }

    /// `decode32_nopad` decode string from base32 no-padding
    pub fn decode32_nopad(content: &str) -> String {
        let bytes = BASE32_NOPAD.decode(content.as_bytes()).unwrap_or_default();
        String::from_utf8(bytes).unwrap_or_default()
    }

    /// `encode32_bytes` encode bytes to base32
    pub fn encode32_bytes(content: &[u8]) -> String {
        BASE32.encode(content)
    }

    /// `decode32_bytes` decode bytes from base32
    pub fn decode32_bytes(content: &str) -> Vec<u8> {
        BASE32.decode(content.as_bytes()).unwrap_or_default()
    }

    /// `encode32_nopad_bytes` encode base32 no-padding bytes
    pub fn encode32_nopad_bytes(content: &[u8]) -> String {
        BASE32_NOPAD.encode(content)
    }

    /// `decode32_nopad_bytes` decode base32 no-padding bytes
    pub fn decode32_nopad_bytes(content: &str) -> Vec<u8> {
        BASE32_NOPAD.decode(content.as_bytes()).unwrap_or_default()
    }

    /// `encode_hex` encode hex string
    pub fn encode_hex(content: &str) -> String {
        HEXLOWER.encode(content.as_bytes())
    }

    /// `decode_hex` decode hex string
    pub fn decode_hex(content: &str) -> String {
        let bytes = HEXLOWER.decode(content.as_bytes()).unwrap_or_default();
        String::from_utf8(bytes).unwrap_or_default()
    }

    /// `encode_hex_bytes` encode hex bytes
    pub fn encode_hex_bytes(content: &[u8]) -> String {
        HEXLOWER.encode(content)
    }

    /// `decode_hex_bytes` decode hex bytes
    pub fn decode_hex_bytes(content: &str) -> Vec<u8> {
        HEXLOWER.decode(content.as_bytes()).unwrap_or_default()
    }

    /// `encode_punycode` encode domain
    pub fn encode_punycode(domain: &str) -> String {
        domain_to_ascii(domain).unwrap_or_default()
    }

    /// `decode_punycode` decode domain
    pub fn decode_punycode(domain: &str) -> String {
        let (result, err) = domain_to_unicode(domain);
        match err {
            Ok(()) => result,
            Err(_) => String::new(),
        }
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

                if !string_numbers.is_empty() {
                    return Some(string_numbers.join(""));
                }
                None
            }
            Err(_) => None,
        }
    }
}

#[test]
fn b64_encode_test() {
    let result = DataEncoding::encode64("你好, world");
    println!("base64 encode: {:?}", result);
    assert_eq!("5L2g5aW9LCB3b3JsZA==", result)
}

#[test]
fn b64_decode_test() {
    let result: String = DataEncoding::decode64("5L2g5aW9LCB3b3JsZA==");
    println!("base64 decode: {:?}", result);
    assert_eq!("你好, world", result)
}

#[test]
fn b64_nopad_encode_test() {
    let result = DataEncoding::encode64_nopad("你好, world");
    println!("base64 no-padding encode: {:?}", result);
    assert_eq!("5L2g5aW9LCB3b3JsZA", result);
}

#[test]
fn b64_nopad_decode_test() {
    let result = DataEncoding::decode64_nopad("5L2g5aW9LCB3b3JsZA");
    println!("base64 no-padding decode: {:?}", result);
    assert_eq!("你好, world", result);
}

#[test]
fn b64_bytes_test() {
    let bytes = "你好, world".as_bytes();
    let encoded = DataEncoding::encode64_bytes(bytes);
    println!("base64 encode bytes: {:?}", encoded);
    assert_eq!("5L2g5aW9LCB3b3JsZA==", encoded);

    let decoded = DataEncoding::decode64_bytes(&encoded);
    assert_eq!(bytes, decoded.as_slice());
}

#[test]
fn b64_nopad_bytes_test() {
    let bytes = "你好, world".as_bytes();
    let encoded = DataEncoding::encode64_nopad_bytes(bytes);
    println!("base64 no-padding encode bytes: {:?}", encoded);
    assert_eq!("5L2g5aW9LCB3b3JsZA", encoded);

    let decoded = DataEncoding::decode64_nopad_bytes(&encoded);
    assert_eq!(bytes, decoded.as_slice());
}

#[test]
fn b32_encode_test() {
    let result1 = DataEncoding::encode32("你好, world");
    let result2 = DataEncoding::encode32_nopad("你好, world");
    println!("base32 encode: {:?}", result1);
    println!("base32 encode no-padding: {:?}", result2);
    assert_eq!("4S62BZNFXUWCA53POJWGI===", result1);
    assert_eq!("4S62BZNFXUWCA53POJWGI", result2);
}

#[test]
fn b32_decode_test() {
    let result1: String = DataEncoding::decode32("4S62BZNFXUWCA53POJWGI===");
    let result2: String = DataEncoding::decode32_nopad("4S62BZNFXUWCA53POJWGI");
    println!("base32 decode: {:?}", result1);
    println!("base32 decode no-padding: {:?}", result2);
    assert_eq!("你好, world", result1);
    assert_eq!("你好, world", result2);
}

#[test]
fn b32_bytes_test() {
    let bytes = "你好, world".as_bytes();
    let encoded = DataEncoding::encode32_bytes(bytes);
    println!("base32 encode bytes: {:?}", encoded);
    assert_eq!("4S62BZNFXUWCA53POJWGI===", encoded);

    let decoded = DataEncoding::decode32_bytes(&encoded);
    assert_eq!(bytes, decoded.as_slice());
}

#[test]
fn b32_nopad_bytes_test() {
    let bytes = "你好, world".as_bytes();
    let encoded = DataEncoding::encode32_nopad_bytes(bytes);
    println!("base32 no-padding encode bytes: {:?}", encoded);
    assert_eq!("4S62BZNFXUWCA53POJWGI", encoded);

    let decoded = DataEncoding::decode32_nopad_bytes(&encoded);
    assert_eq!(bytes, decoded.as_slice());
}

#[test]
fn hex_encode_test() {
    let result = DataEncoding::encode_hex("你好, world");
    println!("hex encode: {:?}", result);
    assert_eq!("e4bda0e5a5bd2c20776f726c64", result);
}

#[test]
fn hex_decode_test() {
    let result = DataEncoding::decode_hex("e4bda0e5a5bd2c20776f726c64");
    println!("hex decode: {:?}", result);
    assert_eq!("你好, world", result);
}

#[test]
fn hex_bytes_test() {
    let bytes = "你好, world".as_bytes();
    let encoded = DataEncoding::encode_hex_bytes(bytes);
    println!("hex encode bytes: {:?}", encoded);
    assert_eq!("e4bda0e5a5bd2c20776f726c64", encoded);

    let decoded = DataEncoding::decode_hex_bytes(&encoded);
    assert_eq!(bytes, decoded.as_slice());
}

#[test]
fn punycode_encode_test() {
    let result = DataEncoding::encode_punycode("https://www.谷歌.com");
    println!("punycode encode: {:?}", result);
    assert_eq!("https://www.xn--flw351e.com", result);
}

#[test]
fn punycode_decode_test() {
    let result = DataEncoding::decode_punycode("https://www.xn--flw351e.com");
    println!("punycode decode: {:?}", result);
    assert_eq!("https://www.谷歌.com", result);
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
