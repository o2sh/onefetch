use onefetch::info::langs::language::Language;
use std::collections::HashMap;
use strum::IntoEnumIterator;

fn main() {
    let json: HashMap<_, _> = Language::iter()
        .map(|l| (l.to_string(), l.jsonish()))
        .collect();
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}
