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
    use rstest::rstest;

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

    #[rstest]
    #[case("Hello", "Hello")]
    #[case(
        "Hello world, how are you doing?",
        "Hello world, how are you\n    doing?"
    )]
    #[case(
        "This is a long sentence that needs to be broken into multiple lines.",
        "This is a long sentence\n    that needs to be broken\n    into multiple lines."
    )]
    fn test_break_sentence_into_lines(#[case] sentence: &str, #[case] expected_result: &str) {
        assert_eq!(break_sentence_into_lines(sentence, 4), expected_result);
    }
}
