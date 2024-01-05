use pinyin::ToPinyin;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Pinyin {}

#[wasm_bindgen]
impl Pinyin {
    pub fn han_to_pinyin(hans: &str) -> String {
        let mut results = String::new();
        for pinyin in hans.to_pinyin() {
            if let Some(pinyin) = pinyin {
                let result = pinyin.plain();
                results = format!("{}{}", results, result);
            }
        }
        results
    }
}

#[test]
fn han_to_pinyin_test() {
    let result = Pinyin::han_to_pinyin("你好");
    println!("Pinyin: {:?}", result);
    assert_eq!("nihao", result)
}
