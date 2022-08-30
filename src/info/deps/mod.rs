use super::info_field::{InfoField, InfoFieldValue, InfoType};
use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;
use std::{ffi::OsStr, fs};

pub mod package_manager;

type DependencyParser = fn(&str) -> Result<usize>;

pub struct DependencyDetector {
    package_managers: HashMap<String, (DependencyParser, package_manager::PackageManager)>,
}

impl DependencyDetector {
    pub fn new() -> Self {
        let package_managers = package_manager::build();

        DependencyDetector { package_managers }
    }

    pub fn get_dependencies(&self, dir: &Path) -> Result<String> {
        let deps = fs::read_dir(dir)?
            .filter_map(std::result::Result::ok)
            .map(|entry| entry.path())
            .filter(|entry| {
                entry.is_file()
                    && entry
                        .file_name()
                        .map(OsStr::to_string_lossy)
                        .map(|s| self.package_managers.contains_key(s.as_ref()))
                        .unwrap_or_default()
            })
            .map(|entry| {
                let (parser, found_package_manager) =
                    &self.package_managers[entry.file_name().unwrap().to_str().unwrap()];
                let contents = fs::read_to_string(entry).unwrap_or_default();
                let number_of_deps = parser(&contents).unwrap();
                match number_of_deps {
                    0 => Err(""),
                    _ => Ok(format!("{} ({})", number_of_deps, found_package_manager)),
                }
            })
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        let output = deps.join(", ");

        Ok(output)
    }
}

pub struct DependenciesInfo {
    pub dependencies: String,
}

impl DependenciesInfo {
    pub fn new(repo_path: &Path) -> Result<Self> {
        let dependencies = DependencyDetector::new().get_dependencies(repo_path)?;
        Ok(Self { dependencies })
    }
}

impl InfoField for DependenciesInfo {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: InfoType::Dependencies,
            value: self.dependencies.to_string(),
        }
    }
    fn title(&self) -> String {
        String::from("Dependencies")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_get_dependencies() -> Result<()> {
        let dependency_detector = DependencyDetector::new();
        let deps = dependency_detector.get_dependencies(Path::new("."))?;
        let re = Regex::new(r"^\d+ \(cargo\)$")?;
        assert!(re.is_match(&deps));

        Ok(())
    }
}
