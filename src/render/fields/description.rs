use super::FieldView;
use crate::info::fields::description::Description;
use crate::render::format::FormatOptions;
use crate::render::text::Line;

const NUMBER_OF_WORDS_PER_LINE: usize = 5;

impl FieldView for Description {
    fn value(&self, _options: &FormatOptions) -> Vec<Line> {
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
    use crate::info::manifest::{Manifest, ManifestType};
    use rstest::rstest;

    #[test]
    fn should_display_description() {
        let description = Description::new(Some(&Manifest {
            manifest_type: ManifestType::Cargo,
            name: None,
            description: Some("test".into()),
            number_of_dependencies: 0,
            version: Some("0.1.0".into()),
            license: None,
        }));

        assert_eq!(
            description.value(&FormatOptions::default()),
            vec![Line::from("test")]
        );
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
