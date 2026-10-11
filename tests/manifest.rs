use anyhow::Result;
use onefetch::info::manifest::{ManifestType, get_manifests};

#[test]
fn should_detect_and_parse_cargo_manifest() -> Result<()> {
    let manifests = get_manifests("tests/fixtures/manifest/cargo")?;
    assert_eq!(manifests.len(), 1);
    let cargo_manifest = manifests.first().unwrap();
    assert_eq!(cargo_manifest.manifest_type, ManifestType::Cargo);
    assert_eq!(cargo_manifest.number_of_dependencies, 5);
    assert_eq!(cargo_manifest.name, Some(String::from("project")));
    assert_eq!(
        cargo_manifest.description,
        Some("this is a description".into())
    );
    assert_eq!(cargo_manifest.version, Some(String::from("0.1.0")));
    assert_eq!(cargo_manifest.license, Some("MIT".into()));

    Ok(())
}

#[test]
fn should_detect_and_parse_npm_manifest() -> Result<()> {
    let manifests = get_manifests("tests/fixtures/manifest/npm")?;
    assert_eq!(manifests.len(), 1);
    let npm_manifest = manifests.first().unwrap();
    assert_eq!(npm_manifest.manifest_type, ManifestType::Npm);
    assert_eq!(npm_manifest.number_of_dependencies, 3);
    assert_eq!(npm_manifest.name, Some(String::from("my_package")));
    assert_eq!(
        npm_manifest.description,
        Some("description for my_package".into())
    );
    assert_eq!(npm_manifest.version, Some(String::from("1.0.0")));
    assert_eq!(npm_manifest.license, Some("ISC".into()));
    Ok(())
}

#[test]
fn should_detect_and_parse_pyproject_manifest() -> Result<()> {
    let manifests = get_manifests("tests/fixtures/manifest/pyproject")?;
    assert_eq!(manifests.len(), 1);
    let pyproject_manifest = manifests.first().unwrap();
    assert_eq!(pyproject_manifest.manifest_type, ManifestType::PyProject);
    assert_eq!(pyproject_manifest.number_of_dependencies, 3);
    assert_eq!(pyproject_manifest.name, Some(String::from("my_package")));
    assert_eq!(
        pyproject_manifest.description,
        Some("description for my_package".into())
    );
    assert_eq!(pyproject_manifest.version, Some(String::from("1.0.0")));
    assert_eq!(pyproject_manifest.license, Some("MIT".into()));
    Ok(())
}
