use base64_light::{base64_decode_str, base64_encode};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Base64 {}

#[wasm_bindgen]
impl Base64 {
    pub fn encode(content: &str) -> String {
        base64_encode(content)
    }

    pub fn decode(content: &str) -> String {
        base64_decode_str(content)
    }
}
