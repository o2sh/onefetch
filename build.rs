use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::path::Path;
use tera::{Context, Tera};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut tera = Tera::new("src/**/*.tera.rs")?;
    tera.register_filter("strip_color_indices", strip_color_indices);

    let lang_data: serde_json::Value = serde_yaml::from_reader(File::open("languages.yaml")?)?;
    let context = Context::from_value(serde_json::json!({
        "languages": lang_data,
    }))?;

    let lang_out = Path::new(&out_dir).join("language.rs");
    eprintln!("creating {:?}", lang_out);
    let rendered = tera.render("info/langs/language.tera.rs", &context)?;
    fs::write(&lang_out, rendered)?;

    Ok(())
}

/// Strips out `{n}` from the given string.
fn strip_color_indices(
    value: &tera::Value,
    _args: &HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    lazy_static! {
        static ref COLOR_INDEX_REGEX: Regex = Regex::new(r"\{\d+\}").unwrap();
    }
    let s = match value {
        tera::Value::String(s) => s,
        _ => return Err(tera::Error::msg("expected string")),
    };
    return Ok(tera::Value::String(
        COLOR_INDEX_REGEX.replace_all(s, "").to_string(),
    ));
}
