use crate::info::utils::info_field::{InfoField, InfoType};
use onefetch_manifest::Manifest;
use serde::Serialize;
use std::fmt::Write;

const NUMBER_OF_WORDS_PER_LINE: usize = 5;

#[derive(Serialize)]
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

#[typetag::serialize]
impl InfoField for DescriptionInfo {
    fn value(&self) -> String {
        match &self.description {
            Some(value) => {
                if value.contains(' ') {
                    let pad = self.title().len() + 2;
                    let words = value.trim().split(' ');
                    let mut description = String::new();
                    for (i, word) in words.enumerate() {
                        if i != 0 && i % NUMBER_OF_WORDS_PER_LINE == 0 {
                            let _ = write!(description, "\n{:<width$}{} ", "", word, width = pad);
                        } else {
                            let _ = write!(description, "{word} ");
                        }
                    }

                    description.trim_end().into()
                } else {
                    value.into()
                }
            }
            None => String::new(),
        }
    }

    fn title(&self) -> String {
        "Description".into()
    }

    fn r#type(&self) -> InfoType {
        InfoType::Description
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use onefetch_manifest::ManifestType;

    #[test]
    fn should_display_description() {
        let description_info = DescriptionInfo::new(Some(&Manifest {
            manifest_type: ManifestType::Cargo,
            name: String::new(),
            description: Some("test".into()),
            number_of_dependencies: 0,
            version: "0.1.0".into(),
            license: None,
        }));

        assert_eq!(description_info.value(), "test".to_string());
    }

    #[test]
    fn should_split_long_description_into_multiple_lines() {
        let description_info = DescriptionInfo::new(Some(&Manifest {
            manifest_type: ManifestType::Cargo,
            name: String::new(),
            description: Some("This is a very long description with a lot of text".into()),
            number_of_dependencies: 0,
            version: String::new(),
            license: None,
        }));

        assert_eq!(
            description_info.value().lines().count(),
            (description_info.description.unwrap().split(' ').count() as f32
                / NUMBER_OF_WORDS_PER_LINE as f32)
                .ceil() as usize
        );
    }
}
