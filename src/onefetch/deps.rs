use {
    crate::onefetch::error::*,
    std::{ffi::OsStr, fs},
    std::collections::HashMap,
};

mod Parsers {
    fn npm(contents: &str) -> i32 {
        println!(contents)
        return 0
    }
}


type DependencyParser = fn(&str) -> i32;
let package_managers: HashMap<&str, (&str, DependencyParser)> = [
    ("npm", ("package.json", Parsers.npm)),
].iter().cloned().collect();

pub struct Detector {}

impl Detector {
    pub fn get_dep_count(&self, dir: &str) -> Result<String> {
        fn is_package_file<S: AsRef<str>>(file_name: S) -> bool {
            package_managers
                .iter()
                .any(|&info| file_name.as_ref().starts_with(info[0]))
        }

        let mut package_files = fs::read_dir(dir)
            .chain_err(|| "Could not read directory")?
            .filter_map(std::result::Result::ok)
            .map(|entry| entry.path())
            .filter(|entry| {
                entry.is_file()
                    && entry
                        .file_name()
                        .map(OsStr::to_string_lossy)
                        .map(is_package_file)
                        .unwrap_or_default()
            })
            .filter_map(|entry| {
                let contents = fs::read_to_string(entry).unwrap_or_default();
                self.analyze(&contents)
            })
            .collect::<Vec<_>>();
    }
}
