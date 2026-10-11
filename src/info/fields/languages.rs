use crate::language::Language;
use serde::Serialize;

#[derive(Serialize)]
pub struct LanguageWithPercentage {
    pub language: Language,
    pub percentage: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Languages {
    /// The `--number-of-languages` largest languages.
    pub languages_with_percentage: Vec<LanguageWithPercentage>,
    /// The rest, shown as "Other" in the terminal.
    #[serde(skip)]
    pub other_languages: Vec<LanguageWithPercentage>,
}

impl Languages {
    pub fn new(loc_by_language: &[(Language, usize)], number_of_languages: usize) -> Self {
        let total: usize = loc_by_language.iter().map(|(_, v)| v).sum();

        let weight_by_language: Vec<(Language, f64)> = loc_by_language
            .iter()
            .map(|(k, v)| {
                let mut val = *v as f64;
                val /= total as f64;
                val *= 100_f64;
                (*k, val)
            })
            .collect();

        let mut languages_with_percentage: Vec<LanguageWithPercentage> = weight_by_language
            .into_iter()
            .map(|(language, percentage)| LanguageWithPercentage {
                language,
                percentage,
            })
            .collect();
        let other_languages =
            languages_with_percentage.split_off(number_of_languages.min(loc_by_language.len()));
        Self {
            languages_with_percentage,
            other_languages,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn keeps_the_largest_languages() {
        let loc_by_language = [
            (Language::Go, 40),
            (Language::Erlang, 30),
            (Language::Java, 20),
            (Language::Rust, 10),
        ];

        let languages = Languages::new(&loc_by_language, 2);

        let kept: Vec<Language> = languages
            .languages_with_percentage
            .iter()
            .map(|l| l.language)
            .collect();
        let other: Vec<Language> = languages
            .other_languages
            .iter()
            .map(|l| l.language)
            .collect();
        assert_eq!(kept, [Language::Go, Language::Erlang]);
        assert_eq!(other, [Language::Java, Language::Rust]);
    }
}
