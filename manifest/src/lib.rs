use anyhow::{Context, Result};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
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
    let manifests = fs::read_dir(path)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|p| p.is_file())
        .filter_map(|file_path: PathBuf| {
            let file_name = file_path.file_name()?.to_str()?;
            let manifest_type = file_name_to_manifest_type(file_name)?;
            Some((file_path, manifest_type))
        })
        .filter_map(|(file_path, manifest_type)| match manifest_type {
            ManifestType::Cargo => parse_cargo_manifest(&file_path).ok(),
            ManifestType::Npm => parse_npm_manifest(&file_path).ok(),
        })
        .collect::<Vec<_>>();

    Ok(manifests)
}

fn parse_cargo_manifest(path: &Path) -> Result<Manifest> {
    let m = cargo_toml::Manifest::from_path(path)
        .with_context(|| format!("Failed to parse Cargo.toml at '{}'", path.display()))?;
    let package = m.package.context("Not a package (only a workspace)")?;
    let description = package.description().map(Into::into);

    Ok(Manifest {
        manifest_type: ManifestType::Cargo,
        number_of_dependencies: m.dependencies.len(),
        name: package.name.clone(),
        description,
        version: package.version().into(),
        license: package.license().map(Into::into),
    })
}

#[derive(Deserialize)]
struct PackageJson {
    name: Option<String>,
    description: Option<String>,
    version: Option<String>,
    license: Option<String>,
    #[serde(default)]
    dependencies: HashMap<String, serde_json::Value>,
}

fn parse_npm_manifest(path: &Path) -> Result<Manifest> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read package.json at '{}'", path.display()))?;

    let pkg: PackageJson = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse package.json at '{}'", path.display()))?;

    Ok(Manifest {
        manifest_type: ManifestType::Npm,
        number_of_dependencies: pkg.dependencies.len(),
        name: pkg.name.unwrap_or_default(),
        description: pkg.description,
        version: pkg.version.unwrap_or_default(),
        license: pkg.license,
    })
}

fn file_name_to_manifest_type(filename: &str) -> Option<ManifestType> {
    match filename {
        "Cargo.toml" => Some(ManifestType::Cargo),
        "package.json" => Some(ManifestType::Npm),
        _ => None,
    }
}
