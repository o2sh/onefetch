use super::FieldView;
use crate::data::fields::languages::{LanguageWithPercentage, Languages};
use crate::language::DEFAULT_CHIP_ICON;
use crate::render::RenderOptions;
use crate::render::style::{Chip, Line, Segment, Style};

const LANGUAGES_BAR_LENGTH: usize = 26;

const LANGUAGES_PER_LINE: usize = 2;

#[derive(Debug, PartialEq)]
struct LanguageDisplayData {
    language: String,
    percentage: f64,
    chip: Chip,
    chip_icon: char,
}

impl LanguageDisplayData {
    fn label(&self) -> String {
        format!("{} ({:.1} %)", self.language, self.percentage)
    }
}

fn prepare_languages(languages: &Languages, options: &RenderOptions) -> Vec<LanguageDisplayData> {
    let mut display_data: Vec<_> = languages
        .languages_with_percentage
        .iter()
        .enumerate()
        .map(
            |(
                rank,
                &LanguageWithPercentage {
                    language,
                    percentage,
                },
            )| LanguageDisplayData {
                language: language.to_string(),
                percentage,
                chip: Chip {
                    language: Some(language),
                    rank,
                },
                chip_icon: language.chip_icon(options.nerd_fonts),
            },
        )
        .collect();
    if let Some(percentage) = other_percentage(languages) {
        display_data.push(LanguageDisplayData {
            language: "Other".to_string(),
            percentage,
            chip: Chip {
                language: None,
                rank: display_data.len(),
            },
            chip_icon: DEFAULT_CHIP_ICON,
        });
    }
    display_data
}

/// The share of the languages left out by `--number-of-languages`, if any.
fn other_percentage(languages: &Languages) -> Option<f64> {
    let shown: f64 = languages
        .languages_with_percentage
        .iter()
        .map(|l| l.percentage)
        .sum();
    let other = 100.0 - shown;
    // The tolerance absorbs floating-point error when no language was left out.
    (other > 1e-9).then_some(other)
}

fn build_language_bar(languages: &[LanguageDisplayData]) -> Line {
    languages
        .iter()
        .map(|language| {
            let width = (language.percentage / 100. * LANGUAGES_BAR_LENGTH as f64).round() as usize;
            Segment::new(" ".repeat(width.max(1)), Style::ChipBar(language.chip))
        })
        .collect::<Vec<_>>()
        .into()
}

fn build_legend_line(languages: &[LanguageDisplayData]) -> Line {
    let mut segments = Vec::new();
    for language in languages {
        if !segments.is_empty() {
            segments.push(Segment::plain(" "));
        }
        segments.push(Segment::new(language.chip_icon, Style::Chip(language.chip)));
        segments.push(Segment::plain(" "));
        segments.push(Segment::value(language.label()));
    }
    segments.into()
}

impl FieldView for Languages {
    fn value(&self, options: &RenderOptions) -> Vec<Line> {
        let languages = prepare_languages(self, options);

        let mut lines = vec![build_language_bar(&languages)];
        lines.extend(languages.chunks(LANGUAGES_PER_LINE).map(build_legend_line));
        lines
    }

    fn key(&self) -> String {
        let mut title: String = "Language".into();
        let entries =
            self.languages_with_percentage.len() + usize::from(other_percentage(self).is_some());
        if entries > 1 {
            title.push('s');
        }
        title
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::language::Language;

    fn language(language: Language, percentage: f64) -> LanguageWithPercentage {
        LanguageWithPercentage {
            language,
            percentage,
        }
    }

    fn chip(language: Option<Language>, rank: usize) -> Chip {
        Chip { language, rank }
    }

    #[test]
    fn test_display_languages() {
        let languages = Languages {
            languages_with_percentage: vec![language(Language::Go, 100_f64)],
        };
        let go = chip(Some(Language::Go), 0);

        assert_eq!(
            languages.value(&RenderOptions::default()),
            vec![
                Line::from(vec![Segment::new(
                    " ".repeat(LANGUAGES_BAR_LENGTH),
                    Style::ChipBar(go),
                )]),
                Line::from(vec![
                    Segment::new(DEFAULT_CHIP_ICON, Style::Chip(go)),
                    Segment::plain(" "),
                    Segment::value("Go (100.0 %)"),
                ]),
            ]
        );
    }

    #[test]
    fn test_build_language_bar_multiple_languages() {
        let rust = chip(Some(Language::Rust), 0);
        let python = chip(Some(Language::Python), 1);
        let languages: Vec<LanguageDisplayData> = vec![
            LanguageDisplayData {
                language: "Rust".to_string(),
                percentage: 60.0,
                chip: rust,
                chip_icon: DEFAULT_CHIP_ICON,
            },
            LanguageDisplayData {
                language: "Python".to_string(),
                percentage: 40.0,
                chip: python,
                chip_icon: DEFAULT_CHIP_ICON,
            },
        ];
        let result = build_language_bar(&languages);

        let rust_bar_width = (0.6 * LANGUAGES_BAR_LENGTH as f64).round() as usize;
        let python_bar_width = (0.4 * LANGUAGES_BAR_LENGTH as f64).round() as usize;

        let expected_result = Line::from(vec![
            Segment::new(" ".repeat(rust_bar_width), Style::ChipBar(rust)),
            Segment::new(" ".repeat(python_bar_width), Style::ChipBar(python)),
        ]);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_prepare_languages() {
        let languages = Languages {
            languages_with_percentage: vec![
                language(Language::Go, 40_f64),
                language(Language::Erlang, 30_f64),
            ],
        };

        let result = prepare_languages(&languages, &RenderOptions::default());

        let expected_result = vec![
            LanguageDisplayData {
                language: Language::Go.to_string(),
                percentage: 40_f64,
                chip: chip(Some(Language::Go), 0),
                chip_icon: DEFAULT_CHIP_ICON,
            },
            LanguageDisplayData {
                language: Language::Erlang.to_string(),
                percentage: 30_f64,
                chip: chip(Some(Language::Erlang), 1),
                chip_icon: DEFAULT_CHIP_ICON,
            },
            LanguageDisplayData {
                language: "Other".to_string(),
                percentage: 30_f64,
                chip: chip(None, 2),
                chip_icon: DEFAULT_CHIP_ICON,
            },
        ];

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_key_counts_other_languages() {
        let languages = Languages {
            languages_with_percentage: vec![language(Language::Go, 60_f64)],
        };
        assert_eq!(languages.key(), "Languages");
    }
}
