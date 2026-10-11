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
    pub languages_with_percentage: Vec<LanguageWithPercentage>,
}

impl Languages {
    pub fn new(loc_by_language: &[(Language, usize)]) -> Self {
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

        let languages_with_percentage = weight_by_language
            .into_iter()
            .map(|(language, percentage)| LanguageWithPercentage {
                language,
                percentage,
            })
            .collect();
        Self {
            languages_with_percentage,
        }
    }
}
