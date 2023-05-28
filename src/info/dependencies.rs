use crate::{
    cli::NumberSeparator,
    info::{format_number, utils::info_field::InfoField},
};
use onefetch_manifest::Manifest;
use serde::Serialize;

#[derive(Serialize)]
pub struct DependenciesInfo {
    pub dependencies: String,
}

impl DependenciesInfo {
    pub fn new(manifest: Option<&Manifest>, number_separator: NumberSeparator) -> Self {
        let dependencies = manifest
            .and_then(|m| {
                (m.number_of_dependencies != 0).then(|| {
                    format!(
                        "{} ({})",
                        format_number(&m.number_of_dependencies, number_separator),
                        m.manifest_type
                    )
                })
            })
            .unwrap_or_default();

        Self { dependencies }
    }
}

#[typetag::serialize]
impl InfoField for DependenciesInfo {
    fn value(&self) -> String {
        self.dependencies.clone()
    }

    fn title(&self) -> String {
        "Dependencies".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use onefetch_manifest::ManifestType;

    #[test]
    fn should_display_license() {
        let dependencies_info = DependenciesInfo::new(
            Some(&Manifest {
                manifest_type: ManifestType::Cargo,
                name: String::new(),
                description: None,
                number_of_dependencies: 21,
                version: String::new(),
                license: None,
            }),
            NumberSeparator::Plain,
        );

        assert_eq!(dependencies_info.value(), "21 (Cargo)".to_string());
    }
}
