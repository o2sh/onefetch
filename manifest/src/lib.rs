use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{ffi::OsStr, fs};
use strum::{Display, EnumIter};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref HASHMAP: HashMap<&'static str, ManifestType> = {
        let mut m = HashMap::new();
        m.insert("Cargo.toml", ManifestType::Cargo);
        m.insert("package.json", ManifestType::Npm);
        m
    };
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Manifest {
    pub manifest_type: ManifestType,
    pub number_of_dependencies: usize,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Display, Clone, Copy, PartialEq, Eq, Debug, EnumIter)]
pub enum ManifestType {
    Npm,
    Cargo,
}

pub fn get_manifests<P: AsRef<Path>>(path: P) -> Result<Vec<Manifest>> {
    let manifest_paths = get_manifest_paths(path)?;
    let manifests = manifest_paths
        .iter()
        .map(|manifest_path| {
            let filename = manifest_path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default();

            let manifest_type = HASHMAP[filename];

            match manifest_type {
                ManifestType::Cargo => parse_cargo_manifest(manifest_path),
                ManifestType::Npm => parse_npm_manifest(manifest_path),
            }
        })
        .filter_map(std::result::Result::ok)
        .collect::<Vec<_>>();
    Ok(manifests)
}

fn get_manifest_paths<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>> {
    let manifest_paths = fs::read_dir(path)?
        .filter_map(std::result::Result::ok)
        .map(|entry| entry.path())
        .filter(|entry| {
            entry.is_file()
                && entry
                    .file_name()
                    .map(OsStr::to_string_lossy)
                    .map(|s| HASHMAP.contains_key(s.as_ref()))
                    .unwrap_or_default()
        })
        .collect::<Vec<_>>();

    Ok(manifest_paths)
}

fn parse_cargo_manifest(path: &PathBuf) -> Result<Manifest> {
    let m = cargo_toml::Manifest::from_path(path.as_path())?;
    let package = m.package();
    let description = match package.description() {
        Some(v) => Some(String::from(v)),
        None => None,
    };

    Ok(Manifest {
        manifest_type: ManifestType::Cargo,
        number_of_dependencies: m.dependencies.len(),
        name: package.name.clone(),
        description,
    })
}

fn parse_npm_manifest(path: &PathBuf) -> Result<Manifest> {
    let package = npm_package_json::Package::from_path(path)?;
    Ok(Manifest {
        manifest_type: ManifestType::Npm,
        number_of_dependencies: package.dependencies.len(),
        name: package.name,
        description: package.description,
    })
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_get_manifest() {
        let toto = get_manifest(Path::from_str("."));
    }
}
