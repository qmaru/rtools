use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};
use data_encoding::BASE64;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// `Secret` encryption / decryption
pub struct Secret {}

#[wasm_bindgen]
impl Secret {
    /// `chacha_encrypt` plaintext with given key_b64 (32 bytes) and nonce_b64 (12 bytes).
    pub fn chacha_encrypt(key_b64: &str, nonce_b64: &str, plaintext: &str) -> String {
        let key_bytes = match BASE64.decode(key_b64.as_bytes()) {
            Ok(k) => k,
            Err(_) => return "error: invalid base64 key".into(),
        };
        if key_bytes.len() != 32 {
            return "error: key length must be 32 bytes".into();
        }

        let nonce_bytes = match BASE64.decode(nonce_b64.as_bytes()) {
            Ok(n) => n,
            Err(_) => return "error: invalid base64 nonce".into(),
        };
        if nonce_bytes.len() != 12 {
            return "error: nonce length must be 12 bytes".into();
        }

        let key = Key::from_slice(&key_bytes);
        let cipher = ChaCha20Poly1305::new(key);
        let nonce = Nonce::from_slice(&nonce_bytes);

        match cipher.encrypt(nonce, plaintext.as_bytes()) {
            Ok(ct) => BASE64.encode(&ct),
            Err(_) => "error: encrypt failed".into(),
        }
    }

    /// `chacha_decrypt` ciphertext_b64 with given key_b64 and nonce_b64.
    pub fn chacha_decrypt(key_b64: &str, nonce_b64: &str, ciphertext_b64: &str) -> String {
        let key_bytes = match BASE64.decode(key_b64.as_bytes()) {
            Ok(k) => k,
            Err(_) => return "error: invalid base64 key".into(),
        };
        if key_bytes.len() != 32 {
            return "error: key length must be 32 bytes".into();
        }

        let nonce_bytes = match BASE64.decode(nonce_b64.as_bytes()) {
            Ok(n) => n,
            Err(_) => return "error: invalid base64 nonce".into(),
        };
        if nonce_bytes.len() != 12 {
            return "error: nonce length must be 12 bytes".into();
        }

        let ciphertext = match BASE64.decode(ciphertext_b64.as_bytes()) {
            Ok(c) => c,
            Err(_) => return "error: invalid base64 ciphertext".into(),
        };

        let key = Key::from_slice(&key_bytes);
        let cipher = ChaCha20Poly1305::new(key);
        let nonce = Nonce::from_slice(&nonce_bytes);

        match cipher.decrypt(nonce, ciphertext.as_ref()) {
            Ok(pt) => match String::from_utf8(pt) {
                Ok(s) => s,
                Err(_) => "error: decrypted plaintext not utf-8".into(),
            },
            Err(_) => "error: decrypt failed".into(),
        }
    }
}

#[test]
fn totp_test() {
    let key_bytes = vec![0u8; 32]; // 32 zero bytes
    let nonce_bytes = (0u8..12u8).collect::<Vec<u8>>(); // 0..11

    let plaintext = "Hello ChaCha20-Poly1305 with TOTP!";
    let key_b64 = BASE64.encode(&key_bytes);
    let nonce_b64 = BASE64.encode(&nonce_bytes);

    let ct_b64 = Secret::chacha_encrypt(&key_b64, &nonce_b64, plaintext);
    assert!(!ct_b64.starts_with("error:"), "encrypt failed: {}", ct_b64);

    let recovered = Secret::chacha_decrypt(&key_b64, &nonce_b64, &ct_b64);
    assert!(
        !recovered.starts_with("error:"),
        "decrypt failed: {}",
        recovered
    );
    assert_eq!(recovered, plaintext);
}
