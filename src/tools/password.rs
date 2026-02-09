use rand::rngs::OsRng;
use rand::seq::SliceRandom;
use wasm_bindgen::prelude::*;

const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBER: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*()-_+=";

#[wasm_bindgen]
pub struct Password {
    rng: OsRng,
}

#[wasm_bindgen]
impl Password {
    pub fn new() -> Self {
        Self { rng: OsRng }
    }

    fn pick(&mut self, set: &[u8]) -> u8 {
        *set.choose(&mut self.rng).expect("empty charset")
    }

    pub fn get_random_password(
        &mut self,
        has_upper: bool,
        has_lower: bool,
        has_number: bool,
        has_symbol: bool,
        length: usize,
    ) -> String {
        let mut pool: Vec<u8> = Vec::new();
        let mut password: Vec<u8> = Vec::with_capacity(length);

        if has_upper {
            pool.extend_from_slice(UPPERCASE);
            password.push(self.pick(UPPERCASE));
        }
        if has_lower {
            pool.extend_from_slice(LOWERCASE);
            password.push(self.pick(LOWERCASE));
        }
        if has_number {
            pool.extend_from_slice(NUMBER);
            password.push(self.pick(NUMBER));
        }
        if has_symbol {
            pool.extend_from_slice(SYMBOLS);
            password.push(self.pick(SYMBOLS));
        }

        if pool.is_empty() || length < password.len() {
            return String::new();
        }

        for _ in password.len()..length {
            if let Some(c) = pool.choose(&mut self.rng) {
                password.push(*c);
            } else {
                return String::new();
            }
        }

        password.shuffle(&mut self.rng);

        String::from_utf8(password).unwrap_or_default()
    }
}

#[test]
fn password_random_basic_test() {
    let mut password = Password::new();
    let len = 16;
    let rpass = password.get_random_password(true, true, true, true, len);
    println!("random password: {}", rpass);
    assert_eq!(rpass.len(), len);
}
