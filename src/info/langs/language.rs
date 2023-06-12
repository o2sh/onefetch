use crate::info::utils::info_field::InfoField;
use owo_colors::OwoColorize;
use serde::Serialize;
use tokei;

include!(concat!(env!("OUT_DIR"), "/language.rs"));

const LANGUAGES_BAR_LENGTH: usize = 26;

#[derive(Serialize)]
pub struct LanguageWithPercentage {
    pub language: Language,
    pub percentage: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguagesInfo {
    pub languages_with_percentage: Vec<LanguageWithPercentage>,
    #[serde(skip_serializing)]
    true_color: bool,
    #[serde(skip_serializing)]
    number_of_languages: usize,
    #[serde(skip_serializing)]
    info_color: DynColors,
}

impl LanguagesInfo {
    pub fn new(
        loc_by_language: &[(Language, usize)],
        true_color: bool,
        number_of_languages: usize,
        info_color: DynColors,
    ) -> Self {
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
            true_color,
            number_of_languages,
            info_color,
        }
    }
}

impl std::fmt::Display for LanguagesInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut languages_info = String::new();
        let pad = self.title().len() + 2;
        let color_palette = vec![
            DynColors::Ansi(AnsiColors::Red),
            DynColors::Ansi(AnsiColors::Green),
            DynColors::Ansi(AnsiColors::Yellow),
            DynColors::Ansi(AnsiColors::Blue),
            DynColors::Ansi(AnsiColors::Magenta),
            DynColors::Ansi(AnsiColors::Cyan),
        ];

        let languages: Vec<(String, f64, DynColors)> = {
            let mut iter = self.languages_with_percentage.iter().enumerate().map(
                |(
                    i,
                    &LanguageWithPercentage {
                        language,
                        percentage,
                    },
                )| {
                    let circle_color = if self.true_color {
                        language.get_circle_color()
                    } else {
                        color_palette[i % color_palette.len()]
                    };
                    (language.to_string(), percentage, circle_color)
                },
            );
            if self.languages_with_percentage.len() > self.number_of_languages {
                let mut languages = iter
                    .by_ref()
                    .take(self.number_of_languages)
                    .collect::<Vec<_>>();
                let other_perc = iter.fold(0.0, |acc, x| acc + x.1);
                languages.push((
                    "Other".to_string(),
                    other_perc,
                    DynColors::Ansi(AnsiColors::White),
                ));
                languages
            } else {
                iter.collect()
            }
        };

        let language_bar: String = languages
            .iter()
            .map(|(_, perc, circle_color)| {
                let bar_width = std::cmp::max(
                    (perc / 100. * LANGUAGES_BAR_LENGTH as f64).round() as usize,
                    1,
                );
                format!("{:<width$}", "".on_color(*circle_color), width = bar_width)
            })
            .collect();

        languages_info.push_str(&language_bar);

        for (i, (language, perc, circle_color)) in languages.iter().enumerate() {
            let formatted_number = format!("{:.*}", 1, perc);
            let circle = "\u{25CF}".color(*circle_color);
            let language_str = format!(
                "{} {} ",
                circle,
                format!("{language} ({formatted_number} %)").color(self.info_color)
            );
            if i % 2 == 0 {
                write!(
                    languages_info,
                    "\n{:<width$}{}",
                    "",
                    language_str,
                    width = pad
                )?;
            } else {
                languages_info.push_str(language_str.trim_end());
            }
        }
        write!(f, "{languages_info}")
    }
}

#[typetag::serialize]
impl InfoField for LanguagesInfo {
    fn value(&self) -> String {
        self.to_string()
    }

    fn title(&self) -> String {
        let mut title: String = "Language".into();
        if self.languages_with_percentage.len() > 1 {
            title.push('s');
        }
        title
    }
}

/// Counts the lines-of-code of a tokei `Language`. Takes into
/// account that a prose language's comments *are* its code.
pub fn loc(language_type: &tokei::LanguageType, language: &tokei::Language) -> usize {
    __loc(language_type, language)
        + language
            .children
            .iter()
            .fold(0, |sum, (lang_type, reports)| {
                sum + reports
                    .iter()
                    .fold(0, |sum, report| sum + stats_loc(lang_type, &report.stats))
            })
}

/// Counts the lines-of-code of a tokei `Report`. This is the child of a
/// `tokei::CodeStats`.
pub fn stats_loc(language_type: &tokei::LanguageType, stats: &tokei::CodeStats) -> usize {
    let stats = stats.summarise();
    __stats_loc(language_type, &stats)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_languages_info() {
        let languages_info = LanguagesInfo {
            languages_with_percentage: vec![LanguageWithPercentage {
                language: Language::Go,
                percentage: 100_f64,
            }],
            true_color: false,
            number_of_languages: 6,
            info_color: DynColors::Ansi(AnsiColors::White),
        };
        let expected_languages_info = format!(
            "{:<width$}\n{:<pad$}{} {} ",
            "".on_color(DynColors::Ansi(AnsiColors::Red)),
            "",
            "\u{25CF}".color(DynColors::Ansi(AnsiColors::Red)),
            "Go (100.0 %)".color(DynColors::Ansi(AnsiColors::White)),
            width = LANGUAGES_BAR_LENGTH,
            pad = "Language".len() + 2
        );

        assert_eq!(languages_info.value(), expected_languages_info);
    }

    #[test]
    fn should_display_correct_number_of_languages() {
        let languages_info = LanguagesInfo {
            languages_with_percentage: vec![
                LanguageWithPercentage {
                    language: Language::Go,
                    percentage: 30_f64,
                },
                LanguageWithPercentage {
                    language: Language::Erlang,
                    percentage: 40_f64,
                },
                LanguageWithPercentage {
                    language: Language::Java,
                    percentage: 20_f64,
                },
                LanguageWithPercentage {
                    language: Language::Rust,
                    percentage: 10_f64,
                },
            ],
            true_color: false,
            number_of_languages: 2,
            info_color: DynColors::Ansi(AnsiColors::White),
        };

        assert!(languages_info.value().contains(
            &"Go (30.0 %)"
                .color(DynColors::Ansi(AnsiColors::White))
                .to_string()
        ));
        assert!(languages_info.value().contains(
            &"Erlang (40.0 %)"
                .color(DynColors::Ansi(AnsiColors::White))
                .to_string()
        ));
        assert!(languages_info.value().contains(
            &"Other (30.0 %)"
                .color(DynColors::Ansi(AnsiColors::White))
                .to_string()
        ));
    }
}
