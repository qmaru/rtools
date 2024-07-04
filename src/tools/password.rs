use wasm_bindgen::prelude::*;

use rand::{thread_rng, Rng};

#[wasm_bindgen]
/// `Password` password tools
pub struct Password {}

#[wasm_bindgen]
impl Password {
    fn get_random(len: f64, offset: i64) -> i64 {
        let mut rng = thread_rng();
        let base_code: i64 = (rng.gen_range(0f64, 1f64) * len).floor() as i64;
        let char_code = base_code + offset;
        char_code
    }

    pub fn get_random_upper() -> char {
        Self::get_random(26f64, 65) as u8 as char
    }

    pub fn get_random_lower() -> char {
        Self::get_random(26f64, 97) as u8 as char
    }

    pub fn get_random_number() -> char {
        Self::get_random(10f64, 48) as u8 as char
    }

    pub fn get_random_symbol() -> char {
        let symbol = "!@#$%^&*(){}[]=<>/,.";
        let len = symbol.len() as f64;
        let offset = 0;
        let index = Self::get_random(len, offset) as usize;
        symbol.chars().nth(index).unwrap()
    }
}

#[test]
fn password_random_test() {
    let upper = Password::get_random_upper();
    let lower = Password::get_random_lower();
    let number = Password::get_random_number();
    let symbol = Password::get_random_symbol();
    println!(
        "upper: {} lower: {} number: {} symbol: {}",
        upper, lower, number, symbol
    );
}
