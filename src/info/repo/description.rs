use crate::info::info_field::{InfoField, InfoType};
use onefetch_manifest::Manifest;
use std::fmt::Write;

const NUMBER_OF_WORDS_PER_LINE: usize = 5;

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
            Some(value) => {
                if value.contains(' ') {
                    let pad = self.title().len() + 2;
                    let words = value.trim().split(' ');
                    let mut description = String::new();
                    for (i, word) in words.enumerate() {
                        if i != 0 && i % NUMBER_OF_WORDS_PER_LINE == 0 {
                            let _ = write!(description, "\n{:<width$}{} ", "", word, width = pad);
                        } else {
                            let _ = write!(description, "{} ", word);
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
