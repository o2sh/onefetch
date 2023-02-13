use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use strum::{Display, EnumIter};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Manifest {
    pub manifest_type: ManifestType,
    pub number_of_dependencies: usize,
    pub name: String,
    pub description: Option<String>,
    pub version: String,
    pub license: Option<String>,
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
            let file_name = file_path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default();
            file_name_to_manifest_type(file_name).map(|manifest_type| (file_path, manifest_type))
        })
        .map(|(file_path, manifest_type)| match manifest_type {
            ManifestType::Cargo => parse_cargo_manifest(&file_path),
            ManifestType::Npm => parse_npm_manifest(&file_path),
        })
        .filter_map(std::result::Result::ok)
        .collect::<Vec<_>>();

    Ok(manifest_paths)
}

fn parse_cargo_manifest(path: &Path) -> Result<Manifest> {
    let m = cargo_toml::Manifest::from_path(path)?;
    let package = m.package.context("Not a package (only a workspace)")?;
    let description = package.description().map(|v| v.into());

    Ok(Manifest {
        manifest_type: ManifestType::Cargo,
        number_of_dependencies: m.dependencies.len(),
        name: package.name.clone(),
        description,
        version: package.version().into(),
        license: package.license().map(|x| x.into()),
    })
}

fn parse_npm_manifest(path: &Path) -> Result<Manifest> {
    let package = npm_package_json::Package::from_path(path)?;
    Ok(Manifest {
        manifest_type: ManifestType::Npm,
        number_of_dependencies: package.dependencies.len(),
        name: package.name,
        description: package.description,
        version: package.version,
        license: package.license,
    })
}

fn file_name_to_manifest_type(filename: &str) -> Option<ManifestType> {
    match filename {
        "Cargo.toml" => Some(ManifestType::Cargo),
        "package.json" => Some(ManifestType::Npm),
        _ => None,
    }
}
