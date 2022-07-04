use onefetch::info::langs::language::Language;
use strum::IntoEnumIterator;

fn main() {
    let json: Vec<_> = Language::iter().map(|lang| lang.jsonish()).collect();
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}
