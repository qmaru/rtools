use wasm_bindgen::prelude::*;

use base64_light::base64_encode_bytes;
use rand::{thread_rng, Rng, RngCore};

#[wasm_bindgen]
/// `Password` password tools
pub struct Password {}

#[wasm_bindgen]
impl Password {
    /// `get_random` random char
    fn get_random(len: f32, offset: i32) -> i32 {
        let mut rng = thread_rng();
        let base_code: i32 = (rng.gen_range(0f32..1f32) * len).floor() as i32;
        let char_code = base_code + offset;
        char_code
    }

    /// `get_random_upper` randomly generate a upper case letter
    pub fn get_random_upper() -> String {
        let result = Self::get_random(26f32, 65) as u8 as char;
        String::from(result)
    }

    /// `get_random_lower` randomly generate a lower case letters
    pub fn get_random_lower() -> String {
        let result = Self::get_random(26f32, 97) as u8 as char;
        String::from(result)
    }

    /// `get_random_number` randomly generate a number
    pub fn get_random_number() -> String {
        let result = Self::get_random(10f32, 48) as u8 as char;
        String::from(result)
    }

    /// `get_random_symbol` randomly generate a symbol
    pub fn get_random_symbol() -> String {
        let symbol = "!@#$%^&*(){}[]=<>/,.";
        let len = symbol.len() as f32;
        let offset = 0;
        let index = Self::get_random(len, offset) as usize;
        let result = symbol.chars().nth(index).unwrap();
        String::from(result)
    }

    /// `get_random_bytes` randomly generate [len] bytes
    pub fn get_random_bytes(len: usize) -> String {
        let mut random_bytes_array = vec![0u8; len];
        rand::thread_rng().fill_bytes(&mut random_bytes_array);
        base64_encode_bytes(&random_bytes_array)
    }
}

#[test]
fn password_random_test() {
    let upper = Password::get_random_upper();
    let lower = Password::get_random_lower();
    let number = Password::get_random_number();
    let symbol = Password::get_random_symbol();
    let bytesp = Password::get_random_bytes(16);
    println!(
        "upper: {} lower: {} number: {} symbol: {} bytes: {}",
        upper, lower, number, symbol, bytesp
    );
}
