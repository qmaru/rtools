use data_encoding::{BASE64, HEXLOWER};
use getrandom::getrandom;
use nanoid::nanoid;
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
/// `SafeRandom` safety random
pub struct SafeRandom {}

#[wasm_bindgen]
pub struct SafeBytes {
    data: Vec<u8>,
}

#[wasm_bindgen]
impl SafeBytes {
    pub fn raw(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn to_base64(&self) -> String {
        BASE64.encode(&self.data)
    }

    pub fn to_hex(&self) -> String {
        HEXLOWER.encode(&self.data)
    }
}

#[wasm_bindgen]
impl SafeRandom {
    /// `gen_bytes` generate a safety bytes
    pub fn gen_bytes(len: usize) -> SafeBytes {
        let mut buf = vec![0u8; len];
        getrandom(&mut buf).expect("Failed to generate random bytes");
        SafeBytes { data: buf }
    }

    /// `gen_secret` generate a 32-bytes secret
    pub fn gen_secret() -> SafeBytes {
        Self::gen_bytes(32)
    }

    /// `gen_nonce` generate a 12-bytes nonce
    pub fn gen_nonce() -> SafeBytes {
        Self::gen_bytes(12)
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
fn safety_test() {
    let result = SafeRandom::gen_nonce();
    println!("safety: {:?}", result.raw());
}
