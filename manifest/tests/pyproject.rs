use anyhow::Result;
use onefetch_manifest::{ManifestType, get_manifests};

#[test]
fn should_detect_and_parse_pyproject_manifest() -> Result<()> {
    let manifests = get_manifests("tests/fixtures/pyproject")?;
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
