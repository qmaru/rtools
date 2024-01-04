use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct UUID {}

#[wasm_bindgen]
impl UUID {
    pub fn v4() -> String {
        let id = Uuid::new_v4();
        id.to_string()
    }
}
