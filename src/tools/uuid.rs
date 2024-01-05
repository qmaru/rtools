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

#[test]
fn v4_test() {
    let result = UUID::v4();
    println!("UUID: {:?}", result);
    assert_ne!("", result)
}
