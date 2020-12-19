use crate::onefetch::error::*;
use std::collections::HashMap;
use {regex::Regex, strum::EnumIter, toml::Value, yaml_rust::YamlLoader};

macro_rules! define_package_managers {
    ($( { $name:ident, $display:literal, [$(($file:literal, $parser:expr))+] } ),+ ,) => {

        #[derive(PartialEq, EnumIter)]
        pub enum PackageManager {
            $(
                $name ,
            )+
        }

        impl std::fmt::Display for PackageManager {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match *self {
                    $( PackageManager::$name => write!(f, $display), )+
                }
            }
        }

        pub fn build() -> HashMap<String, (fn(&str) -> Result<usize>, PackageManager)> {
            let mut package_managers: HashMap<
                String,
                (fn(&str) -> Result<usize>, PackageManager)
            > = HashMap::new();

            $(
                $(
                     package_managers.insert(String::from($file), ($parser, PackageManager::$name));
                )+
            )+

            package_managers
        }

    }
}

define_package_managers! {
    { Cargo, "cargo", [ ("Cargo.toml", cargo) ] },
    { GoModules, "go modules", [ ("go.mod", go_modules) ] },
    { Npm, "npm", [ ("package.json", npm) ] },
    { Pip, "pip", [ ("requirements.txt", pip_requirement) ("pyproject.toml", pip_pyproject) ("Pipfile", pip_pipfile) ] },
    { Pub, "pub", [ ("pubspec.yaml", pub_packages) ] },
}

fn cargo(contents: &str) -> Result<usize> {
    let parsed = contents.parse::<Value>()?;
    let count = parsed.get("dependencies");

    match count {
        Some(val) => Ok(val.as_table().unwrap().len()),
        None => Ok(0),
    }
}

fn go_modules(contents: &str) -> Result<usize> {
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

fn npm(contents: &str) -> Result<usize> {
    let parsed = json::parse(contents)?;

    Ok(parsed["dependencies"].len())
}

fn pip_requirement(contents: &str) -> Result<usize> {
    let count = Regex::new(r"(^|\n)[A-z]+")?.find_iter(contents).count();

    Ok(count)
}

fn pip_pyproject(contents: &str) -> Result<usize> {
    let parsed = contents.parse::<Value>()?;
    let count = parsed
        .get("tool")
        .and_then(|tool| tool.get("poetry"))
        .and_then(|poetry| poetry.get("dependencies"));
    match count {
        Some(val) => Ok(val.as_table().unwrap().len()),
        None => Ok(0),
    }
}

fn pip_pipfile(contents: &str) -> Result<usize> {
    let parsed = contents.parse::<Value>()?;
    let count = parsed.get("packages");

    match count {
        Some(val) => Ok(val.as_table().unwrap().len()),
        None => Ok(0),
    }
}

fn pub_packages(contents: &str) -> Result<usize> {
    match YamlLoader::load_from_str(contents) {
        Ok(parsed) => match &parsed[0]["dependencies"].as_hash() {
            Some(deps) => Ok(deps.len()),
            None => Ok(0),
        },
        Err(_) => Ok(0),
    }
}
