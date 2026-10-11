use super::FieldView;
use crate::info::fields::languages::{LanguageWithPercentage, Languages};
use crate::language::DEFAULT_CHIP_ICON;
use crate::render::RenderOptions;
use crate::render::text::{Line, Span, Style};
use owo_colors::{AnsiColors, DynColors};

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
    languages: &Languages,
    options: &RenderOptions,
    color_palette: &[DynColors],
) -> Vec<LanguageDisplayData> {
    let mut display_data: Vec<_> = languages
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
                let chip_color = if options.true_color {
                    language.chip_color()
                } else {
                    color_palette[i % color_palette.len()]
                };

                let chip_icon = language.chip_icon(options.nerd_fonts);

                LanguageDisplayData {
                    language: language.to_string(),
                    percentage,
                    chip_color,
                    chip_icon,
                }
            },
        )
        .collect();
    if let Some(percentage) = other_percentage(languages) {
        display_data.push(LanguageDisplayData {
            language: "Other".to_string(),
            percentage,
            chip_color: DynColors::Ansi(AnsiColors::White),
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

impl FieldView for Languages {
    fn value(&self, options: &RenderOptions) -> Vec<Line> {
        let languages = prepare_languages(self, options, &COLOR_PALETTE);

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

    #[test]
    fn test_display_languages() {
        let languages = Languages {
            languages_with_percentage: vec![language(Language::Go, 100_f64)],
        };
        let red = DynColors::Ansi(AnsiColors::Red);

        assert_eq!(
            languages.value(&RenderOptions::default()),
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
        let languages = Languages {
            languages_with_percentage: vec![
                language(Language::Go, 40_f64),
                language(Language::Erlang, 30_f64),
            ],
        };

        let color_palette = [
            DynColors::Ansi(AnsiColors::Red),
            DynColors::Ansi(AnsiColors::Green),
        ];

        let result = prepare_languages(&languages, &RenderOptions::default(), &color_palette);

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

    #[test]
    fn test_key_counts_other_languages() {
        let languages = Languages {
            languages_with_percentage: vec![language(Language::Go, 60_f64)],
        };
        assert_eq!(languages.key(), "Languages");
    }
}
