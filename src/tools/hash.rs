use blake2::{Blake2b512, Blake2bMac512, Blake2s256};
use blake3;
use data_encoding::BASE64;
use digest::Digest;
use hkdf::Hkdf;
use hmac::{Hmac, Mac};
use md5::Md5;
use murmur3::{murmur3_32, murmur3_x64_128};
use sha2::Sha256;
use sha3::Sha3_256;
use sm3::Sm3;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// `Hash` hash tools
pub struct Hash {}

#[wasm_bindgen]
impl Hash {
    /// `generate_hash` hash wrapper
    fn generate_hash<D: Digest>(message: &str) -> String {
        let mut hasher = D::new();
        hasher.update(message);
        let hash = hasher.finalize();
        let hex_string = hash
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>();
        hex_string
    }

    /// `gen_md5` encode an MD5 hash
    pub fn gen_md5(message: &str) -> String {
        Self::generate_hash::<Md5>(message)
    }

    /// `gen_sha256` encode a SHA-256 hash
    pub fn gen_sha256(message: &str) -> String {
        Self::generate_hash::<Sha256>(message)
    }

    /// `gen_sha3_256` encode a SHA3-256 hash
    pub fn gen_sha3_256(message: &str) -> String {
        Self::generate_hash::<Sha3_256>(message)
    }

    /// `gen_sm3` encode a SM3 hash
    pub fn gen_sm3(message: &str) -> String {
        Self::generate_hash::<Sm3>(message)
    }

    /// `gen_blake2s256` encode a BLAKE2s hash
    pub fn gen_blake2s256(message: &str) -> String {
        Self::generate_hash::<Blake2s256>(message)
    }

    /// `gen_blake2b512` encode a BLAKE2b hash
    pub fn gen_blake2b512(message: &str) -> String {
        Self::generate_hash::<Blake2b512>(message)
    }

    /// `gen_blake3` encode a BLAKE3 hash
    pub fn gen_blake3(message: &str) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(message.as_bytes());
        let hash = hasher.finalize();
        let hex_string = hash.to_string();
        hex_string
    }

    /// `gen_murmur32` encode a MurmurHash3-32 hash
    pub fn gen_murmur32(message: &str) -> Option<String> {
        let mut input_reader = message.as_bytes();
        let hash_result = murmur3_32(&mut input_reader, 0);
        match hash_result {
            Ok(result) => Some(result.to_string()),
            Err(_) => None,
        }
    }

    /// `gen_murmur128` encode a MurmurHash3-128 hash
    pub fn gen_murmur128(message: &str) -> Option<String> {
        let mut input_reader = message.as_bytes();
        let hash_result = murmur3_x64_128(&mut input_reader, 0);

        match hash_result {
            Ok(result) => {
                let low: u64 = result as u64;
                let high: u64 = (result >> 64) as u64;
                let low_string = format!("{:016x}", low);
                let high_string = format!("{:016x}", high);
                let hash_string = format!("{}{}", low_string, high_string);
                Some(hash_string)
            }
            Err(_) => None,
        }
    }

    /// `gen_hkdf` encode a hkdf hash
    pub fn gen_hkdf(ikm: &[u8], salt: &[u8], info: &[u8], out_len: usize) -> Option<Vec<u8>> {
        let salt_option = if salt.is_empty() { None } else { Some(salt) };
        let hk = Hkdf::<Sha256>::new(salt_option, ikm);
        let mut okm = vec![0u8; out_len];
        if hk.expand(info, &mut okm).is_ok() {
            Some(okm)
        } else {
            None
        }
    }

    /// `gen_hkdf_b64` encode a hkdf base64
    pub fn gen_hkdf_b64(ikm: &[u8], salt: &[u8], info: &[u8], out_len: usize) -> Option<String> {
        let key = Hash::gen_hkdf(ikm, salt, info, out_len);
        if let Some(key) = key {
            Some(BASE64.encode(&key))
        } else {
            None
        }
    }

    /// `gen_hmac_sha256` encode a HMAC-SHA256 hash
    pub fn gen_hmac_sha256(key: &[u8], message: &str) -> String {
        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
        mac.update(message.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        let hex_string = code_bytes
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>();
        hex_string
    }

    /// `verify_hmac_sha256` verify a HMAC-SHA256 hash
    pub fn verify_hmac_sha256(key: &[u8], message: &str, expected_hmac: &str) -> bool {
        let generated_hmac = Self::gen_hmac_sha256(key, message);
        generated_hmac == expected_hmac
    }

    /// `gen_hmac_blake2b512` encode a HMAC-BLAKE2b-512 hash
    pub fn gen_hmac_blake2b512(key: &[u8], message: &str) -> String {
        let mut mac = Blake2bMac512::new_from_slice(key).expect("HMAC can take key of any size");
        mac.update(message.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        let hex_string = code_bytes
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>();
        hex_string
    }

    /// `verify_hmac_blake2b512` verify a HMAC-BLAKE2b-512 hash
    pub fn verify_hmac_blake2b512(key: &[u8], message: &str, expected_hmac: &str) -> bool {
        let generated_hmac = Self::gen_hmac_blake2b512(key, message);
        generated_hmac == expected_hmac
    }
}

#[test]
fn md5_test() {
    let result = Hash::gen_md5("hello world");
    println!("md5: {:?}", result);
    assert_eq!("5eb63bbbe01eeed093cb22bb8f5acdc3", result)
}

#[test]
fn sha256_test() {
    let result = Hash::gen_sha256("hello world");
    println!("sha256: {:?}", result);
    assert_eq!(
        "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
        result
    )
}

#[test]
fn sha3_256_test() {
    let result = Hash::gen_sha3_256("hello world");
    println!("sha3_256: {:?}", result);
    assert_eq!(
        "644bcc7e564373040999aac89e7622f3ca71fba1d972fd94a31c3bfbf24e3938",
        result
    )
}

#[test]
fn sm3_test() {
    let result = Hash::gen_sm3("hello world");
    println!("sm3: {:?}", result);
    assert_eq!(
        "44f0061e69fa6fdfc290c494654a05dc0c053da7e5c52b84ef93a9d67d3fff88",
        result
    )
}

#[test]
fn blake2s256_test() {
    let result = Hash::gen_blake2s256("hello world");
    println!("blake2s_256: {:?}", result);
    assert_eq!(
        "9aec6806794561107e594b1f6a8a6b0c92a0cba9acf5e5e93cca06f781813b0b",
        result
    )
}

#[test]
fn blake2b512_test() {
    let result = Hash::gen_blake2b512("hello world");
    println!("blake2b512: {:?}", result);
    assert_eq!(
        "021ced8799296ceca557832ab941a50b4a11f83478cf141f51f933f653ab9fbcc05a037cddbed06e309bf334942c4e58cdf1a46e237911ccd7fcf9787cbc7fd0",
        result
    )
}

#[test]
fn blake3_test() {
    let result = Hash::gen_blake3("hello world");
    println!("blake3: {:?}", result);
    assert_eq!(
        "d74981efa70a0c880b8d8c1985d075dbcbf679b99a5f9914e5aaf96b831a9e24",
        result
    )
}

#[test]
fn murmur3_32_test() {
    let result = Hash::gen_murmur32("hello world");
    println!("murmur32: {:?}", result);
    assert!(result.is_some());
    assert_eq!("1586663183", result.unwrap());
}

#[test]
fn murmur3_128_test() {
    let result = Hash::gen_murmur128("hello world");
    println!("murmur128: {:?}", result);
    assert!(result.is_some());
    assert_eq!("533f6046eb7f610eab97467d60eb63b1", result.unwrap());
}

#[test]
fn hkdf_test() {
    let ikm = b"hello input";
    let salt = b"hello salt";
    let info = b"hello info";
    let out_len = 32;

    let result = Hash::gen_hkdf_b64(ikm, salt, info, out_len);
    println!("hkdf: {:?}", result);
    assert!(result.is_some());
    assert_eq!(
        "A0oeefGWqceKVpXoudSQQAf96rxzkSPcPOLpao2eQ3A=",
        result.unwrap()
    );
}

#[test]
fn hmac_sha256_test() {
    let key = b"my secret and secure key";
    let message = "input message";
    let expected_hmac = "97d2a569059bbcd8ead4444ff99071f4c01d005bcefe0d3567e1be628e5fdcd9";
    let generated_hmac = Hash::gen_hmac_sha256(key, message);
    println!("hmac_sha256: {:?}", generated_hmac);
    assert_eq!(expected_hmac, generated_hmac);
    let is_valid = Hash::verify_hmac_sha256(key, message, expected_hmac);
    assert!(is_valid);
}

#[test]
fn hmac_blake2b512_test() {
    let key = b"my secret and secure key";
    let message = "input message";
    let expected_hmac = "5e48e32976b5b2b2900df7d1fd75377b6bae49e3aca630e44e1f3db44981fa4812d2b3a348acf01631a5173c18b55421cb380aa59bf680f1d398c76dc806d411";
    let generated_hmac = Hash::gen_hmac_blake2b512(key, message);
    println!("hmac_blake2b512: {:?}", generated_hmac);
    assert_eq!(expected_hmac, generated_hmac);
    let is_valid = Hash::verify_hmac_blake2b512(key, message, expected_hmac);
    assert!(is_valid);
}
