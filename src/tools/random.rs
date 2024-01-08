use nanoid::nanoid;
use sqids::Sqids;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// `UUID` uuid tools
pub struct UUID {}

#[wasm_bindgen]
impl UUID {
    /// `v4` creates v4 UUIDs
    pub fn v4() -> String {
        let id = Uuid::new_v4();
        id.to_string()
    }
}

#[wasm_bindgen]
/// `NanoID` nanoid tools
pub struct NanoID {}

#[wasm_bindgen]
impl NanoID {
    /// `nanoid` a tiny, secure, URL-friendly, unique string ID
    pub fn nanoid(len: usize) -> String {
        let id = nanoid!(len);
        id
    }
}

#[wasm_bindgen]
/// `SqIDs` Generate unique IDs from numbers
pub struct SqIDs {}

#[wasm_bindgen]
impl SqIDs {
    /// `encode` encode number to string
    pub fn encode(number: u64) -> String {
        let mut numbers: Vec<u64> = Vec::new();
        for c in number.to_string().chars() {
            if let Some(number) = c.to_digit(10) {
                numbers.push(number as u64)
            } else {
                return String::new();
            }
        }

        let sqids = Sqids::default();
        let result = sqids.encode(&numbers);
        match result {
            Ok(id) => id,
            Err(_) => String::new(),
        }
    }

    /// `decode` decode string to number
    pub fn decode(content: &str) -> String {
        let sqids = Sqids::default();
        let numbers = sqids.decode(content);
        let string_numbers: Vec<String> = numbers.iter().map(|&num| num.to_string()).collect();
        string_numbers.join("")
    }
}

#[test]
fn uuid_v4_test() {
    let result = UUID::v4();
    println!("UUID: {:?}", result);
    assert_ne!("", result)
}

#[test]
fn nanoid_test() {
    let result = NanoID::nanoid(21);
    println!("nanoid: {:?}", result);
    assert_ne!("", result)
}

#[test]
fn sqids_encode_test() {
    let result = SqIDs::encode(123);
    println!("sqids encode: {:?}", result);
    assert_eq!("86Rf07", result)
}

#[test]
fn sqids_decode_test() {
    let result = SqIDs::decode("86Rf07");
    println!("sqids decode: {:?}", result);
    assert_eq!("123", result)
}
