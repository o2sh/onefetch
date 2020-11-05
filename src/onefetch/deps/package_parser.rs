use crate::onefetch::error::*;
use {regex::Regex, toml::Value};

pub fn cargo(contents: &str) -> Result<i32> {
    let parsed = contents.parse::<Value>()?;

    Ok(parsed["dependencies"].as_table().unwrap().len() as i32)
}

pub fn go_modules(contents: &str) -> Result<i32> {
    let count = Regex::new(r"v[0-9]+")?.find_iter(contents).count();

    Ok(count as i32)
}

pub fn npm(contents: &str) -> Result<i32> {
    let parsed = json::parse(contents)?;

    Ok(parsed["dependencies"].len() as i32)
}

pub fn pip(contents: &str) -> Result<i32> {
    let count = Regex::new(r"(^[A-z]+)|(\n[A-z]+)")?.find_iter(contents).count();

    Ok(count as i32)
}
