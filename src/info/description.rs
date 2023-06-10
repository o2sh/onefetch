use crate::info::utils::info_field::InfoField;
use onefetch_manifest::Manifest;
use serde::Serialize;

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
            Some(description) => {
                let left_pad = self.title().len() + 2;
                break_sentence_into_lines(description, left_pad)
            }
            None => String::new(),
        }
    }

    fn title(&self) -> String {
        "Description".into()
    }
}

fn break_sentence_into_lines(sentence: &str, left_pad: usize) -> String {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    let mut lines = Vec::new();

    for (i, chunk) in words.chunks(NUMBER_OF_WORDS_PER_LINE).enumerate() {
        let line = if i == 0 {
            chunk.join(" ")
        } else {
            format!("{:>width$}{}", "", chunk.join(" "), width = left_pad)
        };
        lines.push(line);
    }

    lines.join("\n")
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
