use super::FieldView;
use crate::data::fields::dependencies::Dependencies;
use crate::render::RenderOptions;
use crate::render::text::Line;

impl FieldView for Dependencies {
    fn value(&self, options: &RenderOptions) -> Vec<Line> {
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
    use crate::data::manifest::{Manifest, ManifestType};

    #[test]
    fn should_display_dependencies() {
        let dependencies = Dependencies::new(Some(&Manifest {
            manifest_type: ManifestType::Cargo,
            name: None,
            description: None,
            number_of_dependencies: 21,
            version: None,
            license: None,
        }));

        assert_eq!(
            dependencies.value(&RenderOptions::default()),
            vec![Line::from("21 (Cargo)")]
        );
    }
}
