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
    tera.register_filter("hextorgb", hex_to_rgb);
    tera.register_filter("cleanansi", clean_color);

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

/// Converts a hex string to an object with keys `r`, `g`, `b`.
fn hex_to_rgb(
    value: &tera::Value,
    _args: &HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let hex_string = match value {
        tera::Value::String(s) => s,
        _ => return Err(tera::Error::msg("expected string")),
    };
    let hex_string = match hex_string.strip_prefix('#') {
        Some(s) => s,
        None => return Err(tera::Error::msg("expected hex string starting with `#`")),
    };
    if hex_string.len() != 6 {
        return Err(tera::Error::msg("expected a 6 digit hex string"));
    }
    let channel_bytes = match u32::from_str_radix(hex_string, 16) {
        Ok(n) => n,
        Err(_) => return Err(tera::Error::msg("expected a valid hex string")),
    };
    let r = (channel_bytes >> 16) & 0xFF;
    let g = (channel_bytes >> 8) & 0xFF;
    let b = channel_bytes & 0xFF;

    Ok(serde_json::json!({
        "r": r,
        "g": g,
        "b": b,
    }))
}

/// Converts ansi colors to their their owo counterparts. Also remaps `white`
/// to `Default`.
fn clean_color(
    value: &tera::Value,
    _args: &HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let ansi = match value {
        tera::Value::String(s) => s,
        _ => return Err(tera::Error::msg("expected string")),
    };
    let owo_var = match ansi.as_str() {
        "black" => "Black",
        "red" => "Red",
        "green" => "Green",
        "yellow" => "Yellow",
        "blue" => "Blue",
        "magenta" => "Magenta",
        "cyan" => "Cyan",
        "white" => "Default",
        _ => return Err(tera::Error::msg("unexpected ansi color")),
    };
    Ok(tera::Value::String(owo_var.to_string()))
}
