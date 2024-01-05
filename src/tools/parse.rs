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

#[test]
fn b64_encode_test() {
    let result = Base64::encode("你好, world");
    println!("base64 encode: {:?}", result);
    assert_eq!("5L2g5aW9LCB3b3JsZA==", result)
}

#[test]
fn b64_decode_test() {
    let result: String = Base64::decode("5L2g5aW9LCB3b3JsZA==");
    println!("base64 decode: {:?}", result);
    assert_eq!("你好, world", result)
}
