use crate::cli::NumberSeparator;
use crate::info::info_field::InfoField;
use crate::info::langs::language::{DEFAULT_CHIP_ICON, Language};
use crate::info::text::{Line, Span, Style};
use owo_colors::{AnsiColors, DynColors};
use serde::Serialize;

const LANGUAGES_BAR_LENGTH: usize = 26;

const LANGUAGES_PER_LINE: usize = 2;

/// Chip colors used when the terminal doesn't support true colors.
const COLOR_PALETTE: [DynColors; 6] = [
    DynColors::Ansi(AnsiColors::Red),
    DynColors::Ansi(AnsiColors::Green),
    DynColors::Ansi(AnsiColors::Yellow),
    DynColors::Ansi(AnsiColors::Blue),
    DynColors::Ansi(AnsiColors::Magenta),
    DynColors::Ansi(AnsiColors::Cyan),
];

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
    number_of_languages_to_display: usize,
    #[serde(skip_serializing)]
    nerd_fonts: bool,
}

impl LanguagesInfo {
    pub fn new(
        loc_by_language: &[(Language, usize)],
        true_color: bool,
        number_of_languages_to_display: usize,
        nerd_fonts: bool,
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
            number_of_languages_to_display,
            nerd_fonts,
        }
    }
}

#[derive(Debug, PartialEq)]
struct LanguageDisplayData {
    language: String,
    percentage: f64,
    chip_color: DynColors,
    chip_icon: char,
}

impl LanguageDisplayData {
    fn label(&self) -> String {
        format!("{} ({:.1} %)", self.language, self.percentage)
    }
}

fn prepare_languages(
    languages_info: &LanguagesInfo,
    color_palette: &[DynColors],
) -> Vec<LanguageDisplayData> {
    let mut iter = languages_info
        .languages_with_percentage
        .iter()
        .enumerate()
        .map(
            |(
                i,
                &LanguageWithPercentage {
                    language,
                    percentage,
                },
            )| {
                let chip_color = if languages_info.true_color {
                    language.get_chip_color()
                } else {
                    color_palette[i % color_palette.len()]
                };

                let chip_icon = language.get_chip_icon(languages_info.nerd_fonts);

                LanguageDisplayData {
                    language: language.to_string(),
                    percentage,
                    chip_color,
                    chip_icon,
                }
            },
        );
    if languages_info.languages_with_percentage.len()
        > languages_info.number_of_languages_to_display
    {
        let mut languages = iter
            .by_ref()
            .take(languages_info.number_of_languages_to_display)
            .collect::<Vec<_>>();
        let other_perc = iter.fold(0.0, |acc, x| acc + x.percentage);
        languages.push(LanguageDisplayData {
            language: "Other".to_string(),
            percentage: other_perc,
            chip_color: DynColors::Ansi(AnsiColors::White),
            chip_icon: DEFAULT_CHIP_ICON,
        });
        languages
    } else {
        iter.collect()
    }
}

fn build_language_bar(languages: &[LanguageDisplayData]) -> Line {
    languages
        .iter()
        .map(|language| {
            let width = (language.percentage / 100. * LANGUAGES_BAR_LENGTH as f64).round() as usize;
            Span::new(
                " ".repeat(width.max(1)),
                Style::Background(language.chip_color),
            )
        })
        .collect::<Vec<_>>()
        .into()
}

fn build_legend_line(languages: &[LanguageDisplayData]) -> Line {
    let mut spans = Vec::new();
    for language in languages {
        if !spans.is_empty() {
            spans.push(Span::plain(" "));
        }
        spans.push(Span::new(
            language.chip_icon,
            Style::Color(language.chip_color),
        ));
        spans.push(Span::plain(" "));
        spans.push(Span::value(language.label()));
    }
    spans.into()
}

#[typetag::serialize]
impl InfoField for LanguagesInfo {
    fn value(&self, _separator: NumberSeparator) -> Vec<Line> {
        let languages = prepare_languages(self, &COLOR_PALETTE);

        let mut lines = vec![build_language_bar(&languages)];
        lines.extend(languages.chunks(LANGUAGES_PER_LINE).map(build_legend_line));
        lines
    }

    fn key(&self) -> String {
        let mut title: String = "Language".into();
        if self.languages_with_percentage.len() > 1 {
            title.push('s');
        }
        title
    }
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
            number_of_languages_to_display: 6,
            nerd_fonts: false,
        };
        let red = DynColors::Ansi(AnsiColors::Red);

        assert_eq!(
            languages_info.value(NumberSeparator::Plain),
            vec![
                Line::from(vec![Span::new(
                    " ".repeat(LANGUAGES_BAR_LENGTH),
                    Style::Background(red),
                )]),
                Line::from(vec![
                    Span::new(DEFAULT_CHIP_ICON, Style::Color(red)),
                    Span::plain(" "),
                    Span::value("Go (100.0 %)"),
                ]),
            ]
        );
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
            number_of_languages_to_display: 2,
            nerd_fonts: false,
        };

        let labels: Vec<String> = prepare_languages(&languages_info, &COLOR_PALETTE)
            .iter()
            .map(LanguageDisplayData::label)
            .collect();

        assert_eq!(labels, ["Go (30.0 %)", "Erlang (40.0 %)", "Other (30.0 %)"]);
    }

    #[test]
    fn test_build_language_bar_multiple_languages() {
        let languages: Vec<LanguageDisplayData> = vec![
            LanguageDisplayData {
                language: "Rust".to_string(),
                percentage: 60.0,
                chip_color: DynColors::Ansi(AnsiColors::Red),
                chip_icon: DEFAULT_CHIP_ICON,
            },
            LanguageDisplayData {
                language: "Python".to_string(),
                percentage: 40.0,
                chip_color: DynColors::Ansi(AnsiColors::Yellow),
                chip_icon: DEFAULT_CHIP_ICON,
            },
        ];
        let result = build_language_bar(&languages);

        let rust_bar_width = (0.6 * LANGUAGES_BAR_LENGTH as f64).round() as usize;
        let python_bar_width = (0.4 * LANGUAGES_BAR_LENGTH as f64).round() as usize;

        let expected_result = Line::from(vec![
            Span::new(
                " ".repeat(rust_bar_width),
                Style::Background(DynColors::Ansi(AnsiColors::Red)),
            ),
            Span::new(
                " ".repeat(python_bar_width),
                Style::Background(DynColors::Ansi(AnsiColors::Yellow)),
            ),
        ]);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_prepare_languages() {
        let languages_info = LanguagesInfo {
            languages_with_percentage: vec![
                LanguageWithPercentage {
                    language: Language::Go,
                    percentage: 40_f64,
                },
                LanguageWithPercentage {
                    language: Language::Erlang,
                    percentage: 30_f64,
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
            number_of_languages_to_display: 2,
            nerd_fonts: false,
        };

        let color_palette = [
            DynColors::Ansi(AnsiColors::Red),
            DynColors::Ansi(AnsiColors::Green),
        ];

        let result = prepare_languages(&languages_info, &color_palette);

        let expected_result = vec![
            LanguageDisplayData {
                language: Language::Go.to_string(),
                percentage: 40_f64,
                chip_color: DynColors::Ansi(AnsiColors::Red),
                chip_icon: DEFAULT_CHIP_ICON,
            },
            LanguageDisplayData {
                language: Language::Erlang.to_string(),
                percentage: 30_f64,
                chip_color: DynColors::Ansi(AnsiColors::Green),
                chip_icon: DEFAULT_CHIP_ICON,
            },
            LanguageDisplayData {
                language: "Other".to_string(),
                percentage: 30_f64,
                chip_color: DynColors::Ansi(AnsiColors::White),
                chip_icon: DEFAULT_CHIP_ICON,
            },
        ];

        assert_eq!(result, expected_result);
    }
}
