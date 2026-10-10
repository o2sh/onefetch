use crate::info::display_options::DisplayOptions;
use crate::info::fields::InfoField;
use crate::info::text::Line;
use onefetch_manifest::Manifest;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DependenciesInfo {
    pub number_of_dependencies: usize,
    pub manifest_type: Option<String>,
}

impl DependenciesInfo {
    pub fn new(manifest: Option<&Manifest>) -> Self {
        Self {
            number_of_dependencies: manifest.map_or(0, |m| m.number_of_dependencies),
            manifest_type: manifest.map(|m| m.manifest_type.to_string()),
        }
    }
}

#[typetag::serialize]
impl InfoField for DependenciesInfo {
    fn value(&self, options: &DisplayOptions) -> Vec<Line> {
        match &self.manifest_type {
            Some(manifest_type) if self.number_of_dependencies > 0 => {
                let dependencies = options.number(&self.number_of_dependencies);
                vec![Line::from(format!("{dependencies} ({manifest_type})"))]
            }
            _ => Vec::new(),
        }
    }

    fn key(&self) -> String {
        "Dependencies".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use onefetch_manifest::ManifestType;

    #[test]
    fn should_display_dependencies() {
        let dependencies_info = DependenciesInfo::new(Some(&Manifest {
            manifest_type: ManifestType::Cargo,
            name: None,
            description: None,
            number_of_dependencies: 21,
            version: None,
            license: None,
        }));

        assert_eq!(
            dependencies_info.value(&DisplayOptions::default()),
            vec![Line::from("21 (Cargo)")]
        );
    }
}
