//! How each field is shown in the terminal: its key and its value lines.

use crate::info::Field;
use crate::info::fields::authors::Authors;
use crate::info::fields::churn::Churn;
use crate::info::fields::commits::Commits;
use crate::info::fields::contributors::Contributors;
use crate::info::fields::created::Created;
use crate::info::fields::dependencies::Dependencies;
use crate::info::fields::description::Description;
use crate::info::fields::head::{Head, HeadRefs};
use crate::info::fields::languages::{LanguageWithPercentage, Languages};
use crate::info::fields::last_change::LastChange;
use crate::info::fields::license::License;
use crate::info::fields::lines_of_code::LinesOfCode;
use crate::info::fields::pending::Pending;
use crate::info::fields::project::Project;
use crate::info::fields::size::Size;
use crate::info::fields::url::Url;
use crate::info::fields::version::Version;
use crate::language::DEFAULT_CHIP_ICON;
use crate::render::format::DisplayOptions;
use crate::render::text::{Line, Span, Style};
use byte_unit::{Byte, UnitType};
use owo_colors::{AnsiColors, DynColors};

pub(crate) trait FieldView {
    fn key(&self) -> String;
    fn value(&self, options: &DisplayOptions) -> Vec<Line>;
}

pub(crate) fn view(field: &Field) -> &dyn FieldView {
    match field {
        Field::Project(f) => f,
        Field::Description(f) => f,
        Field::Head(f) => f,
        Field::Pending(f) => f,
        Field::Version(f) => f,
        Field::Created(f) => f,
        Field::Languages(f) => f,
        Field::Dependencies(f) => f,
        Field::Authors(f) => f,
        Field::LastChange(f) => f,
        Field::Contributors(f) => f,
        Field::Url(f) => f,
        Field::Commits(f) => f,
        Field::Churn(f) => f,
        Field::LinesOfCode(f) => f,
        Field::Size(f) => f,
        Field::License(f) => f,
    }
}

impl FieldView for Project {
    fn value(&self, options: &DisplayOptions) -> Vec<Line> {
        if self.repo_name.is_empty() {
            return Vec::new();
        }

        let branches = match self.number_of_branches {
            0 => String::new(),
            1 => "1 branch".into(),
            _ => format!("{} branches", options.number(&self.number_of_branches)),
        };

        let tags = match self.number_of_tags {
            0 => String::new(),
            1 => "1 tag".into(),
            _ => format!("{} tags", options.number(&self.number_of_tags)),
        };

        let project = if tags.is_empty() && branches.is_empty() {
            self.repo_name.clone()
        } else if branches.is_empty() || tags.is_empty() {
            format!("{} ({}{})", self.repo_name, tags, branches)
        } else {
            format!("{} ({}, {})", self.repo_name, branches, tags)
        };
        vec![Line::from(project)]
    }

    fn key(&self) -> String {
        "Project".into()
    }
}

const NUMBER_OF_WORDS_PER_LINE: usize = 5;

impl FieldView for Description {
    fn value(&self, _options: &DisplayOptions) -> Vec<Line> {
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

impl FieldView for Head {
    fn value(&self, _options: &DisplayOptions) -> Vec<Line> {
        let HeadRefs {
            short_commit_id,
            refs,
        } = &self.head_refs;
        let head = if refs.is_empty() {
            short_commit_id.clone()
        } else {
            format!("{short_commit_id} ({})", refs.join(", "))
        };
        vec![Line::from(head)]
    }

    fn key(&self) -> String {
        "HEAD".into()
    }
}

impl FieldView for Pending {
    fn value(&self, _options: &DisplayOptions) -> Vec<Line> {
        let mut pending = String::new();
        if self.modified > 0 {
            pending = format!("{}+-", self.modified);
        }

        if self.added > 0 {
            pending = format!("{pending} {}+", self.added);
        }

        if self.deleted > 0 {
            pending = format!("{pending} {}-", self.deleted);
        }

        vec![Line::from(pending.trim())]
    }

    fn key(&self) -> String {
        "Pending".into()
    }
}

impl FieldView for Version {
    fn value(&self, _options: &DisplayOptions) -> Vec<Line> {
        vec![Line::from(self.version.to_string())]
    }

    fn key(&self) -> String {
        "Version".into()
    }
}

impl FieldView for Created {
    fn value(&self, options: &DisplayOptions) -> Vec<Line> {
        vec![Line::from(options.time(self.creation_date))]
    }

    fn key(&self) -> String {
        "Created".into()
    }
}

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
    options: &DisplayOptions,
    color_palette: &[DynColors],
) -> Vec<LanguageDisplayData> {
    let mut iter = languages.languages_with_percentage.iter().enumerate().map(
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
    );
    if languages.languages_with_percentage.len() > options.number_of_languages {
        let mut languages = iter
            .by_ref()
            .take(options.number_of_languages)
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

impl FieldView for Languages {
    fn value(&self, options: &DisplayOptions) -> Vec<Line> {
        let languages = prepare_languages(self, options, &COLOR_PALETTE);

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

impl FieldView for Dependencies {
    fn value(&self, options: &DisplayOptions) -> Vec<Line> {
        match &self.manifest_type {
            Some(manifest_type) if self.number_of_dependencies > 0 => {
                let dependencies = options.number(&self.number_of_dependencies);
                vec![Line::from(format!("{dependencies} ({manifest_type})"))]
            }
            _ => Vec::new(),
        }
    }

    fn key(&self) -> String {
        "Dependencies".into()
    }
}

impl FieldView for Authors {
    fn value(&self, options: &DisplayOptions) -> Vec<Line> {
        let top_contribution = self.authors.first().map_or(0, |a| a.contribution);
        self.authors
            .iter()
            .map(|author| {
                let pad = digit_difference(top_contribution, author.contribution);
                let contribution = format!("{:pad$}{}%", "", author.contribution);
                let commits = options.number(&author.nbr_of_commits);
                let line = match &author.email {
                    Some(email) => format!("{contribution} {} <{email}> {commits}", author.name),
                    None => format!("{contribution} {} {commits}", author.name),
                };
                Line::from(line)
            })
            .collect()
    }

    fn key(&self) -> String {
        let mut title: String = "Author".into();
        if self.authors.len() > 1 {
            title.push('s');
        }
        title
    }
}

fn digit_difference(num1: usize, num2: usize) -> usize {
    let count_digits = |num: usize| (num.checked_ilog10().unwrap_or(0) + 1) as usize;
    count_digits(num1).abs_diff(count_digits(num2))
}

impl FieldView for LastChange {
    fn value(&self, options: &DisplayOptions) -> Vec<Line> {
        vec![Line::from(options.time(self.last_change))]
    }

    fn key(&self) -> String {
        "Last change".into()
    }
}

impl FieldView for Contributors {
    fn value(&self, options: &DisplayOptions) -> Vec<Line> {
        if self.total_number_of_authors > options.number_of_authors {
            vec![Line::from(options.number(&self.total_number_of_authors))]
        } else {
            Vec::new()
        }
    }

    fn key(&self) -> String {
        "Contributors".into()
    }
}

impl FieldView for Url {
    fn value(&self, _options: &DisplayOptions) -> Vec<Line> {
        vec![Line::from(self.repo_url.to_string())]
    }

    fn key(&self) -> String {
        "URL".into()
    }
}

impl FieldView for Commits {
    fn value(&self, options: &DisplayOptions) -> Vec<Line> {
        let commits = format!(
            "{}{}",
            options.number(&self.number_of_commits),
            if self.is_shallow {
                " (shallow)"
            } else {
                Default::default()
            }
        );
        vec![Line::from(commits)]
    }

    fn key(&self) -> String {
        "Commits".into()
    }
}

impl FieldView for Churn {
    fn value(&self, options: &DisplayOptions) -> Vec<Line> {
        self.file_churns
            .iter()
            .map(|file_churn| {
                Line::from(format!(
                    "{} {}",
                    shorten_file_path(&file_churn.file_path, 2),
                    options.number(&file_churn.nbr_of_commits)
                ))
            })
            .collect()
    }

    fn key(&self) -> String {
        format!("Churn ({})", self.churn_pool_size)
    }
}

fn shorten_file_path(file_path: &str, depth: usize) -> String {
    let components: Vec<&str> = file_path.split('/').collect();

    if depth == 0 || components.len() <= depth {
        return file_path.to_string();
    }

    let truncated_components: Vec<&str> = components
        .iter()
        .skip(components.len() - depth)
        .copied()
        .collect();

    format!("\u{2026}/{}", truncated_components.join("/"))
}

impl FieldView for LinesOfCode {
    fn value(&self, options: &DisplayOptions) -> Vec<Line> {
        vec![Line::from(options.number(&self.lines_of_code))]
    }

    fn key(&self) -> String {
        "Lines of code".into()
    }
}

impl FieldView for Size {
    fn value(&self, options: &DisplayOptions) -> Vec<Line> {
        let repo_size = bytes_to_human_readable(self.repo_size);
        let size = match self.file_count {
            0 => repo_size,
            1 => format!("{repo_size} (1 file)"),
            _ => format!("{repo_size} ({} files)", options.number(&self.file_count)),
        };
        vec![Line::from(size)]
    }

    fn key(&self) -> String {
        "Size".into()
    }
}

fn bytes_to_human_readable(bytes: u64) -> String {
    let byte = Byte::from_u64(bytes);
    let adjusted_byte_based = byte.get_appropriate_unit(UnitType::Binary);
    format!("{adjusted_byte_based:#.2}")
}

impl FieldView for License {
    fn value(&self, _options: &DisplayOptions) -> Vec<Line> {
        vec![Line::from(self.license.to_string())]
    }

    fn key(&self) -> String {
        "License".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::info::fields::authors::Author;
    use crate::info::fields::churn::FileChurn;
    use crate::info::manifest::{Manifest, ManifestType};
    use crate::language::Language;
    use gix::date::Time;
    use rstest::rstest;

    fn project(repo_name: &str, number_of_branches: usize, number_of_tags: usize) -> Project {
        Project {
            repo_name: repo_name.to_string(),
            number_of_branches,
            number_of_tags,
        }
    }

    #[rstest]
    #[case(project("onefetch", 3, 2), "onefetch (3 branches, 2 tags)")]
    #[case(project("onefetch", 0, 0), "onefetch")]
    #[case(project("onefetch", 3, 0), "onefetch (3 branches)")]
    #[case(project("onefetch", 0, 2), "onefetch (2 tags)")]
    #[case(project("onefetch", 1, 1), "onefetch (1 branch, 1 tag)")]
    fn test_display_project(#[case] project: Project, #[case] expected: &str) {
        assert_eq!(
            project.value(&DisplayOptions::default()),
            vec![Line::from(expected)]
        );
    }

    #[test]
    fn test_display_project_when_no_repo_name() {
        assert!(
            project("", 0, 0)
                .value(&DisplayOptions::default())
                .is_empty()
        );
    }

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
            description.value(&DisplayOptions::default()),
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

    #[test]
    fn test_display_head_refs() {
        let head = Head {
            head_refs: HeadRefs::new("be561d5".into(), vec!["main".into(), "origin/main".into()]),
        };
        assert_eq!(
            head.value(&DisplayOptions::default()),
            vec![Line::from("be561d5 (main, origin/main)")]
        );
    }

    #[test]
    fn test_display_head_refs_with_no_refs() {
        let head = Head {
            head_refs: HeadRefs::new("be561d5".into(), vec![]),
        };
        assert_eq!(
            head.value(&DisplayOptions::default()),
            vec![Line::from("be561d5")]
        );
    }

    #[test]
    fn test_display_pending() {
        let pending = Pending {
            added: 0,
            deleted: 0,
            modified: 4,
        };

        assert_eq!(
            pending.value(&DisplayOptions::default()),
            vec![Line::from("4+-")]
        );
    }

    #[test]
    fn test_display_created() {
        let created = Created {
            creation_date: Time::new(946_771_200, 0),
        };
        let iso_time = DisplayOptions {
            iso_time: true,
            ..DisplayOptions::default()
        };

        assert_eq!(
            created.value(&iso_time),
            vec![Line::from("2000-01-02T00:00:00Z")]
        );
    }

    fn languages_options(number_of_languages: usize) -> DisplayOptions {
        DisplayOptions {
            number_of_languages,
            ..DisplayOptions::default()
        }
    }

    #[test]
    fn test_display_languages() {
        let languages = Languages {
            languages_with_percentage: vec![LanguageWithPercentage {
                language: Language::Go,
                percentage: 100_f64,
            }],
        };
        let red = DynColors::Ansi(AnsiColors::Red);

        assert_eq!(
            languages.value(&languages_options(6)),
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
        };

        let color_palette = [
            DynColors::Ansi(AnsiColors::Red),
            DynColors::Ansi(AnsiColors::Green),
        ];

        let result = prepare_languages(&languages, &languages_options(2), &color_palette);

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
    fn should_display_dependencies() {
        let dependencies = Dependencies::new(Some(&Manifest {
            manifest_type: ManifestType::Cargo,
            name: None,
            description: None,
            number_of_dependencies: 21,
            version: None,
            license: None,
        }));

        assert_eq!(
            dependencies.value(&DisplayOptions::default()),
            vec![Line::from("21 (Cargo)")]
        );
    }

    fn john_and_roberto() -> Vec<Author> {
        vec![
            Author::new(
                "John Doe".into(),
                Some("john.doe@email.com".into()),
                1500,
                2000,
            ),
            Author::new("Roberto Berto".into(), None, 240, 300),
        ]
    }

    #[test]
    fn test_authors_title_with_one_author() {
        let mut authors = john_and_roberto();
        authors.truncate(1);
        assert_eq!(Authors { authors }.key(), "Author");
    }

    #[test]
    fn test_authors_title_with_two_authors() {
        let authors = Authors {
            authors: john_and_roberto(),
        };
        assert_eq!(authors.key(), "Authors");
    }

    #[test]
    fn test_display_two_authors() {
        let authors = Authors {
            authors: john_and_roberto(),
        };
        assert_eq!(
            authors.value(&DisplayOptions::default()),
            vec![
                Line::from("75% John Doe <john.doe@email.com> 1500"),
                Line::from("80% Roberto Berto 240"),
            ]
        );
    }

    #[test]
    fn test_display_authors_alignment_with_three_authors() {
        let mut authors = john_and_roberto();
        authors.push(Author::new("Jane Doe".into(), None, 1, 100));
        assert_eq!(
            Authors { authors }.value(&DisplayOptions::default()),
            vec![
                Line::from("75% John Doe <john.doe@email.com> 1500"),
                Line::from("80% Roberto Berto 240"),
                Line::from(" 1% Jane Doe 1"),
            ]
        );
    }

    #[rstest]
    #[case(456, 123, 0)]
    #[case(456789, 123, 3)]
    #[case(1, 12, 1)]
    fn test_digit_difference(#[case] num1: usize, #[case] num2: usize, #[case] expected: usize) {
        assert_eq!(digit_difference(num1, num2), expected);
    }

    fn authors_options(number_of_authors: usize) -> DisplayOptions {
        DisplayOptions {
            number_of_authors,
            ..DisplayOptions::default()
        }
    }

    #[test]
    fn test_display_contributors() {
        let contributors = Contributors::new(12);
        assert_eq!(
            contributors.value(&authors_options(2)),
            vec![Line::from("12")]
        );
        assert_eq!(contributors.key(), "Contributors".to_string());
    }

    #[test]
    fn test_display_contributors_less_than_authors_to_display() {
        let contributors = Contributors::new(1);

        assert!(contributors.value(&authors_options(3)).is_empty());
    }

    #[rstest]
    #[case(false, "3")]
    #[case(true, "3 (shallow)")]
    fn test_display_commits(#[case] is_shallow: bool, #[case] expected: &str) {
        let commits = Commits {
            number_of_commits: 3,
            is_shallow,
        };

        assert_eq!(
            commits.value(&DisplayOptions::default()),
            vec![Line::from(expected)]
        );
    }

    #[test]
    fn test_display_churn_with_two_file_churns() {
        let churn = Churn {
            file_churns: vec![
                FileChurn::new("path/to/file.txt".into(), 50),
                FileChurn::new("file_2.txt".into(), 30),
            ],
            churn_pool_size: 5,
        };

        assert_eq!(
            churn.value(&DisplayOptions::default()),
            vec![
                Line::from("\u{2026}/to/file.txt 50"),
                Line::from("file_2.txt 30"),
            ]
        );
    }

    #[test]
    fn test_truncate_file_path() {
        assert_eq!(shorten_file_path("path/to/file.txt", 3), "path/to/file.txt");
        assert_eq!(shorten_file_path("another/file.txt", 2), "another/file.txt");
        assert_eq!(shorten_file_path("file.txt", 1), "file.txt");
        assert_eq!(
            shorten_file_path("path/to/file.txt", 2),
            "\u{2026}/to/file.txt"
        );
        assert_eq!(
            shorten_file_path("another/file.txt", 1),
            "\u{2026}/file.txt"
        );
        assert_eq!(shorten_file_path("file.txt", 0), "file.txt");
    }

    #[test]
    fn test_display_lines_of_code() {
        let lines_of_code = LinesOfCode {
            lines_of_code: 1235,
        };

        assert_eq!(
            lines_of_code.value(&DisplayOptions::default()),
            vec![Line::from("1235")]
        );
    }

    #[rstest]
    #[case(123, "2.46 MiB (123 files)")]
    #[case(0, "2.46 MiB")]
    #[case(1, "2.46 MiB (1 file)")]
    fn test_display_size(#[case] file_count: u64, #[case] expected: &str) {
        let size = Size {
            repo_size: 2_577_152,
            file_count,
        };

        assert_eq!(
            size.value(&DisplayOptions::default()),
            vec![Line::from(expected)]
        );
    }

    #[rstest(
        case(0, "0 B"),
        case(1023, "1023 B"),
        case(1024, "1 KiB"),
        case(2048, "2 KiB"),
        case(1048576, "1 MiB"),
        case(1099511627776, "1 TiB"),
        case(2577152, "2.46 MiB")
    )]
    fn test_bytes_to_human_readable(#[case] input: u64, #[case] expected: &str) {
        assert_eq!(bytes_to_human_readable(input), expected);
    }

    #[test]
    fn test_display_license() {
        let license = License {
            license: "MIT".into(),
        };
        assert_eq!(
            license.value(&DisplayOptions::default()),
            vec![Line::from("MIT")]
        );
    }
}
