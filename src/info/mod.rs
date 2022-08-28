use self::author::AuthorsInfo;
use self::deps::DependenciesInfo;
use self::deps::DependencyDetector;
use self::git::Commits;
use self::git::Repo;
use self::info_field::{InfoField, InfoFieldValueGetter, InfoType};
use self::langs::language::Language;
use self::langs::language::LanguagesInfo;
use self::license::Detector;
use self::license::LicenseInfo;
use self::project::ProjectInfo;
use self::repo::{
    CommitsInfo, ContributorsInfo, CreatedInfo, HeadInfo, LastChangeInfo, LocInfo, PendingInfo,
    SizeInfo, UrlInfo, VersionInfo,
};
use self::title::Title;
use crate::cli::{self, is_truecolor_terminal, Config, MyRegex, When};
use crate::ui::get_ascii_colors;
use crate::ui::text_colors::TextColors;
use anyhow::{Context, Result};
use owo_colors::{DynColors, OwoColorize, Style};
use regex::Regex;
use serde::ser::SerializeStruct;
use serde::Serialize;
use std::str::FromStr;

mod author;
pub mod deps;
mod git;
mod head;
pub mod info_field;
pub mod langs;
mod license;
mod project;
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
            self.write_styled_info_line(&self.project.title(), &project_info_value.value, f)?;
        }

        if let Some(head_info_value) = self.project.get(&self.disabled_fields) {
            self.write_styled_info_line(&self.head.title(), &head_info_value.value, f)?;
        }

        if let Some(pending_info_value) = self.pending.get(&self.disabled_fields) {
            self.write_styled_info_line(&self.pending.title(), &pending_info_value.value, f)?;
        }

        if let Some(version_info_value) = self.version.get(&self.disabled_fields) {
            self.write_styled_info_line(&self.version.title(), &version_info_value.value, f)?;
        }

        if let Some(created_info_value) = self.created.get(&self.disabled_fields) {
            self.write_styled_info_line(&self.created.title(), &created_info_value.value, f)?;
        }

        if let Some(languages_info_value) = self.languages.get(&self.disabled_fields) {
            self.write_styled_info_line(&self.languages.title(), &languages_info_value.value, f)?;
        }

        if let Some(dependencies_info_value) = self.dependencies.get(&self.disabled_fields) {
            self.write_styled_info_line(
                &self.dependencies.title(),
                &dependencies_info_value.value,
                f,
            )?;
        }

        if let Some(authors_info_value) = self.authors.get(&self.disabled_fields) {
            self.write_styled_info_line(&self.authors.title(), &authors_info_value.value, f)?;
        }

        if let Some(last_change_info_value) = self.last_change.get(&self.disabled_fields) {
            self.write_styled_info_line(
                &self.last_change.title(),
                &last_change_info_value.value,
                f,
            )?;
        }

        if let Some(contributors_info_value) = self.contributors.get(&self.disabled_fields) {
            self.write_styled_info_line(
                &self.contributors.title(),
                &contributors_info_value.value,
                f,
            )?;
        }

        if let Some(repo_info_value) = self.repo.get(&self.disabled_fields) {
            self.write_styled_info_line(&self.repo.title(), &repo_info_value.value, f)?;
        }

        if let Some(commits_info_value) = self.commits.get(&self.disabled_fields) {
            self.write_styled_info_line(&self.commits.title(), &commits_info_value.value, f)?;
        }

        if let Some(lines_of_code_info_value) = self.lines_of_code.get(&self.disabled_fields) {
            self.write_styled_info_line(
                &self.lines_of_code.title(),
                &lines_of_code_info_value.value,
                f,
            )?;
        }

        if let Some(size_info_value) = self.size.get(&self.disabled_fields) {
            self.write_styled_info_line(&self.size.title(), &size_info_value.value, f)?;
        }

        if let Some(license_info_value) = self.license.get(&self.disabled_fields) {
            self.write_styled_info_line(&self.license.title(), &license_info_value.value, f)?;
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
    pub fn new(config: &Config) -> Result<Self> {
        let git_version = cli::get_git_version();
        let repo = git_repository::discover(&config.input)?;
        let workdir = repo
            .work_dir()
            .context("please run onefetch inside of a non-bare git repository")?
            .to_owned();

        let pending_changes = std::thread::spawn({
            let git_dir = repo.git_dir().to_owned();
            move || {
                let repo = git2::Repository::open(git_dir)?;
                git::get_pending_changes(&repo)
            }
        });
        let languages_handle = std::thread::spawn({
            let ignored_directories = config.exclude.clone();
            let language_types = config.r#type.clone();
            let include_hidden = config.include_hidden;
            let workdir = workdir.clone();
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

        let repo = Repo::new(repo)?;
        let mut commits = Commits::new(
            repo.gitoxide(),
            config.no_merges,
            &no_bots,
            config.number_of_authors,
            config.email,
        )?;
        let (repo_name, repo_url) = repo.get_name_and_url()?;
        let head_refs = repo.get_head_refs()?;
        let version = repo.get_version()?;
        let git_username = repo.get_git_username();
        let number_of_tags = repo.get_number_of_tags()?;
        let number_of_branches = repo.get_number_of_branches()?;
        let (repo_size, file_count) = repo.get_repo_size();
        let license = Detector::new()?.get_license(&workdir)?;
        let dependencies = DependencyDetector::new().get_dependencies(&workdir)?;

        let creation_date = commits.get_creation_date(config.iso_time);
        let number_of_commits = commits.count();
        let (authors, contributors) = commits.take_authors();
        let last_change = commits.get_date_of_last_commit(config.iso_time);

        let pending_changes = pending_changes
            .join()
            .ok()
            .context("BUG: panic in pending-changes thread")??;

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
        let title = Title {
            git_username,
            git_version,
            title_color: text_colors.title,
            tilde_color: text_colors.tilde,
            underline_color: text_colors.underline,
            is_bold: !config.no_bold,
        };
        let project = ProjectInfo {
            repo_name,
            number_of_tags,
            number_of_branches,
        };
        let head = HeadInfo { head_refs };
        let pending = PendingInfo { pending_changes };
        let version = VersionInfo { version };
        let created = CreatedInfo { creation_date };
        let languages = LanguagesInfo::new(languages, true_color, text_colors.info);
        let dependencies = DependenciesInfo { dependencies };
        let authors = AuthorsInfo {
            authors,
            info_color: text_colors.info,
        };
        let last_change = LastChangeInfo { last_change };
        let contributors = ContributorsInfo { contributors };
        let repo = UrlInfo { repo_url };
        let commits = CommitsInfo { number_of_commits };
        let lines_of_code = LocInfo { lines_of_code };
        let size = SizeInfo {
            repo_size,
            file_count,
        };
        let license = LicenseInfo { license };

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
            repo,
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
        state.serialize_field("authors", &self.authors.authors)?;
        state.serialize_field("languages", &self.languages.languages_with_percentage)?;

        state.end()
    }
}
