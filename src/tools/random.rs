use crate::tools::parse::DataEncoding;
use String;
use getrandom::getrandom;
use nanoid::nanoid;
use std::time::{SystemTime, UNIX_EPOCH};
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

    /// `v7` creates v7 UUIDs based on the given milliseconds since UNIX epoch
    pub fn v7(ms: u64) -> String {
        let seconds = ms / 1000;
        let subsec_nanos = ((ms % 1000) * 1_000_000) as u32;
        Uuid::new_v7(uuid::Timestamp::from_unix_time(
            seconds,
            subsec_nanos,
            0,
            12,
        ))
        .to_string()
    }

    /// `v7_now` creates v7 UUIDs based on the current system time
    pub fn v7_now() -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        Uuid::new_v7(uuid::Timestamp::from_unix_time(
            now.as_secs(),
            now.subsec_nanos(),
            0,
            12,
        ))
        .to_string()
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
    /// `raw` get raw bytes
    pub fn raw(&self) -> Vec<u8> {
        self.data.clone()
    }

    /// `to_base32` encode to base32 string
    pub fn to_base32(&self) -> String {
        DataEncoding::encode32_bytes(&self.data)
    }

    /// `to_base32_nopad` encode to base32 no padding string
    pub fn to_base32_nopad(&self) -> String {
        DataEncoding::encode32_nopad_bytes(&self.data)
    }

    /// `to_base64` encode to base64 string
    pub fn to_base64(&self) -> String {
        DataEncoding::encode64_bytes(&self.data)
    }

    /// `to_base64_nopad` encode to base64 no padding string
    pub fn to_base64_nopad(&self) -> String {
        DataEncoding::encode64_nopad_bytes(&self.data)
    }

    /// `to_base64_url` encode to base64 url-safe string
    pub fn to_base64_url(&self) -> String {
        DataEncoding::encode64_url_bytes(&self.data)
    }

    /// `to_base64_url_nopad` encode to base64 url-safe no padding string
    pub fn to_base64_url_nopad(&self) -> String {
        DataEncoding::encode64_url_nopad_bytes(&self.data)
    }

    /// `to_hex` encode to hex string
    pub fn to_hex(&self) -> String {
        DataEncoding::encode_hex_bytes(&self.data)
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

    /// `gen_secret_16_bytes` generate a 16 bytes(128 bit) secret
    pub fn gen_secret_16_bytes() -> SafeBytes {
        Self::gen_bytes(16)
    }

    /// `gen_secret_20_bytes` generate a 20 bytes(160 bit) secret
    pub fn gen_secret_20_bytes() -> SafeBytes {
        Self::gen_bytes(20)
    }

    /// `gen_secret_32_bytes` generate a 32 bytes(256 bit) secret
    pub fn gen_secret_32_bytes() -> SafeBytes {
        Self::gen_bytes(32)
    }

    /// `gen_nonce_12_bytes` generate a 12 bytes(96 bit) nonce
    pub fn gen_nonce_12_bytes() -> SafeBytes {
        Self::gen_bytes(12)
    }

    /// `gen_nonce_16_bytes` generate a 16 bytes(128 bit) nonce
    pub fn gen_nonce_16_bytes() -> SafeBytes {
        Self::gen_bytes(16)
    }

    /// `gen_nonce_24_bytes` generate a 24 bytes(192 bit) nonce
    pub fn gen_nonce_24_bytes() -> SafeBytes {
        Self::gen_bytes(24)
    }
}

#[test]
fn uuid_v4_test() {
    let result = UUID::v4();
    println!("UUIDv4: {:?}", result);
    assert_ne!("", result)
}

#[test]
fn uuid_v7_test() {
    let result = UUID::v7(1762482616);
    println!("UUIDv7: {:?}", result);
    assert_ne!("", result);

    let result = UUID::v7_now();
    println!("UUIDv7 now: {:?}", result);
    assert_ne!("", result);
}

#[test]
fn nanoid_test() {
    let result = NanoID::nanoid(21);
    println!("nanoid: {:?}", result);
    assert_ne!("", result)
}

#[test]
fn safety_test() {
    let result = SafeRandom::gen_bytes(16);
    println!("safety: {:?}", result.raw());
    println!("safety: {:?}", result.to_base32());
    println!("safety: {:?}", result.to_base64());
    println!("safety: {:?}", result.to_hex());
}
