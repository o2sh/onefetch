use crate::info::info_field::{InfoField, InfoType};
use onefetch_manifest::Manifest;

pub struct DependenciesInfo {
    pub dependencies: String,
}

impl DependenciesInfo {
    pub fn new(manifest: Option<&Manifest>) -> Self {
        let dependencies = manifest
            .as_ref()
            .and_then(|m| {
                (m.number_of_dependencies != 0)
                    .then(|| format!("{} ({})", m.number_of_dependencies, m.manifest_type))
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
