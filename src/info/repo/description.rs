use crate::info::info_field::{InfoField, InfoType};
use manifest::Manifest;

pub struct DescriptionInfo {
    pub description: Option<String>,
}

impl DescriptionInfo {
    pub fn new(manifest: &Option<Manifest>) -> Self {
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
            Some(v) => v.to_string(),
            None => String::from(""),
        }
    }

    fn title(&self) -> String {
        String::from("Description")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_description_info() {
        let description_info = DescriptionInfo::new(&Some(Manifest {
            manifest_type: manifest::ManifestType::Cargo,
            name: String::new(),
            description: Some(String::from("test")),
            number_of_dependencies: 0,
        }));

        assert_eq!(description_info.value(), "test".to_string());
    }
}
