use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::path::Path;
use tera::{Context, Tera};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let tera = Tera::new("templates/**/*.rs")?;

    let lang_data: serde_json::Value = serde_yaml::from_reader(File::open("languages.yaml")?)?;
    let context = Context::from_value(serde_json::json!({
        "languages": lang_data,
    }))?;

    let lang_out = Path::new(&out_dir).join("language.rs");
    eprintln!("creating {:?}", lang_out);
    let rendered = tera.render("language.rs", &context)?;
    fs::write(&lang_out, rendered)?;

    Ok(())
}
