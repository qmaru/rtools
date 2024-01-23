use pinyin::ToPinyin;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// `Pinyin` pinyin tools
pub struct Pinyin {}

#[wasm_bindgen]
impl Pinyin {
    /// `han_to_pinyin` generate 汉字 to hanzi
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

    /// `han_to_pinyin_with_tone` generate 汉字 to hànzì
    pub fn han_to_pinyin_with_tone(hans: &str) -> String {
        let mut results = String::new();
        for pinyin in hans.to_pinyin() {
            if let Some(pinyin) = pinyin {
                let result = pinyin.with_tone();
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

#[test]
fn han_to_pinyin_with_tone_test() {
    let result = Pinyin::han_to_pinyin_with_tone("你好");
    println!("Pinyin: {:?}", result);
    assert_eq!("nǐhǎo", result)
}
