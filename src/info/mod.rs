use crate::cli::{self, Config};
use crate::ui::get_ascii_colors;
use crate::ui::text_colors::TextColors;
use anyhow::Result;
use author::Author;
use deps::DependencyDetector;
use git2::Repository;
use head_refs::HeadRefs;
use langs::language::Language;
use license::Detector;
use owo_colors::{AnsiColors, DynColors, OwoColorize, Style};
use repo::Repo;
use serde::ser::SerializeStruct;
use serde::Serialize;

mod author;
pub mod deps;
mod head_refs;
pub mod info_field;
pub mod langs;
mod license;
pub mod repo;

pub struct Info {
    git_username: String,
    git_version: String,
    repo_name: String,
    number_of_tags: usize,
    number_of_branches: usize,
    head_refs: HeadRefs,
    pending_changes: String,
    version: String,
    creation_date: String,
    languages: Vec<(Language, f64)>,
    dependencies: String,
    authors: Vec<Author>,
    last_change: String,
    contributors: usize,
    repo_url: String,
    number_of_commits: String,
    lines_of_code: usize,
    file_count: u64,
    repo_size: String,
    license: String,
    text_colors: TextColors,
    pub dominant_language: Language,
    pub ascii_colors: Vec<DynColors>,
    pub config: Config,
}

impl std::fmt::Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if !self.config.disabled_fields.git_info
            && (!&self.git_username.is_empty() || !&self.git_version.is_empty())
        {
            let (git_info_field_str, git_info_field_len) = self.get_git_info_field();
            writeln!(f, "{}", git_info_field_str)?;
            let separator = "-".repeat(git_info_field_len);
            writeln!(f, "{}", separator.color(self.text_colors.underline))?;
        }

        if !self.config.disabled_fields.project && !self.repo_name.is_empty() {
            let branches_tags_str = self.get_branches_and_tags_field();
            let project_str = format!("{} {}", &self.repo_name, branches_tags_str);
            self.write_styled_info_line("Project", &project_str, f)?;
        }

        if !self.config.disabled_fields.head {
            self.write_styled_info_line("HEAD", &self.head_refs.to_string(), f)?;
        }

        if !self.config.disabled_fields.pending && !self.pending_changes.is_empty() {
            self.write_styled_info_line("Pending", &self.pending_changes, f)?;
        }

        if !self.config.disabled_fields.version && !self.version.is_empty() {
            self.write_styled_info_line("Version", &self.version, f)?;
        }

        if !self.config.disabled_fields.created && !self.creation_date.is_empty() {
            self.write_styled_info_line("Created", &self.creation_date, f)?;
        }

        if !self.config.disabled_fields.languages && !self.languages.is_empty() {
            let title = if self.languages.len() > 1 {
                "Languages"
            } else {
                "Language"
            };
            let languages_str = self.get_language_field(title);
            self.write_info_line(title, &languages_str, f)?;
        }

        if !self.config.disabled_fields.dependencies && !self.dependencies.is_empty() {
            self.write_styled_info_line("Dependencies", &self.dependencies, f)?;
        }

        if !self.config.disabled_fields.authors && !self.authors.is_empty() {
            let title = if self.authors.len() > 1 {
                "Authors"
            } else {
                "Author"
            };
            let author_str = self.get_author_field(title);
            self.write_info_line(title, &author_str, f)?;
        }

        if !self.config.disabled_fields.last_change && !self.last_change.is_empty() {
            self.write_styled_info_line("Last change", &self.last_change, f)?;
        }

        if !self.config.disabled_fields.contributors
            && self.contributors > self.config.number_of_authors
        {
            self.write_styled_info_line("Contributors", &self.contributors.to_string(), f)?;
        }

        if !self.config.disabled_fields.repo && !self.repo_url.is_empty() {
            self.write_styled_info_line("Repo", &self.repo_url, f)?;
        }

        if !self.config.disabled_fields.commits {
            self.write_styled_info_line("Commits", &self.number_of_commits, f)?;
        }

        if !self.config.disabled_fields.lines_of_code {
            self.write_styled_info_line("Lines of code", &self.lines_of_code.to_string(), f)?;
        }

        if !self.config.disabled_fields.size && !self.repo_size.is_empty() {
            let repo_size_str = self.get_repo_size_field();
            self.write_styled_info_line("Size", &repo_size_str, f)?;
        }

        if !self.config.disabled_fields.license && !self.license.is_empty() {
            self.write_styled_info_line("License", &self.license, f)?;
        }

        if !self.config.no_color_palette {
            writeln!(
                f,
                "\n{0}{1}{2}{3}{4}{5}{6}{7}",
                "   ".on_black(),
                "   ".on_red(),
                "   ".on_green(),
                "   ".on_yellow(),
                "   ".on_blue(),
                "   ".on_magenta(),
                "   ".on_cyan(),
                "   ".on_white()
            )?;
        }

        Ok(())
    }
}

impl Info {
    pub fn new(config: Config) -> Result<Self> {
        let git_version = cli::get_git_version();
        let repo = Repository::discover(&config.repo_path)?;
        let internal_repo = Repo::new(&repo, config.no_merges, &config.bot_regex_pattern)?;
        let (repo_name, repo_url) = internal_repo.get_name_and_url()?;
        let head_refs = internal_repo.get_head_refs()?;
        let pending_changes = internal_repo.get_pending_changes()?;
        let version = internal_repo.get_version()?;
        let git_username = internal_repo.get_git_username()?;
        let number_of_tags = internal_repo.get_number_of_tags()?;
        let number_of_branches = internal_repo.get_number_of_branches()?;
        let creation_date = internal_repo.get_creation_date(config.iso_time)?;
        let number_of_commits = internal_repo.get_number_of_commits();
        let (authors, contributors) =
            internal_repo.get_authors(config.number_of_authors, config.show_email)?;
        let last_change = internal_repo.get_date_of_last_commit(config.iso_time)?;
        let (repo_size, file_count) = internal_repo.get_repo_size();
        let workdir = internal_repo.get_work_dir()?;
        let license = Detector::new()?.get_license(&workdir)?;
        let dependencies = DependencyDetector::new().get_dependencies(&workdir)?;
        let (languages, lines_of_code) = langs::get_language_statistics(
            &workdir,
            &config.ignored_directories,
            &config.language_types,
            config.include_hidden,
        )?;
        let dominant_language = langs::get_dominant_language(&languages);
        let ascii_colors = get_ascii_colors(
            &config.ascii_language,
            &dominant_language,
            &config.ascii_colors,
            config.true_color,
        );
        let text_colors = TextColors::new(&config.text_colors, ascii_colors[0]);

        Ok(Self {
            git_username,
            git_version,
            repo_name,
            number_of_tags,
            number_of_branches,
            head_refs,
            pending_changes,
            version,
            creation_date,
            languages,
            dependencies,
            authors,
            last_change,
            contributors,
            repo_url,
            number_of_commits,
            lines_of_code,
            file_count,
            repo_size,
            license,
            dominant_language,
            ascii_colors,
            text_colors,
            config,
        })
    }

    fn write_styled_info_line(
        &self,
        subtitle: &str,
        info: &str,
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        let colored_info = info.color(self.text_colors.info);
        self.write_info_line(subtitle, &colored_info.to_string(), f)
    }

    fn write_info_line(
        &self,
        subtitle: &str,
        info: &str,
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        writeln!(f, "{} {}", &self.style_subtitle(subtitle), info)
    }

    fn style_subtitle(&self, subtitle: &str) -> String {
        let subtitle_style = self.style(self.text_colors.subtitle);
        let colon_style = self.style(self.text_colors.colon);
        format!(
            "{}{}",
            subtitle.style(subtitle_style),
            ":".style(colon_style)
        )
    }

    fn style(&self, color: DynColors) -> Style {
        let mut style = Style::new().color(color);
        if !self.config.no_bold {
            style = style.bold();
        }
        style
    }

    fn get_git_info_field(&self) -> (String, usize) {
        let git_info_length = self.git_username.len() + self.git_version.len();
        let title_style = self.style(self.text_colors.title);

        if !&self.git_username.is_empty() && !&self.git_version.is_empty() {
            let tilde_style = self.style(self.text_colors.tilde);
            (
                format!(
                    "{} {} {}",
                    &self.git_username.style(title_style),
                    "~".style(tilde_style),
                    &self.git_version.style(title_style)
                ),
                git_info_length + 3,
            )
        } else {
            (
                format!(
                    "{}{}",
                    &self.git_username.style(title_style),
                    &self.git_version.style(title_style)
                ),
                git_info_length,
            )
        }
    }

    fn get_author_field(&self, title: &str) -> String {
        let mut author_field = String::from("");

        let pad = title.len() + 2;

        for (i, author) in self.authors.iter().enumerate() {
            let author_str = author.color(self.text_colors.info);

            if i == 0 {
                author_field.push_str(&format!("{}", author_str));
            } else {
                author_field.push_str(&format!("\n{:<width$}{}", "", author_str, width = pad));
            }
        }

        author_field
    }

    fn get_language_field(&self, title: &str) -> String {
        let mut language_field = String::from("");
        let language_bar_length = 26;
        let pad = title.len() + 2;
        let color_palette = vec![
            DynColors::Ansi(AnsiColors::Red),
            DynColors::Ansi(AnsiColors::Green),
            DynColors::Ansi(AnsiColors::Yellow),
            DynColors::Ansi(AnsiColors::Blue),
            DynColors::Ansi(AnsiColors::Magenta),
            DynColors::Ansi(AnsiColors::Cyan),
        ];

        let languages: Vec<(String, f64, DynColors)> = {
            let mut iter = self
                .languages
                .iter()
                .enumerate()
                .map(|(i, (language, perc))| {
                    let circle_color = if self.config.true_color {
                        language.get_circle_color()
                    } else {
                        color_palette[i % color_palette.len()]
                    };
                    (language.to_string(), *perc, circle_color)
                });
            if self.languages.len() > 6 {
                let mut languages = iter.by_ref().take(6).collect::<Vec<_>>();
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
                    (perc / 100. * language_bar_length as f64).round() as usize,
                    1,
                );
                format!("{:<width$}", "".on_color(*circle_color), width = bar_width)
            })
            .collect();

        language_field.push_str(&language_bar);

        for (i, (language, perc, circle_color)) in languages.iter().enumerate() {
            let formatted_number = format!("{:.*}", 1, perc);
            let circle = "\u{25CF}".color(*circle_color);
            let language_str = format!(
                "{} {} ",
                circle,
                format!("{} ({} %)", language, formatted_number).color(self.text_colors.info)
            );
            if i % 2 == 0 {
                language_field.push_str(&format!("\n{:<width$}{}", "", language_str, width = pad));
            } else {
                language_field.push_str(&language_str.to_string());
            }
        }
        language_field
    }

    fn get_branches_and_tags_field(&self) -> String {
        let branches_str = match self.number_of_branches {
            0 => String::new(),
            1 => String::from("1 branch"),
            _ => format!("{} branches", self.number_of_branches),
        };

        let tags_str = match self.number_of_tags {
            0 => String::new(),
            1 => String::from("1 tag"),
            _ => format!("{} tags", self.number_of_tags),
        };

        if tags_str.is_empty() && branches_str.is_empty() {
            String::new()
        } else if branches_str.is_empty() || tags_str.is_empty() {
            format!("({}{})", tags_str, branches_str)
        } else {
            format!("({}, {})", branches_str, tags_str)
        }
    }

    fn get_repo_size_field(&self) -> String {
        match self.file_count {
            0 => String::from(&self.repo_size),
            _ => {
                let res = format!("{} ({} files)", self.repo_size, self.file_count);
                res
            }
        }
    }
}

impl Serialize for Info {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Info", 15)?;
        let langs: Vec<String> = self
            .languages
            .iter()
            .map(|(l, _)| format!("{}", l))
            .collect();
        state.serialize_field("repoName", &self.repo_name)?;
        state.serialize_field("numberOfTags", &self.number_of_tags)?;
        state.serialize_field("numberOfBranches", &self.number_of_branches)?;
        state.serialize_field("headRefs", &self.head_refs)?;
        state.serialize_field("version", &self.version)?;
        state.serialize_field("creationDate", &self.creation_date)?;
        state.serialize_field("languages", &langs)?;
        state.serialize_field("authors", &self.authors)?;
        state.serialize_field("lastChange", &self.last_change)?;
        state.serialize_field("repoUrl", &self.repo_url)?;
        state.serialize_field("numberOfCommits", &self.number_of_commits)?;
        state.serialize_field("linesOfCode", &self.lines_of_code)?;
        state.serialize_field("repoSize", &self.repo_size)?;
        state.serialize_field("filesCount", &self.file_count)?;
        state.serialize_field("license", &self.license)?;

        state.end()
    }
}
