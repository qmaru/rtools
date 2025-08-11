use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key, Nonce, XChaCha20Poly1305, XNonce,
};
use data_encoding::BASE64;

use wasm_bindgen::prelude::*;

fn decode_key(key_b64: &str) -> Result<Key, String> {
    let key_bytes = BASE64
        .decode(key_b64.as_bytes())
        .map_err(|_| "error: invalid base64 key")?;
    if key_bytes.len() != 32 {
        return Err("error: key length must be 32 bytes".into());
    }
    Ok(*Key::from_slice(&key_bytes))
}

fn decode_nonce(nonce_b64: &str, expected_len: usize) -> Result<Vec<u8>, String> {
    let nonce_bytes = BASE64
        .decode(nonce_b64.as_bytes())
        .map_err(|_| "error: invalid base64 nonce")?;
    if nonce_bytes.len() != expected_len {
        return Err(format!(
            "error: nonce length must be {} bytes",
            expected_len
        ));
    }
    Ok(nonce_bytes)
}

fn decode_ciphertext(ct_b64: &str) -> Result<Vec<u8>, String> {
    BASE64
        .decode(ct_b64.as_bytes())
        .map_err(|_| "error: invalid base64 ciphertext".into())
}

#[wasm_bindgen]
/// `Secret` encryption / decryption
pub struct Secret {}

#[wasm_bindgen]
impl Secret {
    /// `xchacha_encrypt` plaintext with given key_b64 (32 bytes) and nonce_b64 (24 bytes).
    pub fn xchacha_encrypt(key_b64: &str, nonce_b64: &str, plaintext: &str) -> String {
        let key = match decode_key(key_b64) {
            Ok(k) => k,
            Err(e) => return e,
        };
        let nonce_bytes = match decode_nonce(nonce_b64, 24) {
            Ok(n) => n,
            Err(e) => return e,
        };

        let cipher = XChaCha20Poly1305::new(&key);
        let nonce = XNonce::from_slice(&nonce_bytes);
        match cipher.encrypt(nonce, plaintext.as_bytes()) {
            Ok(ct) => BASE64.encode(&ct),
            Err(_) => "error: encrypt failed".into(),
        }
    }

    /// `xchacha_decrypt` ciphertext_b64 with given key_b64 and nonce_b64.
    pub fn xchacha_decrypt(key_b64: &str, nonce_b64: &str, ciphertext_b64: &str) -> String {
        let key = match decode_key(key_b64) {
            Ok(k) => k,
            Err(e) => return e,
        };
        let nonce_bytes = match decode_nonce(nonce_b64, 24) {
            Ok(n) => n,
            Err(e) => return e,
        };
        let ciphertext = match decode_ciphertext(ciphertext_b64) {
            Ok(c) => c,
            Err(e) => return e,
        };

        let cipher = XChaCha20Poly1305::new(&key);
        let nonce = XNonce::from_slice(&nonce_bytes);

        match cipher.decrypt(nonce, ciphertext.as_ref()) {
            Ok(pt) => match String::from_utf8(pt) {
                Ok(s) => s,
                Err(_) => "error: decrypted plaintext not utf-8".into(),
            },
            Err(_) => "error: decrypt failed".into(),
        }
    }

    /// `chacha_encrypt` plaintext with given key_b64 (32 bytes) and nonce_b64 (12 bytes).
    pub fn chacha_encrypt(key_b64: &str, nonce_b64: &str, plaintext: &str) -> String {
        let key = match decode_key(key_b64) {
            Ok(k) => k,
            Err(e) => return e,
        };
        let nonce_bytes = match decode_nonce(nonce_b64, 12) {
            Ok(n) => n,
            Err(e) => return e,
        };
        let cipher = ChaCha20Poly1305::new(&key);
        let nonce = Nonce::from_slice(&nonce_bytes);

        match cipher.encrypt(nonce, plaintext.as_bytes()) {
            Ok(ct) => BASE64.encode(&ct),
            Err(_) => "error: encrypt failed".into(),
        }
    }

    /// `chacha_decrypt` ciphertext_b64 with given key_b64 and nonce_b64.
    pub fn chacha_decrypt(key_b64: &str, nonce_b64: &str, ciphertext_b64: &str) -> String {
        let key = match decode_key(key_b64) {
            Ok(k) => k,
            Err(e) => return e,
        };
        let nonce_bytes = match decode_nonce(nonce_b64, 12) {
            Ok(n) => n,
            Err(e) => return e,
        };
        let ciphertext = match decode_ciphertext(ciphertext_b64) {
            Ok(c) => c,
            Err(e) => return e,
        };

        let cipher = ChaCha20Poly1305::new(&key);
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
fn chacha_test() {
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

#[test]
fn xchacha_test() {
    let key_bytes = vec![0u8; 32];
    let nonce_bytes = (0u8..24u8).collect::<Vec<u8>>();

    let plaintext = "Hello XChaCha20-Poly1305 in WASM!";
    let key_b64 = BASE64.encode(&key_bytes);
    let nonce_b64 = BASE64.encode(&nonce_bytes);

    let ct_b64 = Secret::xchacha_encrypt(&key_b64, &nonce_b64, plaintext);
    assert!(!ct_b64.starts_with("error:"), "encrypt failed: {}", ct_b64);

    let recovered = Secret::xchacha_decrypt(&key_b64, &nonce_b64, &ct_b64);
    assert!(
        !recovered.starts_with("error:"),
        "decrypt failed: {}",
        recovered
    );
    assert_eq!(recovered, plaintext);
}
