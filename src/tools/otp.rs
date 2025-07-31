use otpauth::TOTP;
use std::time::{SystemTime, UNIX_EPOCH};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// `UUID` uuid tools
pub struct OPTAuth {}

#[wasm_bindgen]
impl OPTAuth {
    pub fn generate_code(period: u64, secret: &str) -> u32 {
        if let Some(totp) = TOTP::from_base32(secret) {
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH);
            match timestamp {
                Ok(t) => totp.generate(period, t.as_secs()),
                Err(_) => 0,
            }
        } else {
            0
        }
    }
}

#[test]
fn totp_test() {
    let input = "us3whsg7x5kapv27vanwkqhf3sh3hull";
    let uppercase = input.replace(" ", "").to_uppercase();
    let result = OPTAuth::generate_code(30, &uppercase);
    println!("totp: {:?}", result);
    assert_ne!(0, result)
}
