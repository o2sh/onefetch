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
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub license: Option<String>,
}

#[derive(Display, Clone, Copy, PartialEq, Eq, Debug, EnumIter)]
pub enum ManifestType {
    Npm,
    Cargo,
    #[strum(to_string = "pyproject.toml")]
    PyProject,
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
            ManifestType::PyProject => parse_pyproject_manifest(&file_path).ok(),
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
        name: Some(package.name.clone()),
        description,
        version: Some(package.version().to_string()),
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
        name: pkg.name,
        description: pkg.description,
        version: pkg.version,
        license: pkg.license,
    })
}

/// A PEP 621 `pyproject.toml` `[project]` table.
///
/// Only the fields that map onto [`Manifest`] are deserialized. Tool-specific
/// tables such as `[tool.poetry]` use a different layout and are not handled here.
#[derive(Deserialize)]
struct PyProject {
    project: Option<PyProjectTable>,
}

#[derive(Deserialize)]
struct PyProjectTable {
    name: Option<String>,
    version: Option<String>,
    description: Option<String>,
    license: Option<PyProjectLicense>,
    #[serde(default)]
    dependencies: Vec<String>,
}

/// PEP 621 allows `license` to be either an SPDX expression string (PEP 639) or
/// a table pointing at the license text or a file.
#[derive(Deserialize)]
#[serde(untagged)]
enum PyProjectLicense {
    Spdx(String),
    Table { text: Option<String> },
}

fn parse_pyproject_manifest(path: &Path) -> Result<Manifest> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read pyproject.toml at '{}'", path.display()))?;

    let pyproject: PyProject = toml::from_str(&content)
        .with_context(|| format!("Failed to parse pyproject.toml at '{}'", path.display()))?;

    let project = pyproject
        .project
        .context("pyproject.toml has no [project] table")?;

    // A `license = { file = "LICENSE" }` table carries no identifier, so leave it
    // unset and let onefetch fall back to detecting the license from the repo.
    let license = project.license.and_then(|license| match license {
        PyProjectLicense::Spdx(spdx) => Some(spdx),
        PyProjectLicense::Table { text } => text,
    });

    Ok(Manifest {
        manifest_type: ManifestType::PyProject,
        number_of_dependencies: project.dependencies.len(),
        name: project.name,
        description: project.description,
        version: project.version,
        license,
    })
}

fn file_name_to_manifest_type(filename: &str) -> Option<ManifestType> {
    match filename {
        "Cargo.toml" => Some(ManifestType::Cargo),
        "package.json" => Some(ManifestType::Npm),
        "pyproject.toml" => Some(ManifestType::PyProject),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_pep621_license_forms() {
        // SPDX expression string (PEP 639)
        let spdx: PyProjectTable = toml::from_str("license = \"MIT\"").unwrap();
        assert!(matches!(spdx.license, Some(PyProjectLicense::Spdx(s)) if s == "MIT"));

        // `{ text = "..." }` table (older PEP 621 form)
        let text: PyProjectTable = toml::from_str("license = { text = \"Apache-2.0\" }").unwrap();
        assert!(
            matches!(text.license, Some(PyProjectLicense::Table { text: Some(t) }) if t == "Apache-2.0")
        );

        // `{ file = "LICENSE" }` table carries no identifier
        let file: PyProjectTable = toml::from_str("license = { file = \"LICENSE\" }").unwrap();
        assert!(matches!(
            file.license,
            Some(PyProjectLicense::Table { text: None })
        ));
    }
}
