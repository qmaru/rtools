use wasm_bindgen::prelude::*;
use zxcvbn::zxcvbn;

#[wasm_bindgen]
/// `Password` password tools
pub struct Password {}

#[wasm_bindgen]
impl Password {
    /// `strength` test the strength of a password
    pub fn strength(password: &str) -> String {
        let estimate = zxcvbn(password, &[]);
        match estimate {
            Ok(result) => result.score().to_string(),
            Err(_) => String::new(),
        }
    }
}

#[test]
fn strength_test() {
    let result = Password::strength("123456");
    println!("strength: {:?}", result);
    assert_eq!("0", result)
}
