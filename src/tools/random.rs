use nanoid::nanoid;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// `UUID` uuid tools
pub struct UUID {}

#[wasm_bindgen]
impl UUID {
    /// `v4` creates v4 UUIDs
    pub fn v4() -> String {
        let id = Uuid::new_v4();
        id.to_string()
    }
}

#[wasm_bindgen]
/// `NanoID` nanoid tools
pub struct NanoID {}

#[wasm_bindgen]
impl NanoID {
    /// `nanoid` a tiny, secure, URL-friendly, unique string ID
    pub fn nanoid(len: usize) -> String {
        let id = nanoid!(len);
        id
    }
}

#[test]
fn uuid_v4_test() {
    let result = UUID::v4();
    println!("UUID: {:?}", result);
    assert_ne!("", result)
}

#[test]
fn nanoid_test() {
    let result = NanoID::nanoid(21);
    println!("nanoid: {:?}", result);
    assert_ne!("", result)
}
