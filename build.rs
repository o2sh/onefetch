use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::path::Path;
use tera::{Context, Tera};

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/onefetch.ico");
        res.compile()?;
    }
    let out_dir = env::var("OUT_DIR").expect("No OUT_DIR variable.");
    let mut tera = Tera::default();
    tera.register_filter("strip_color_tokens", strip_color_tokens_filter);
    tera.register_filter("hex_to_rgb", hex_to_rgb_filter);

    let lang_data: serde_json::Value = serde_yaml::from_reader(File::open("languages.yaml")?)?;

    let output_path = Path::new(&out_dir).join("language.rs");

    let rust_code = tera.render_str(
        &std::fs::read_to_string("src/info/langs/language.tera")?,
        &Context::from_value(serde_json::json!({ "languages": lang_data, }))?,
    )?;
    fs::write(output_path, rust_code)?;

    Ok(())
}

/// Strips out `{n}` from the given string.
fn strip_color_tokens_filter(
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

fn hex_to_rgb_filter(
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
