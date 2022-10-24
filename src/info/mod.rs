use self::deps::DependenciesInfo;
use self::git::Commits;
use self::info_field::{InfoField, InfoType};
use self::langs::language::Language;
use self::langs::language::LanguagesInfo;
use self::repo::author::AuthorsInfo;
use self::repo::commits::CommitsInfo;
use self::repo::contributors::ContributorsInfo;
use self::repo::created::CreatedInfo;
use self::repo::head::HeadInfo;
use self::repo::last_change::LastChangeInfo;
use self::repo::license::LicenseInfo;
use self::repo::loc::LocInfo;
use self::repo::pending::PendingInfo;
use self::repo::project::ProjectInfo;
use self::repo::size::SizeInfo;
use self::repo::url::UrlInfo;
use self::repo::version::VersionInfo;
use self::title::Title;
use crate::cli::{is_truecolor_terminal, Config, MyRegex, When};
use crate::ui::get_ascii_colors;
use crate::ui::text_colors::TextColors;
use anyhow::{Context, Result};
use owo_colors::{DynColors, OwoColorize, Style};
use regex::Regex;
use serde::ser::SerializeStruct;
use serde::Serialize;
use std::str::FromStr;

pub mod deps;
mod git;
pub mod info_field;
pub mod langs;
mod repo;
mod title;

pub struct Info {
    title: Title,
    project: ProjectInfo,
    head: HeadInfo,
    pending: PendingInfo,
    version: VersionInfo,
    created: CreatedInfo,
    languages: LanguagesInfo,
    dependencies: DependenciesInfo,
    authors: AuthorsInfo,
    last_change: LastChangeInfo,
    contributors: ContributorsInfo,
    repo: UrlInfo,
    commits: CommitsInfo,
    lines_of_code: LocInfo,
    size: SizeInfo,
    license: LicenseInfo,
    disabled_fields: Vec<InfoType>,
    text_colors: TextColors,
    no_color_palette: bool,
    no_bold: bool,
    pub dominant_language: Language,
    pub ascii_colors: Vec<DynColors>,
}

impl std::fmt::Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        //Title
        if !self.disabled_fields.contains(&InfoType::Title) {
            write!(f, "{}", self.title)?;
        }

        //Info lines
        if let Some(project_info_value) = self.project.get(&self.disabled_fields) {
            self.write_styled_info_line(&self.project.title(), &project_info_value, f)?;
        }

        if let Some(head_info_value) = self.head.get(&self.disabled_fields) {
            self.write_styled_info_line(&self.head.title(), &head_info_value, f)?;
        }

        if let Some(pending_info_value) = self.pending.get(&self.disabled_fields) {
            self.write_styled_info_line(&self.pending.title(), &pending_info_value, f)?;
        }

        if let Some(version_info_value) = self.version.get(&self.disabled_fields) {
            self.write_styled_info_line(&self.version.title(), &version_info_value, f)?;
        }

        if let Some(created_info_value) = self.created.get(&self.disabled_fields) {
            self.write_styled_info_line(&self.created.title(), &created_info_value, f)?;
        }

        if let Some(languages_info_value) = self.languages.get(&self.disabled_fields) {
            self.write_styled_info_line(&self.languages.title(), &languages_info_value, f)?;
        }

        if let Some(dependencies_info_value) = self.dependencies.get(&self.disabled_fields) {
            self.write_styled_info_line(&self.dependencies.title(), &dependencies_info_value, f)?;
        }

        if let Some(authors_info_value) = self.authors.get(&self.disabled_fields) {
            self.write_styled_info_line(&self.authors.title(), &authors_info_value, f)?;
        }

        if let Some(last_change_info_value) = self.last_change.get(&self.disabled_fields) {
            self.write_styled_info_line(&self.last_change.title(), &last_change_info_value, f)?;
        }

        if let Some(contributors_info_value) = self.contributors.get(&self.disabled_fields) {
            self.write_styled_info_line(&self.contributors.title(), &contributors_info_value, f)?;
        }

        if let Some(repo_info_value) = self.repo.get(&self.disabled_fields) {
            self.write_styled_info_line(&self.repo.title(), &repo_info_value, f)?;
        }

        if let Some(commits_info_value) = self.commits.get(&self.disabled_fields) {
            self.write_styled_info_line(&self.commits.title(), &commits_info_value, f)?;
        }

        if let Some(lines_of_code_info_value) = self.lines_of_code.get(&self.disabled_fields) {
            self.write_styled_info_line(&self.lines_of_code.title(), &lines_of_code_info_value, f)?;
        }

        if let Some(size_info_value) = self.size.get(&self.disabled_fields) {
            self.write_styled_info_line(&self.size.title(), &size_info_value, f)?;
        }

        if let Some(license_info_value) = self.license.get(&self.disabled_fields) {
            self.write_styled_info_line(&self.license.title(), &license_info_value, f)?;
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

impl Info {
    pub fn init_repo_path(repo: &git_repository::Repository) -> Result<std::path::PathBuf> {
        Ok(repo
            .work_dir()
            .context("please run onefetch inside of a non-bare git repository")?
            .to_owned())
    }

    pub fn new(config: &Config) -> Result<Self> {
        let git_repo = git_repository::discover(&config.input)?;
        let repo_path = Info::init_repo_path(&git_repo)?;

        let languages_handle = std::thread::spawn({
            let ignored_directories = config.exclude.clone();
            let language_types = config.r#type.clone();
            let include_hidden = config.include_hidden;
            let workdir = repo_path.clone();
            move || {
                langs::get_language_statistics(
                    &workdir,
                    &ignored_directories,
                    &language_types,
                    include_hidden,
                )
            }
        });

        let no_bots = if let Some(r) = config.no_bots.clone() {
            match r {
                Some(p) => Some(p),
                None => Some(MyRegex(Regex::from_str(r"(b|B)ot")?)),
            }
        } else {
            None
        };
        let (languages, lines_of_code) = languages_handle
            .join()
            .ok()
            .context("BUG: panic in language statistics thread")??;
        let dominant_language = langs::get_dominant_language(&languages);
        let true_color = match config.true_color {
            When::Always => true,
            When::Never => false,
            When::Auto => is_truecolor_terminal(),
        };
        let ascii_colors = get_ascii_colors(
            &config.ascii_language,
            &dominant_language,
            &config.ascii_colors,
            true_color,
        );

        let text_colors = TextColors::new(&config.text_colors, ascii_colors[0]);
        let title = Title::new(
            &git_repo,
            text_colors.title,
            text_colors.tilde,
            text_colors.underline,
            !config.no_bold,
        );
        let pending = PendingInfo::new(&git_repo)?;
        let repo_url = UrlInfo::new(&git_repo)?;
        let project = ProjectInfo::new(&git_repo, &repo_url.repo_url)?;
        let head = HeadInfo::new(&git_repo)?;
        let version = VersionInfo::new(&git_repo)?;
        let size = SizeInfo::new(&git_repo);
        let license = LicenseInfo::new(&repo_path)?;
        let mut commits = Commits::new(
            git_repo,
            config.no_merges,
            &no_bots,
            config.number_of_authors,
            config.email,
        )?;

        let created = CreatedInfo::new(config.iso_time, &commits);
        let languages = LanguagesInfo::new(languages, true_color, text_colors.info);
        let dependencies = DependenciesInfo::new(&repo_path)?;
        let authors = AuthorsInfo::new(text_colors.info, &mut commits);
        let last_change = LastChangeInfo::new(config.iso_time, &commits);
        let contributors = ContributorsInfo::new(&commits, config.number_of_authors);
        let commits = CommitsInfo::new(&commits);
        let lines_of_code = LocInfo { lines_of_code };

        Ok(Self {
            title,
            project,
            head,
            pending,
            version,
            created,
            languages,
            dependencies,
            authors,
            last_change,
            contributors,
            repo: repo_url,
            commits,
            lines_of_code,
            size,
            license,
            disabled_fields: config.disabled_fields.clone(),
            text_colors,
            dominant_language,
            ascii_colors,
            no_color_palette: config.no_color_palette,
            no_bold: config.no_bold,
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
        let subtitle_style = get_style(!self.no_bold, self.text_colors.subtitle);
        let colon_style = get_style(!self.no_bold, self.text_colors.colon);
        format!(
            "{}{}",
            subtitle.style(subtitle_style),
            ":".style(colon_style)
        )
    }
}

pub fn get_style(is_bold: bool, color: DynColors) -> Style {
    let mut style = Style::new().color(color);
    if is_bold {
        style = style.bold();
    }
    style
}

impl Serialize for Info {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Info", 4)?;
        state.serialize_field("gitUsername", &self.title.git_username)?;
        state.serialize_field("gitVersion", &self.title.git_version)?;
        state.serialize_field("project", &self.project)?;
        state.serialize_field("head", &self.head.head_refs)?;
        state.serialize_field("version", &self.version.version)?;
        state.serialize_field("created", &self.created.creation_date)?;
        state.serialize_field("languages", &self.languages.languages_with_percentage)?;
        state.serialize_field("dependencies", &self.dependencies.dependencies)?;
        state.serialize_field("authors", &self.authors.authors)?;
        state.serialize_field("lastChange", &self.last_change.last_change)?;
        state.serialize_field("contributors", &self.contributors.number_of_contributors)?;
        state.serialize_field("repoUrl", &self.repo.repo_url)?;
        state.serialize_field("numberOfCommits", &self.commits.number_of_commits)?;
        state.serialize_field("linesOfCode", &self.lines_of_code.lines_of_code)?;
        state.serialize_field("repoSize", &self.size.repo_size)?;
        state.serialize_field("license", &self.license.license)?;

        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::num_to_color;
    use clap::Parser;
    use git_repository::{open, Repository, ThreadSafeRepository};
    use git_testtools;
    use owo_colors::AnsiColors;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_get_style() -> Result<()> {
        let style = get_style(true, DynColors::Ansi(AnsiColors::Cyan));
        assert_eq!(
            style,
            Style::new().color(DynColors::Ansi(AnsiColors::Cyan)).bold()
        );
        Ok(())
    }

    #[test]
    fn test_get_style_no_bold() -> Result<()> {
        let style = get_style(false, DynColors::Ansi(AnsiColors::Cyan));
        assert_eq!(style, Style::new().color(DynColors::Ansi(AnsiColors::Cyan)));
        Ok(())
    }

    #[test]
    fn test_style_subtitle() -> Result<()> {
        let info = Info::new(&Config::parse_from(&[
            "onefetch",
            "--text-colors",
            "0",
            "0",
            "0",
            "3",
            "4",
        ]))?;
        let styled_subtitle = info.style_subtitle("test");
        assert_eq!(
            styled_subtitle,
            format!(
                "{}{}",
                "test".style(Style::new().color(num_to_color(&3)).bold()),
                ":".style(Style::new().color(num_to_color(&4)).bold())
            )
        );
        Ok(())
    }

    type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

    fn test_repo(name: &str) -> Result<Repository> {
        let repo_path = git_testtools::scripted_fixture_repo_read_only(name)?;
        let safe_repo = ThreadSafeRepository::open_opts(repo_path, open::Options::isolated())?;
        Ok(safe_repo.to_thread_local())
    }

    #[test]
    fn test_bare_repo() -> Result {
        let repo = test_repo(&"bare_repo.sh")?;
        let res = Info::init_repo_path(&repo);
        assert!(res.is_err(), "oops, info was returned on a bare git repo");
        assert_eq!(
            res.unwrap_err().to_string(),
            "please run onefetch inside of a non-bare git repository"
        );
        Ok(())
    }

    #[test]
    fn test_language_repo() -> Result {
        let repo_path = git_testtools::scripted_fixture_repo_read_only_with_args(
            "language_repo.sh",
            ["verilog"],
        )
        .unwrap();
        let safe_repo =
            ThreadSafeRepository::open_opts(repo_path.join("verilog"), open::Options::isolated())?;
        let repo = safe_repo.to_thread_local();
        let mut config = Config::parse_from(&["."]);
        config.input = repo.path().to_path_buf();
        let info = Info::new(&config)?;
        let info_str = format!("{}", info);
        let info_u8 = strip_ansi_escapes::strip(&info_str)?;
        let simple_info_str = std::str::from_utf8(&info_u8)?;
        let expected_regex = include_str!("../../tests/regex/test_verilog_repo.stdout.regex");
        let re = Regex::new(&expected_regex).unwrap();
        assert!(
            re.is_match(&simple_info_str),
            "OOPS, REGEX\n{}\nDOESNT MATCH\n{}",
            expected_regex,
            simple_info_str
        );

        let mut v = serde_json::to_value(info).unwrap();
        let expected_json: serde_json::Value = serde_json::from_str(&include_str!(
            "../../tests/json/test_verilog_repo.serialize.json"
        ))
        .unwrap();
        v["gitVersion"] = serde_json::Value::String("git version".to_string());
        v["head"]["short_commit_id"] = serde_json::Value::String("short commit id".to_string());
        assert_eq!(v, expected_json);

        Ok(())
    }
}
