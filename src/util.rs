use std::fs;
use json::JsonValue;
use crate::core::WordleWord;

pub fn load_wordlist(path: &str) -> Vec<WordleWord> {
    let content = fs::read_to_string(path)
        .expect("failed to load wordlist");
    load_wordlist_from_json(&content)
}

pub fn load_wordlist_from_json(json: &str) -> Vec<WordleWord> {
    let parsed = json::parse(json).expect("invalid wordlist");
    match parsed {
        JsonValue::Array(array) => {
            array.into_iter().map(|x| x.as_str().unwrap().parse().unwrap()).collect()
        }
        _ => panic!("not a json array")
    }
}
