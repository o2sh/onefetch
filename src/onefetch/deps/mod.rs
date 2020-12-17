use {
    crate::onefetch::error::*,
    std::collections::HashMap,
    std::{ffi::OsStr, fs},
};

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

    pub fn get_dependencies(&self, dir: &str) -> Result<String> {
        let deps = fs::read_dir(dir)
            .chain_err(|| "Could not read directory")?
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
                let contents = fs::read_to_string(entry)?;
                let number_of_deps = parser(&contents)?;
                match number_of_deps {
                    0 => Err("".into()),
                    _ => Ok(format!("{} ({})", number_of_deps, found_package_manager)),
                }
            })
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        let output = deps.join(", ");

        Ok(output)
    }
}
