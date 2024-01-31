use pinyin::ToPinyin;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// `Pinyin` pinyin tools
pub struct Pinyin {}

#[wasm_bindgen]
impl Pinyin {
    /// `han_to_pinyin` generate 汉字 to [han,zi]
    pub fn han_to_pinyin(hans: &str) -> Vec<String> {
        let mut results: Vec<String> = Vec::new();
        for pinyin in hans.to_pinyin() {
            if let Some(pinyin) = pinyin {
                let result = pinyin.plain();
                results.push(result.to_string());
            }
        }
        results
    }

    /// `han_to_pinyin_with_tone` generate 汉字 to [hàn,zì]
    pub fn han_to_pinyin_with_tone(hans: &str) -> Vec<String> {
        let mut results: Vec<String> = Vec::new();
        for pinyin in hans.to_pinyin() {
            if let Some(pinyin) = pinyin {
                let result = pinyin.with_tone();
                results.push(result.to_string());
            }
        }
        results
    }
}

#[test]
fn han_to_pinyin_test() {
    let result = Pinyin::han_to_pinyin("你好");
    println!("Pinyin: {:?}", result);
    assert_eq!("nihao", result.join(""))
}

#[test]
fn han_to_pinyin_with_tone_test() {
    let result = Pinyin::han_to_pinyin_with_tone("你好");
    println!("Pinyin: {:?}", result);
    assert_eq!("nǐhǎo", result.join(""))
}
