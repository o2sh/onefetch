use crate::{
    cli::Format,
    info::{
        format_number,
        info_field::{InfoField, InfoType},
    },
};
use onefetch_manifest::Manifest;

pub struct DependenciesInfo {
    pub dependencies: String,
}

impl DependenciesInfo {
    pub fn new(manifest: Option<&Manifest>, format: Option<&Format>) -> Self {
        let dependencies = manifest
            .and_then(|m| {
                (m.number_of_dependencies != 0).then(|| {
                    format!(
                        "{} ({})",
                        format_number(m.number_of_dependencies, format),
                        m.manifest_type
                    )
                })
            })
            .unwrap_or_default();

        Self { dependencies }
    }
}

impl InfoField for DependenciesInfo {
    const TYPE: InfoType = InfoType::Dependencies;

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
            None,
        );

        assert_eq!(dependencies_info.value(), "21 (Cargo)".to_string());
    }
}
