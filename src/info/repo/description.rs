use crate::info::info_field::{InfoField, InfoType};
use onefetch_manifest::Manifest;

pub struct DescriptionInfo {
    pub description: Option<String>,
}

impl DescriptionInfo {
    pub fn new(manifest: Option<&Manifest>) -> Self {
        let description = match manifest {
            Some(m) => m.description.clone(),
            None => None,
        };

        Self { description }
    }
}

impl InfoField for DescriptionInfo {
    const TYPE: InfoType = InfoType::Description;

    fn value(&self) -> String {
        match &self.description {
            Some(v) => v.into(),
            None => String::new(),
        }
    }

    fn title(&self) -> String {
        "Description".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use onefetch_manifest::ManifestType;

    #[test]
    fn test_display_description_info() {
        let description_info = DescriptionInfo::new(Some(&Manifest {
            manifest_type: ManifestType::Cargo,
            name: String::new(),
            description: Some("test".into()),
            number_of_dependencies: 0,
            version: "0.1.0".into(),
        }));

        assert_eq!(description_info.value(), "test".to_string());
    }
}
