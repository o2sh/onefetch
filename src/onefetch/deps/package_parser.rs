use crate::onefetch::error::*;
use {regex::Regex, toml::Value};

pub fn cargo(contents: &str) -> Result<usize> {
    let parsed = contents.parse::<Value>()?;
    let count = parsed.get("dependencies");

    match count {
        Some(val) => Ok(val.as_table().unwrap().len()),
        None => Ok(0),
    }
}

pub fn go_modules(contents: &str) -> Result<usize> {
    let mut count = 0;
    let mut start = false;
    for line in contents.lines() {
        if line.contains("require") {
            start = true;
            continue;
        }

        if start && line.contains(')') {
            break;
        }

        if start {
            count += 1;
        }
    }

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
