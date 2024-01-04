use blake2::{Blake2b512, Blake2s256};
use blake3;
use digest::Digest;
use md5::Md5;
use murmur3::{murmur3_32, murmur3_x64_128};
use sha2::Sha256;
use sha3::Sha3_256;
use sm3::Sm3;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Hash {}

#[wasm_bindgen]
impl Hash {
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

    pub fn gen_md5(message: &str) -> String {
        Self::generate_hash::<Md5>(message)
    }

    pub fn gen_sha256(message: &str) -> String {
        Self::generate_hash::<Sha256>(message)
    }

    pub fn gen_sha3_256(message: &str) -> String {
        Self::generate_hash::<Sha3_256>(message)
    }

    pub fn gen_sm3(message: &str) -> String {
        Self::generate_hash::<Sm3>(message)
    }

    pub fn gen_blake2s256(message: &str) -> String {
        Self::generate_hash::<Blake2s256>(message)
    }

    pub fn gen_blake2b512(message: &str) -> String {
        Self::generate_hash::<Blake2b512>(message)
    }

    pub fn gen_blake3(message: &str) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(message.as_bytes());
        let hash = hasher.finalize();
        let hex_string = hash.to_string();
        hex_string
    }

    pub fn gen_murmur32(message: &str) -> String {
        let mut input_reader = message.as_bytes();
        let hash_result = murmur3_32(&mut input_reader, 0);
        match hash_result {
            Ok(result) => result.to_string(),
            Err(_) => String::new(),
        }
    }

    pub fn gen_murmur128(message: &str) -> String {
        let mut input_reader = message.as_bytes();
        let hash_result = murmur3_x64_128(&mut input_reader, 0);

        match hash_result {
            Ok(result) => {
                let low: u64 = result as u64;
                let high: u64 = (result >> 64) as u64;
                let low_string = format!("{:016x}", low);
                let high_string = format!("{:016x}", high);
                let hash_string = format!("{}{}", low_string, high_string);
                hash_string
            }
            Err(_) => String::new(),
        }
    }
}
