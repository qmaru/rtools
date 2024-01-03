use base64_light::{base64_decode_str, base64_encode};
use blake2::{Blake2b512, Blake2s256};
use blake3;
use digest::Digest;
use md5::Md5;
use murmur3::{murmur3_32, murmur3_x64_128};
use pinyin::ToPinyin;
use sha2::Sha256;
use sha3::Sha3_256;
use sm3::Sm3;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use zxcvbn::zxcvbn;

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

#[wasm_bindgen]
pub fn gen_md5(message: &str) -> String {
    generate_hash::<Md5>(message)
}

#[wasm_bindgen]
pub fn gen_sha256(message: &str) -> String {
    generate_hash::<Sha256>(message)
}

#[wasm_bindgen]
pub fn gen_sha3_256(message: &str) -> String {
    generate_hash::<Sha3_256>(message)
}

#[wasm_bindgen]
pub fn gen_sm3(message: &str) -> String {
    generate_hash::<Sm3>(message)
}

#[wasm_bindgen]
pub fn gen_blake2s256(message: &str) -> String {
    generate_hash::<Blake2s256>(message)
}

#[wasm_bindgen]
pub fn gen_blake2b512(message: &str) -> String {
    generate_hash::<Blake2b512>(message)
}

#[wasm_bindgen]
pub fn gen_blake3(message: &str) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(message.as_bytes());
    let hash = hasher.finalize();
    let hex_string = hash.to_string();
    hex_string
}

#[wasm_bindgen]
pub fn gen_murmur32(message: &str) -> String {
    let mut input_reader = message.as_bytes();
    let hash_result = murmur3_32(&mut input_reader, 0);
    match hash_result {
        Ok(result) => result.to_string(),
        Err(_) => String::new(),
    }
}

#[wasm_bindgen]
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

#[wasm_bindgen]
pub fn passwd_strength(password: &str) -> String {
    let estimate = zxcvbn(password, &[]);
    match estimate {
        Ok(result) => result.score().to_string(),
        Err(_) => String::new(),
    }
}

#[wasm_bindgen]
pub fn han_to_pinyin(hans: &str) -> String {
    let mut results = String::new();
    for pinyin in hans.to_pinyin() {
        if let Some(pinyin) = pinyin {
            let result = pinyin.plain();
            results = format!("{}{}", results, result);
        }
    }
    results
}

#[wasm_bindgen]
pub fn uuidv4() -> String {
    let id = Uuid::new_v4();
    id.to_string()
}

#[wasm_bindgen]
pub fn b64encode(content: &str) -> String {
    base64_encode(content)
}

#[wasm_bindgen]
pub fn b64decode(content: &str) -> String {
    base64_decode_str(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn md5_test() {
        let result = gen_md5("hello world");
        println!("md5: {:?}", result);
        assert_eq!("5eb63bbbe01eeed093cb22bb8f5acdc3", result)
    }

    #[test]
    fn sha256_test() {
        let result = gen_sha256("hello world");
        println!("sha256: {:?}", result);
        assert_eq!(
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
            result
        )
    }

    #[test]
    fn sha3_256_test() {
        let result = gen_sha3_256("hello world");
        println!("sha3_256: {:?}", result);
        assert_eq!(
            "644bcc7e564373040999aac89e7622f3ca71fba1d972fd94a31c3bfbf24e3938",
            result
        )
    }

    #[test]
    fn sm3_test() {
        let result = gen_sm3("hello world");
        println!("sm3: {:?}", result);
        assert_eq!(
            "44f0061e69fa6fdfc290c494654a05dc0c053da7e5c52b84ef93a9d67d3fff88",
            result
        )
    }

    #[test]
    fn blake2s256_test() {
        let result = gen_blake2s256("hello world");
        println!("blake2s_256: {:?}", result);
        assert_eq!(
            "9aec6806794561107e594b1f6a8a6b0c92a0cba9acf5e5e93cca06f781813b0b",
            result
        )
    }

    #[test]
    fn blake2b512_test() {
        let result = gen_blake2b512("hello world");
        println!("blake2b512: {:?}", result);
        assert_eq!(
            "021ced8799296ceca557832ab941a50b4a11f83478cf141f51f933f653ab9fbcc05a037cddbed06e309bf334942c4e58cdf1a46e237911ccd7fcf9787cbc7fd0",
            result
        )
    }

    #[test]
    fn blake3_test() {
        let result = gen_blake3("hello world");
        println!("blake3: {:?}", result);
        assert_eq!(
            "d74981efa70a0c880b8d8c1985d075dbcbf679b99a5f9914e5aaf96b831a9e24",
            result
        )
    }

    #[test]
    fn murmur3_32_test() {
        let result = gen_murmur32("hello world");
        println!("murmur32: {:?}", result);
        assert_eq!("1586663183", result)
    }

    #[test]
    fn murmur3_128_test() {
        let result = gen_murmur128("hello world");
        println!("murmur128: {:?}", result);
        assert_eq!("533f6046eb7f610eab97467d60eb63b1", result)
    }

    #[test]
    fn passwd_strength_test() {
        let result = passwd_strength("123456");
        println!("strength: {:?}", result);
        assert_eq!("", result)
    }

    #[test]
    fn han_to_pinyin_test() {
        let result = han_to_pinyin("你好");
        println!("Pinyin: {:?}", result);
        assert_eq!("nihao", result)
    }

    #[test]
    fn uuid_test() {
        let result = uuidv4();
        println!("UUID: {:?}", result);
        assert_ne!("", result)
    }

    #[test]
    fn b64_encode_test() {
        let result = base64_encode("你好, world");
        println!("base64 encode: {:?}", result);
        assert_eq!("5L2g5aW9LCB3b3JsZA==", result)
    }

    #[test]
    fn b64_decode_test() {
        let result: String = base64_decode_str("5L2g5aW9LCB3b3JsZA==");
        println!("base64 decode: {:?}", result);
        assert_eq!("你好, world", result)
    }
}
