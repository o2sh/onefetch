use {
    crate::onefetch::error::*,
    std::collections::HashMap,
    std::{ffi::OsStr, fs},
};

pub enum PackageManager {
    Npm,
}

impl std::fmt::Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            PackageManager::Npm => write!(f, "Npm"),
        }
    }
}

type DependencyParser = fn(&str) -> Option<i32>;

struct Detector {
    package_managers: HashMap<String, (DependencyParser, PackageManager)>,
}

fn npm(contents: &str) -> Option<i32> {
    Some(0)
}

impl Detector {
    pub fn new(&self) -> Detector {
        let mut package_managers: HashMap<String, (DependencyParser, PackageManager)> =
            HashMap::new();
        package_managers.insert(String::from("package.json"), (npm, PackageManager::Npm));

        Detector { package_managers }
    }

    pub fn get_dep_count(&self, dir: &str) -> Result<String> {
        fn is_package_file(detector: &Detector, file_name: &str) -> bool {
            detector
                .package_managers
                .iter()
                .any(|(package_manager_file_name, _)| {
                    file_name.starts_with(package_manager_file_name)
                })
        }

        let package_files = fs::read_dir(dir)
            .chain_err(|| "Could not read directory")?
            .filter_map(std::result::Result::ok)
            .map(|entry| entry.path())
            .filter(|entry| {
                entry.is_file()
                    && entry
                        .file_name()
                        .map(OsStr::to_string_lossy)
                        .map(|s| is_package_file(&self, s.as_ref()))
                        .unwrap_or_default()
            })
            .map(|entry| {
                let (parser, package_manager) =
                    &self.package_managers[entry.file_name().unwrap().to_str().unwrap()];
                let contents = fs::read_to_string(entry).unwrap_or_default();
                format!("{} ({})", parser(&contents).unwrap(), package_manager)
            })
            .collect::<Vec<_>>();

        let output = package_files.join(", ");

        Ok(output)
    }
}
