use wasm_bindgen::prelude::*;

use base64_light::base64_encode_bytes;
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng, RngCore};

#[wasm_bindgen]
/// `Password` password tools
pub struct Password {
    rng: ThreadRng,
}

#[wasm_bindgen]
impl Password {
    pub fn new() -> Self {
        let rng = thread_rng();
        Password { rng: rng }
    }

    /// `get_random` random char
    fn get_random(&mut self, min: u8, max: u8) -> u8 {
        self.rng.gen_range(min..max)
    }

    /// `get_random_upper` randomly generate a upper case letter
    pub fn get_random_upper(&mut self) -> char {
        self.get_random(b'A', b'Z') as char
    }

    /// `get_random_lower` randomly generate a lower case letters
    pub fn get_random_lower(&mut self) -> char {
        self.get_random(b'a', b'z') as char
    }

    /// `get_random_number` randomly generate a number
    pub fn get_random_number(&mut self) -> char {
        self.get_random(b'0', b'9') as char
    }

    /// `get_random_symbol` randomly generate a symbol
    pub fn get_random_symbol(&mut self) -> char {
        let symbols = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
        let index = self.get_random(0, symbols.len() as u8) as usize;
        symbols.chars().nth(index).unwrap()
    }

    /// `get_random_bytes` randomly generate [len] bytes
    pub fn get_random_bytes(&self, len: usize) -> String {
        let mut random_bytes_array = vec![0u8; len];
        rand::thread_rng().fill_bytes(&mut random_bytes_array);
        base64_encode_bytes(&random_bytes_array)
    }

    /// `get_random_password` randomly password
    pub fn get_random_password(
        &mut self,
        has_upper: bool,
        has_lower: bool,
        has_number: bool,
        has_symbol: bool,
        length: usize,
    ) -> Result<String, JsValue> {
        let mut char_pool = String::new();
        let mut password: Vec<char> = Vec::with_capacity(length);

        if has_upper {
            char_pool.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
            password.push(self.get_random_upper())
        }
        if has_lower {
            char_pool.push_str("abcdefghijklmnopqrstuvwxyz");
            password.push(self.get_random_lower())
        }
        if has_number {
            char_pool.push_str("0123456789");
            password.push(self.get_random_number())
        }
        if has_symbol {
            char_pool.push_str("!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~");
            password.push(self.get_random_symbol())
        }

        if char_pool.is_empty() {
            return Err(JsValue::from_str(
                "At least one character type must be included.",
            ));
        }

        for _ in 0..(length - password.len()) {
            password.push(*char_pool.as_bytes().choose(&mut self.rng).unwrap() as char);
        }

        password.shuffle(&mut self.rng);
        Ok(password.iter().collect())
    }
}

#[test]
fn password_random_basic_test() {
    let mut password = Password::new();
    let upper = password.get_random_upper();
    let lower = password.get_random_lower();
    let number = password.get_random_number();
    let symbol = password.get_random_symbol();
    let bytesp = password.get_random_bytes(16);
    println!(
        "upper: {} lower: {} number: {} symbol: {} bytes: {} ",
        upper, lower, number, symbol, bytesp
    );
}

#[test]
fn password_random_suit_test() {
    let mut password = Password::new();
    let rpass = password.get_random_password(false, false, false, false, 16);
    match rpass {
        Ok(p) => println!("password: {}", p),
        Err(e) => eprintln!("{}", e.is_string()),
    }
}
