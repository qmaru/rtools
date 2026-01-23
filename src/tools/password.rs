use wasm_bindgen::prelude::*;

use data_encoding::{BASE64, BASE64_NOPAD, BASE64URL, BASE64URL_NOPAD};
use rand::{Rng, RngCore, rngs::ThreadRng, seq::SliceRandom, thread_rng};

const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const NUMBER: &str = "0123456789";
const SYMBOLS: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

#[wasm_bindgen]
#[derive(Clone, Copy)]
/// QR code version
pub enum EncodedingFormat {
    Base64 = "base64",
    Base64Nopad = "base64_nopad",
    Base64Url = "base64_url",
    Base64UrlNopad = "base64_url_nopad",
}

#[wasm_bindgen]
/// `Password` password tools
pub struct Password {
    rng: ThreadRng,
}

#[wasm_bindgen]
impl Password {
    pub fn new() -> Self {
        let rng = thread_rng();
        Password { rng }
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
        let index = self.get_random(0, SYMBOLS.len() as u8) as usize;
        let symbol = SYMBOLS.chars().nth(index);
        if let Some(symbol) = symbol {
            return symbol;
        }
        return ' ';
    }

    /// `get_random_bytes` randomly generate [len] bytes
    pub fn get_random_bytes(&self, len: usize) -> String {
        let mut random_bytes_array = vec![0u8; len];
        rand::thread_rng().fill_bytes(&mut random_bytes_array);
        BASE64.encode(&random_bytes_array)
    }

    /// `get_random_bytes_with_encoding` randomly generate [len] bytes with specified encoding
    /// Supported formats: ENCODING_BASE64, ENCODING_BASE64_NOPAD, ENCODING_BASE64_URL, ENCODING_BASE64_URL_NOPAD
    pub fn get_random_bytes_with_encoding(&self, len: usize, encoding: EncodedingFormat) -> String {
        let mut random_bytes_array = vec![0u8; len];
        rand::thread_rng().fill_bytes(&mut random_bytes_array);

        match encoding {
            EncodedingFormat::Base64 => BASE64.encode(&random_bytes_array),
            EncodedingFormat::Base64Nopad => BASE64_NOPAD.encode(&random_bytes_array),
            EncodedingFormat::Base64Url => BASE64URL.encode(&random_bytes_array),
            EncodedingFormat::Base64UrlNopad => BASE64URL_NOPAD.encode(&random_bytes_array),
            _ => BASE64.encode(&random_bytes_array),
        }
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
            char_pool.push_str(UPPERCASE);
            password.push(self.get_random_upper())
        }
        if has_lower {
            char_pool.push_str(LOWERCASE);
            password.push(self.get_random_lower())
        }
        if has_number {
            char_pool.push_str(NUMBER);
            password.push(self.get_random_number())
        }
        if has_symbol {
            char_pool.push_str(SYMBOLS);
            password.push(self.get_random_symbol())
        }

        if char_pool.is_empty() {
            return Err(JsValue::from_str(
                "At least one character type must be included.",
            ));
        }

        for _ in 0..(length - password.len()) {
            let result = char_pool.as_bytes().choose(&mut self.rng);
            if let Some(result) = result {
                password.push(*result as char);
            }
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
    let bytesp2 = password.get_random_bytes_with_encoding(16, EncodedingFormat::Base64UrlNopad);
    println!(
        "upper: {} lower: {} number: {} symbol: {} bytes: {} bytes_url_nopad: {}",
        upper, lower, number, symbol, bytesp, bytesp2,
    );
}

#[test]
fn password_random_suit_test() {
    let mut password = Password::new();
    let rpass = password.get_random_password(true, true, true, true, 16);
    match rpass {
        Ok(p) => println!("password: {}", p),
        Err(e) => eprintln!("{}", e.is_string()),
    }
}
