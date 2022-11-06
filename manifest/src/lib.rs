use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use strum::{Display, EnumIter};

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
    let manifest_paths = fs::read_dir(path)?
        .filter_map(std::result::Result::ok)
        .map(|entry| entry.path())
        .filter(|entry| entry.is_file())
        .filter_map(|file_path| {
            let filename = file_path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default();
            match file_name_to_manifest_type(filename) {
                Some(manifest_type) => Some((file_path, manifest_type)),
                None => None,
            }
        })
        .map(|(file_path, manifest_type)| match manifest_type {
            ManifestType::Cargo => parse_cargo_manifest(&file_path),
            ManifestType::Npm => parse_npm_manifest(&file_path),
        })
        .filter_map(std::result::Result::ok)
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

fn file_name_to_manifest_type(filename: &str) -> Option<ManifestType> {
    let manifest_type = match filename {
        "Cargo.toml" => Some(ManifestType::Cargo),
        "package.json" => Some(ManifestType::Npm),
        _ => None,
    };
    manifest_type
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_get_manifest() {
        let toto = get_manifest(Path::from_str("."));
    }
}
