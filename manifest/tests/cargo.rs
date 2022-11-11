use anyhow::Result;
use onefetch_manifest::{get_manifests, ManifestType};

#[test]
fn should_detect_and_parse_cargo_manifest() -> Result<()> {
    let manifests = get_manifests("tests/fixtures/cargo")?;
    assert_eq!(manifests.len(), 1);
    let cargo_manifest = manifests.first().unwrap();
    assert_eq!(cargo_manifest.manifest_type, ManifestType::Cargo);
    assert_eq!(cargo_manifest.number_of_dependencies, 5);
    assert_eq!(cargo_manifest.name, String::from("project"));
    assert_eq!(
        cargo_manifest.description,
        Some("this is a description".into())
    );
    assert_eq!(cargo_manifest.version, String::from("0.1.0"));
    assert_eq!(cargo_manifest.license, Some("MIT".into()));

    Ok(())
}
