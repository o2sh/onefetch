use anyhow::Result;
use onefetch_manifest::{ManifestType, get_manifests};

#[test]
fn should_detect_and_parse_npm_manifest() -> Result<()> {
    let manifests = get_manifests("tests/fixtures/npm")?;
    assert_eq!(manifests.len(), 1);
    let npm_manifest = manifests.first().unwrap();
    assert_eq!(npm_manifest.manifest_type, ManifestType::Npm);
    assert_eq!(npm_manifest.number_of_dependencies, 3);
    assert_eq!(npm_manifest.name, String::from("my_package"));
    assert_eq!(
        npm_manifest.description,
        Some("description for my_package".into())
    );
    assert_eq!(npm_manifest.version, String::from("1.0.0"));
    assert_eq!(npm_manifest.license, Some("ISC".into()));
    Ok(())
}

#[test]
fn should_only_count_dependencies_for_npm_manifest() -> Result<()> {
    let manifests = get_manifests("tests/fixtures/npm_only_dependencies_counted")?;
    assert_eq!(manifests.len(), 1);
    let npm_manifest = manifests.first().unwrap();
    assert_eq!(npm_manifest.manifest_type, ManifestType::Npm);
    assert_eq!(npm_manifest.number_of_dependencies, 2);
    Ok(())
}

#[test]
fn should_count_zero_when_npm_dependencies_field_is_missing() -> Result<()> {
    let manifests = get_manifests("tests/fixtures/npm_without_dependencies")?;
    assert_eq!(manifests.len(), 1);
    let npm_manifest = manifests.first().unwrap();
    assert_eq!(npm_manifest.manifest_type, ManifestType::Npm);
    assert_eq!(npm_manifest.number_of_dependencies, 0);
    Ok(())
}
