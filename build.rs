use regex::regex;
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::path::Path;
use tera::{Context, Kwargs, State, Tera};

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
        &Context::from_serialize(&serde_json::json!({ "languages": lang_data, }))?,
        false,
    )?;
    fs::write(output_path, rust_code)?;

    Ok(())
}

/// Strips out `{n}` from the given string.
fn strip_color_tokens_filter(value: &str, _args: Kwargs, _state: &State) -> String {
    regex!(r"\{\d+\}").replace_all(value, "").to_string()
}

fn hex_to_rgb_filter(
    hex_string: &str,
    _args: Kwargs,
    _state: &State,
) -> tera::TeraResult<tera::Value> {
    let Some(hex_string) = hex_string.strip_prefix('#') else {
        return Err(tera::Error::message(
            "expected hex string starting with `#`",
        ));
    };
    if hex_string.len() != 6 {
        return Err(tera::Error::message("expected a 6 digit hex string"));
    }
    let Ok(channel_bytes) = u32::from_str_radix(hex_string, 16) else {
        return Err(tera::Error::message("expected a valid hex string"));
    };
    let r = (channel_bytes >> 16) & 0xFF;
    let g = (channel_bytes >> 8) & 0xFF;
    let b = channel_bytes & 0xFF;

    tera::Value::try_from_serializable(&serde_json::json!({
        "r": r,
        "g": g,
        "b": b,
    }))
}
