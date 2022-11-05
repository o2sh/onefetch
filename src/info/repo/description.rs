use crate::info::info_field::{InfoField, InfoType};
use anyhow::Result;
use manifest::Manifest;

pub struct DescriptionInfo {
    pub description: Option<String>,
}

impl DescriptionInfo {
    pub fn new(manifest: &Option<Manifest>) -> Result<Self> {
        let description = match manifest {
            Some(m) => m.description.clone(),
            None => None,
        };

        Ok(Self { description })
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
        let loc_info = DescriptionInfo {
            description: Some(String::from("")),
        };

        assert_eq!(loc_info.value(), "1235".to_string());
    }
}
