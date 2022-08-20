use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use strum::EnumIter;
use toml::Value;
use yaml_rust::YamlLoader;

macro_rules! define_package_managers {
    ($( { $name:ident, $display:literal, [$(($file:literal, $parser:expr))+] } ),+ ,) => {

        #[derive(PartialEq, Eq, EnumIter)]
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
    let parsed: serde_json::Value = serde_json::from_str(contents)?;
    match &parsed["dependencies"].as_object() {
        Some(val) => Ok(val.len()),
        None => Ok(0),
    }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cargo() -> Result<()> {
        let data = r#"
[dependencies]
bytecount = "0.6.3"
clap = {version = "3.2.16", features = ["derive"]}
clap_complete = "3.2.3"
color_quant = "1.1.0"
git-repository = {version = "0.20.0", features = ["max-performance", "unstable", "serde1"]}
# Comment
git2 = {version = "0.15.0", default-features = false}"#;

        let number_of_deps = cargo(data)?;
        assert_eq!(number_of_deps, 6);
        Ok(())
    }

    #[test]
    fn test_go_modules() -> Result<()> {
        let data = r#"
require (
    golang.org/x/crypto v0.0.0-20210921155107-089bfa567519
    golang.org/x/tools v0.0.0-20191119224855-298f0cb1881e
)
"#;

        let number_of_deps = go_modules(data)?;
        assert_eq!(number_of_deps, 2);
        Ok(())
    }

    #[test]
    fn test_npm() -> Result<()> {
        let data = r#"
{
    "dependencies": {
        "@rollup/plugin-yaml": "^3.1.0",
        "@sveltejs/vite-plugin-svelte": "^1.0.1",
        "@tsconfig/svelte": "^3.0.0"
    }
}"#;

        let number_of_deps = npm(data)?;
        assert_eq!(number_of_deps, 3);
        Ok(())
    }

    #[test]
    fn test_pip_requirement() -> Result<()> {
        let data = r#"
cycler==0.10.0
matplotlib==3.2.1
numpy==1.18.5
pandas==1.0.4
pyparsing==2.4.7
python-dateutil==2.8.1
pytz==2020.1
scipy==1.4.1"#;
        let number_of_deps = pip_requirement(data)?;
        assert_eq!(number_of_deps, 8);
        Ok(())
    }

    #[test]
    fn test_pip_pyproject() -> Result<()> {
        let data = r#"
[tool.poetry.dependencies]
python = ">=3.7,<3.10"
boto3 = "^1.12"
requests = "^2.23"
attrs = ">=19.3,<22.2"
jsonpickle = ">=1.3,<2.3"
redis = ">=3.4,<5.0"
numpy = ">=1.19.2,<1.20.0"
scipy = ">=1.4.1,<1.8.0"
absl-py = ">=0.9,<1.3""#;

        let number_of_deps = pip_pyproject(data)?;
        assert_eq!(number_of_deps, 9);
        Ok(())
    }

    #[test]
    fn test_pip_pipfile() -> Result<()> {
        let data = r#"
[packages]
requests = { extras = ['socks'] }
records = '>0.5.0'
django = { git = 'https://github.com/django/django.git', ref = '1.11.4', editable = true }
"e682b37" = {file = "https://github.com/divio/django-cms/archive/release/3.4.x.zip"}
"e1839a8" = {path = ".", editable = true}
pywinusb = { version = "*", os_name = "=='nt'", index="pypi"}"#;

        let number_of_deps = pip_pipfile(data)?;
        assert_eq!(number_of_deps, 6);
        Ok(())
    }

    #[test]
    fn test_pub_packages() -> Result<()> {
        let data = r#"
dependencies:
  # To update these, use "flutter update-packages --force-upgrade".
  archive: 3.3.1
  args: 2.3.1
  browser_launcher: 1.1.1
  dds: 2.2.6
  dwds: 15.0.0
  completion: 1.0.0
  coverage: 1.5.0
  crypto: 3.0.2"#;

        let number_of_deps = pub_packages(data)?;
        assert_eq!(number_of_deps, 8);
        Ok(())
    }
}
