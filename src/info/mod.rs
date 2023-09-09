use self::author::AuthorsInfo;
use self::churn::ChurnInfo;
use self::commits::CommitsInfo;
use self::contributors::ContributorsInfo;
use self::created::CreatedInfo;
use self::dependencies::DependenciesInfo;
use self::description::DescriptionInfo;
use self::git::metrics::GitMetrics;
use self::git::traverse_commit_graph;
use self::head::HeadInfo;
use self::langs::language::Language;
use self::langs::language::LanguagesInfo;
use self::last_change::LastChangeInfo;
use self::license::LicenseInfo;
use self::loc::LocInfo;
use self::pending::PendingInfo;
use self::project::ProjectInfo;
use self::size::SizeInfo;
use self::title::Title;
use self::url::get_repo_url;
use self::url::UrlInfo;
use self::utils::info_field::{InfoField, InfoType};
use self::version::VersionInfo;
use crate::cli::{is_truecolor_terminal, CliOptions, NumberSeparator, When};
use crate::ui::get_ascii_colors;
use crate::ui::text_colors::TextColors;
use anyhow::{Context, Result};
use gix::sec::trust::Mapping;
use gix::Repository;
use num_format::ToFormattedString;
use onefetch_manifest::Manifest;
use owo_colors::{DynColors, OwoColorize, Style};
use serde::Serialize;
use std::path::Path;

mod author;
mod churn;
mod commits;
mod contributors;
mod created;
mod dependencies;
mod description;
mod git;
mod head;
pub mod langs;
mod last_change;
mod license;
mod loc;
mod pending;
mod project;
mod size;
mod title;
mod url;
pub mod utils;
mod version;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    title: Option<Title>,
    info_fields: Vec<Box<dyn InfoField>>,
    #[serde(skip_serializing)]
    text_colors: TextColors,
    #[serde(skip_serializing)]
    no_color_palette: bool,
    #[serde(skip_serializing)]
    no_bold: bool,
    #[serde(skip_serializing)]
    pub dominant_language: Language,
    #[serde(skip_serializing)]
    pub ascii_colors: Vec<DynColors>,
}

struct InfoBuilder {
    title: Option<Title>,
    info_fields: Vec<Box<dyn InfoField>>,
    disabled_fields: Vec<InfoType>,
    no_title: bool,
}

impl std::fmt::Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        //Title
        if let Some(title) = &self.title {
            write!(f, "{title}")?;
        }

        //Info lines
        for info_field in self.info_fields.iter() {
            let info_field_value = info_field.value();
            if !info_field_value.is_empty() {
                write_styled_info_line(
                    f,
                    &info_field.title(),
                    &info_field_value,
                    self.no_bold,
                    &self.text_colors,
                )?;
            }
        }

        //Palette
        if !self.no_color_palette {
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

pub fn build_info(cli_options: &CliOptions) -> Result<Info> {
    let mut repo = gix::ThreadSafeRepository::discover_opts(
        &cli_options.input,
        gix::discover::upwards::Options {
            dot_git_only: true,
            ..Default::default()
        },
        Mapping::default(),
    )?
    .to_thread_local();
    // Having an object cache is important for getting much better traversal and diff performance.
    repo.object_cache_size_if_unset(4 * 1024 * 1024);
    let repo_path = get_work_dir(&repo)?;

    let loc_by_language_sorted_handle = std::thread::spawn({
        let globs_to_exclude = cli_options.info.exclude.clone();
        let language_types = cli_options.info.r#type.clone();
        let include_hidden = cli_options.info.include_hidden;
        let workdir = repo_path.clone();
        move || {
            langs::get_loc_by_language_sorted(
                &workdir,
                &globs_to_exclude,
                &language_types,
                include_hidden,
            )
        }
    });

    let loc_by_language = loc_by_language_sorted_handle
        .join()
        .ok()
        .context("BUG: panic in language statistics thread")??;
    let manifest = get_manifest(&repo_path)?;
    let repo_url = get_repo_url(&repo);

    let git_metrics = traverse_commit_graph(&repo, cli_options)?;
    let true_color = match cli_options.ascii.true_color {
        When::Always => true,
        When::Never => false,
        When::Auto => is_truecolor_terminal(),
    };
    let dominant_language = langs::get_main_language(&loc_by_language);
    let ascii_colors = get_ascii_colors(
        &cli_options.ascii.ascii_language,
        &dominant_language,
        &cli_options.ascii.ascii_colors,
        true_color,
    );

    let text_colors = TextColors::new(&cli_options.text_formatting.text_colors, ascii_colors[0]);
    let no_bold = cli_options.text_formatting.no_bold;
    let number_separator = cli_options.text_formatting.number_separator;
    let iso_time = cli_options.text_formatting.iso_time;
    let number_of_languages = cli_options.info.number_of_languages;
    let number_of_authors = cli_options.info.number_of_authors;

    Ok(InfoBuilder::new(cli_options)
        .title(&repo, no_bold, &text_colors)
        .project(&repo, &repo_url, &manifest, number_separator)?
        .description(&manifest)
        .head(&repo)?
        .pending(&repo)?
        .version(&repo, &manifest)?
        .created(&git_metrics, iso_time)
        .languages(
            &loc_by_language,
            true_color,
            number_of_languages,
            &text_colors,
        )
        .dependencies(&manifest, number_separator)
        .authors(&git_metrics)
        .last_change(&git_metrics, iso_time)
        .contributors(&git_metrics, number_of_authors, number_separator)
        .url(&repo_url)
        .commits(&git_metrics, repo.is_shallow(), number_separator)
        .churn(&git_metrics)
        .loc(&loc_by_language, number_separator)
        .size(&repo, number_separator)
        .license(&repo_path, &manifest)?
        .build(cli_options, text_colors, dominant_language, ascii_colors))
}

impl InfoBuilder {
    fn new(cli_options: &CliOptions) -> Self {
        Self {
            title: None,
            info_fields: Vec::new(),
            disabled_fields: cli_options.info.disabled_fields.clone(),
            no_title: cli_options.info.no_title,
        }
    }

    fn title(mut self, repo: &Repository, no_bold: bool, text_colors: &TextColors) -> Self {
        if !self.no_title {
            let title = Title::new(
                repo,
                text_colors.title,
                text_colors.tilde,
                text_colors.underline,
                !no_bold,
            );
            self.title = Some(title);
        }
        self
    }

    fn description(mut self, manifest: &Option<Manifest>) -> Self {
        if !self.disabled_fields.contains(&InfoType::Description) {
            let description = DescriptionInfo::new(manifest.as_ref());
            self.info_fields.push(Box::new(description));
        }
        self
    }

    fn pending(mut self, repo: &Repository) -> Result<Self> {
        if !self.disabled_fields.contains(&InfoType::Pending) {
            let pending = PendingInfo::new(repo)?;
            self.info_fields.push(Box::new(pending));
        }
        Ok(self)
    }

    fn url(mut self, repo_url: &str) -> Self {
        if !self.disabled_fields.contains(&InfoType::URL) {
            let repo_url = UrlInfo::new(repo_url);
            self.info_fields.push(Box::new(repo_url));
        }
        self
    }

    fn project(
        mut self,
        repo: &Repository,
        repo_url: &str,
        manifest: &Option<Manifest>,
        number_separator: NumberSeparator,
    ) -> Result<Self> {
        if !self.disabled_fields.contains(&InfoType::Project) {
            let project = ProjectInfo::new(repo, repo_url, manifest.as_ref(), number_separator)?;
            self.info_fields.push(Box::new(project));
        }
        Ok(self)
    }

    fn head(mut self, repo: &Repository) -> Result<Self> {
        if !self.disabled_fields.contains(&InfoType::Head) {
            let head = HeadInfo::new(repo)?;
            self.info_fields.push(Box::new(head));
        }
        Ok(self)
    }

    fn version(mut self, repo: &Repository, manifest: &Option<Manifest>) -> Result<Self> {
        if !self.disabled_fields.contains(&InfoType::Version) {
            let version = VersionInfo::new(repo, manifest.as_ref())?;
            self.info_fields.push(Box::new(version));
        }
        Ok(self)
    }

    fn size(mut self, repo: &Repository, number_separator: NumberSeparator) -> Self {
        if !self.disabled_fields.contains(&InfoType::Size) {
            let size = SizeInfo::new(repo, number_separator);
            self.info_fields.push(Box::new(size));
        }
        self
    }

    fn license(mut self, repo_path: &Path, manifest: &Option<Manifest>) -> Result<Self> {
        if !self.disabled_fields.contains(&InfoType::License) {
            let license = LicenseInfo::new(repo_path, manifest.as_ref())?;
            self.info_fields.push(Box::new(license));
        }
        Ok(self)
    }

    fn created(mut self, git_metrics: &GitMetrics, iso_time: bool) -> Self {
        if !self.disabled_fields.contains(&InfoType::Created) {
            let created = CreatedInfo::new(iso_time, git_metrics);
            self.info_fields.push(Box::new(created));
        }
        self
    }

    fn languages(
        mut self,
        loc_by_language: &[(Language, usize)],
        true_color: bool,
        number_of_languages: usize,
        text_colors: &TextColors,
    ) -> Self {
        if !self.disabled_fields.contains(&InfoType::Languages) {
            let languages = LanguagesInfo::new(
                loc_by_language,
                true_color,
                number_of_languages,
                text_colors.info,
            );
            self.info_fields.push(Box::new(languages));
        }
        self
    }

    fn dependencies(
        mut self,
        manifest: &Option<Manifest>,
        number_separator: NumberSeparator,
    ) -> Self {
        if !self.disabled_fields.contains(&InfoType::Dependencies) {
            let dependencies = DependenciesInfo::new(manifest.as_ref(), number_separator);
            self.info_fields.push(Box::new(dependencies));
        }
        self
    }

    fn authors(mut self, git_metrics: &GitMetrics) -> Self {
        if !self.disabled_fields.contains(&InfoType::Authors) {
            let authors = AuthorsInfo::new(git_metrics);
            self.info_fields.push(Box::new(authors));
        }
        self
    }

    fn last_change(mut self, git_metrics: &GitMetrics, iso_time: bool) -> Self {
        if !self.disabled_fields.contains(&InfoType::LastChange) {
            let last_change = LastChangeInfo::new(iso_time, git_metrics);
            self.info_fields.push(Box::new(last_change));
        }
        self
    }

    fn contributors(
        mut self,
        git_metrics: &GitMetrics,
        number_of_authors: usize,
        number_separator: NumberSeparator,
    ) -> Self {
        if !self.disabled_fields.contains(&InfoType::Contributors) {
            let contributors =
                ContributorsInfo::new(git_metrics, number_of_authors, number_separator);
            self.info_fields.push(Box::new(contributors));
        }
        self
    }

    fn commits(
        mut self,
        git_metrics: &GitMetrics,
        is_shallow: bool,
        number_separator: NumberSeparator,
    ) -> Self {
        if !self.disabled_fields.contains(&InfoType::Commits) {
            let commits = CommitsInfo::new(git_metrics, is_shallow, number_separator);
            self.info_fields.push(Box::new(commits));
        }
        self
    }

    fn churn(mut self, git_metrics: &GitMetrics) -> Self {
        if !self.disabled_fields.contains(&InfoType::Churn) {
            let churn = ChurnInfo::new(git_metrics);
            self.info_fields.push(Box::new(churn));
        }
        self
    }

    fn loc(
        mut self,
        loc_by_language: &[(Language, usize)],
        number_separator: NumberSeparator,
    ) -> Self {
        if !self.disabled_fields.contains(&InfoType::LinesOfCode) {
            let lines_of_code = LocInfo::new(loc_by_language, number_separator);
            self.info_fields.push(Box::new(lines_of_code));
        }
        self
    }

    fn build(
        self,
        cli_options: &CliOptions,
        text_colors: TextColors,
        dominant_language: Language,
        ascii_colors: Vec<DynColors>,
    ) -> Info {
        Info {
            title: self.title,
            info_fields: self.info_fields,
            text_colors,
            dominant_language,
            ascii_colors,
            no_color_palette: cli_options.visuals.no_color_palette,
            no_bold: cli_options.text_formatting.no_bold,
        }
    }
}

fn write_styled_info_line(
    f: &mut std::fmt::Formatter,
    subtitle: &str,
    info: &str,
    no_bold: bool,
    text_colors: &TextColors,
) -> std::fmt::Result {
    writeln!(
        f,
        "{} {}",
        style_subtitle(subtitle, text_colors, no_bold),
        style_info(info, text_colors)
    )
}

fn style_info(info: &str, text_colors: &TextColors) -> String {
    let info_lines: Vec<&str> = info.lines().collect();
    let info_style = get_style(false, text_colors.info);

    let styled_lines: Vec<String> = info_lines
        .iter()
        .map(|line| format!("{}", line.style(info_style)))
        .collect();

    styled_lines.join("\n")
}

fn style_subtitle(subtitle: &str, text_colors: &TextColors, no_bold: bool) -> String {
    let subtitle_style = get_style(!no_bold, text_colors.subtitle);
    let colon_style = get_style(!no_bold, text_colors.colon);
    format!(
        "{}{}",
        subtitle.style(subtitle_style),
        ":".style(colon_style)
    )
}

fn get_style(is_bold: bool, color: DynColors) -> Style {
    let mut style = Style::new().color(color);
    if is_bold {
        style = style.bold();
    }
    style
}

fn get_manifest(repo_path: &Path) -> Result<Option<Manifest>> {
    let manifests = onefetch_manifest::get_manifests(repo_path)?;

    if manifests.is_empty() {
        Ok(None)
    } else {
        Ok(manifests.first().cloned())
    }
}

pub fn get_work_dir(repo: &gix::Repository) -> Result<std::path::PathBuf> {
    Ok(repo
        .work_dir()
        .context("please run onefetch inside of a non-bare git repository")?
        .to_owned())
}

fn format_number<T: ToFormattedString + std::fmt::Display>(
    number: &T,
    number_separator: NumberSeparator,
) -> String {
    number.to_formatted_string(&number_separator.get_format())
}

#[cfg(test)]
mod tests {
    use super::*;
    use owo_colors::AnsiColors;

    #[test]
    fn test_get_style() {
        let style = get_style(true, DynColors::Ansi(AnsiColors::Cyan));
        assert_eq!(
            style,
            Style::new().color(DynColors::Ansi(AnsiColors::Cyan)).bold()
        );
    }

    #[test]
    fn test_get_style_no_bold() {
        let style = get_style(false, DynColors::Ansi(AnsiColors::Cyan));
        assert_eq!(style, Style::new().color(DynColors::Ansi(AnsiColors::Cyan)));
    }

    #[test]
    fn test_format_number() {
        assert_eq!(
            &format_number(&1_000_000, NumberSeparator::Comma),
            "1,000,000"
        );
        assert_eq!(
            &format_number(&1_000_000, NumberSeparator::Space),
            "1\u{202f}000\u{202f}000"
        );
        assert_eq!(
            &format_number(&1_000_000, NumberSeparator::Underscore),
            "1_000_000"
        );
        assert_eq!(
            &format_number(&1_000_000, NumberSeparator::Plain),
            "1000000"
        );
    }

    #[test]
    fn test_info_style_info() {
        let text_colors = TextColors::new(&[0, 0, 0, 0, 0, 0], DynColors::Ansi(AnsiColors::Blue));

        let info_text = style_info("foo", &text_colors);
        // Rendered text: black `foo`
        assert_eq!(info_text, "\u{1b}[30mfoo\u{1b}[0m");
    }

    #[test]
    fn test_info_style_subtitle() {
        let text_colors = TextColors::new(&[0, 0, 0, 0, 15, 0], DynColors::Ansi(AnsiColors::Blue));

        let subtitle_text = style_subtitle("foo", &text_colors, false);
        assert_eq!(
            subtitle_text,
            // Rendered text: black `foo` and bright white colon
            "\u{1b}[30;1mfoo\u{1b}[0m\u{1b}[97;1m:\u{1b}[0m"
        );
    }
}
