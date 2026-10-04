use crate::info::utils::{module::Module, text::Line};
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
impl Module for DescriptionInfo {
    fn value(&self) -> Vec<Line> {
        match &self.description {
            Some(description) => break_sentence_into_lines(description)
                .into_iter()
                .map(Line::from)
                .collect(),
            None => Vec::new(),
        }
    }

    fn key(&self) -> String {
        "Description".into()
    }
}

fn break_sentence_into_lines(sentence: &str) -> Vec<String> {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    words
        .chunks(NUMBER_OF_WORDS_PER_LINE)
        .map(|chunk| chunk.join(" "))
        .collect()
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
            name: None,
            description: Some("test".into()),
            number_of_dependencies: 0,
            version: Some("0.1.0".into()),
            license: None,
        }));

        assert_eq!(description_info.value(), vec![Line::from("test")]);
    }

    #[rstest]
    #[case("Hello", &["Hello"])]
    #[case("Hello world, how are you doing?", &["Hello world, how are you", "doing?"])]
    #[case(
        "This is a long sentence that needs to be broken into multiple lines.",
        &["This is a long sentence", "that needs to be broken", "into multiple lines."]
    )]
    fn test_break_sentence_into_lines(#[case] sentence: &str, #[case] expected_result: &[&str]) {
        assert_eq!(break_sentence_into_lines(sentence), expected_result);
    }
}
