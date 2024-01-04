use wasm_bindgen::prelude::*;
use zxcvbn::zxcvbn;

#[wasm_bindgen]
pub struct Password {}

#[wasm_bindgen]
impl Password {
    pub fn strength(password: &str) -> String {
        let estimate = zxcvbn(password, &[]);
        match estimate {
            Ok(result) => result.score().to_string(),
            Err(_) => String::new(),
        }
    }
}
