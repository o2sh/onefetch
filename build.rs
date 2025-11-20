use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::{self, create_dir_all, read_to_string, File};
use std::io::BufWriter;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
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

    println!("cargo:rerun-if-changed=locales/");
    let locales_dir = PathBuf::from("locales").read_dir()?;
    let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("__locales_compiled");

    for dir in locales_dir {
        let dir = dir?.path();
        let lang = dir.components().last().unwrap();

        let out_dir = out_dir.join(lang);
        create_dir_all(&out_dir).unwrap();

        let mut out_file = BufWriter::new(File::create(out_dir.join("onefetch.ftl"))?);

        for ftl in dir.read_dir()? {
            let ftl = ftl?.path();
            let contents = read_to_string(&ftl)?;
            writeln!(out_file, "{contents}")?;
        }
    }

    Ok(())
}

/// Strips out `{n}` from the given string.
fn strip_color_tokens_filter(
    value: &tera::Value,
    _args: &HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    static COLOR_INDEX_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\{\d+\}").unwrap());
    let tera::Value::String(s) = value else {
        return Err(tera::Error::msg("expected string"));
    };
    Ok(tera::Value::String(
        COLOR_INDEX_REGEX.replace_all(s, "").to_string(),
    ))
}

fn hex_to_rgb_filter(
    value: &tera::Value,
    _args: &HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let tera::Value::String(hex_string) = value else {
        return Err(tera::Error::msg("expected string"));
    };
    let Some(hex_string) = hex_string.strip_prefix('#') else {
        return Err(tera::Error::msg("expected hex string starting with `#`"));
    };
    if hex_string.len() != 6 {
        return Err(tera::Error::msg("expected a 6 digit hex string"));
    }
    let Ok(channel_bytes) = u32::from_str_radix(hex_string, 16) else {
        return Err(tera::Error::msg("expected a valid hex string"));
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
