use crate::onefetch::error::*;
use {regex::Regex, toml::Value};

pub fn cargo(contents: &str) -> Result<usize> {
    let parsed = contents.parse::<Value>()?;

    Ok(parsed["dependencies"].as_table().unwrap().len())
}

pub fn go_modules(contents: &str) -> Result<usize> {
    let count = Regex::new(r"v[0-9]+")?.find_iter(contents).count();

    Ok(count)
}

pub fn npm(contents: &str) -> Result<usize> {
    let parsed = json::parse(contents)?;

    Ok(parsed["dependencies"].len())
}

pub fn pip(contents: &str) -> Result<usize> {
    let count = Regex::new(r"(^|\n)[A-z]+")?.find_iter(contents).count();

    Ok(count)
}
