use regex::Regex;
use std::{collections::HashMap, fs, hash::BuildHasherDefault};
use twox_hash::XxHash64;

pub fn load_lang(lang: &str) -> HashMap<String, String, BuildHasherDefault<XxHash64>> {
    let mut map = HashMap::default();
    let file = fs::read_to_string(format!("lang/{}.json", lang)).unwrap();
    let re = Regex::new(r####""*([\w_]+)"* *: *"*([\w ]+)"*"####).unwrap();
    for cap in re.captures_iter(&file) {
        let key = cap[1].to_string();
        let value = cap[2].to_string();
        map.insert(key, value);
    }
    map
}
