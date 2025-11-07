use otpauth::TOTP;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// `OTPAuth` otp auth
pub struct OTPAuth {}

#[wasm_bindgen]
impl OTPAuth {
    /// `generate_code` generate TOTP code
    pub fn generate_code(secret: &str, timestamp: u64, period: u64) -> u32 {
        if let Some(totp) = TOTP::from_base32(secret) {
            totp.generate(period, timestamp)
        } else {
            0
        }
    }
}

#[test]
fn totp_test() {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let input = "us3whsg7x5kapv27vanwkqhf3sh3hull";
    let secret = input.replace(" ", "").to_uppercase();
    let result = OTPAuth::generate_code(&secret, timestamp, 30);
    println!("totp: {:?}", result);
    assert_ne!(0, result)
}
