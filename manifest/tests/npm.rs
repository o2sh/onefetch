use anyhow::Result;
use onefetch_manifest::{get_manifests, ManifestType};

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
