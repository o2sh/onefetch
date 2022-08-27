use crate::cli::{self, is_truecolor_terminal, Config, MyRegex, When};
use crate::info::info_field::{FieldType, InfoField, InfoFieldValueGetter};
use crate::ui::get_ascii_colors;
use crate::ui::text_colors::TextColors;
use anyhow::{Context, Result};
use deps::DependencyDetector;
use git::Commits;
use git::Repo;
use langs::language::Language;
use license::Detector;
use owo_colors::{DynColors, OwoColorize, Style};
use regex::Regex;
use serde::ser::SerializeStruct;
use serde::Serialize;
use std::str::FromStr;

use self::author::AuthorsInfoField;
use self::deps::DependenciesInfoField;
use self::langs::language::LanguagesInfoField;
use self::license::LicenseInfoField;
use self::project::ProjectInfoField;
use self::repo::{
    RepoCommitsInfoField, RepoContributorsInfoField, RepoCreatedInfoField, RepoHeadInfoField,
    RepoLastChangeInfoField, RepoLocInfoField, RepoPendingInfoField, RepoSizeInfoField,
    RepoUrlInfoField, RepoVersionInfoField,
};
use self::title::Title;

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
    info_fields: Vec<Box<dyn InfoField>>,
    disabled_fields: Vec<FieldType>,
    text_colors: TextColors,
    no_color_palette: bool,
    no_bold: bool,
    pub dominant_language: Language,
    pub ascii_colors: Vec<DynColors>,
}

impl std::fmt::Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        //Title
        if !self.disabled_fields.contains(&FieldType::Title) {
            writeln!(f, "{}", self.title)?;
        }

        //Info fields
        for info_field in self.info_fields.iter() {
            if let Some(info_field_value) = info_field.get(&self.disabled_fields) {
                self.write_styled_info_line(
                    info_field_value.r#type.as_str(),
                    &info_field_value.value,
                    f,
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
        let (authors, contributors) = commits.take_authors(config.email);
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
        let info_fields: Vec<Box<dyn InfoField>> = vec![
            Box::new(ProjectInfoField {
                repo_name,
                number_of_tags,
                number_of_branches,
            }),
            Box::new(RepoHeadInfoField { head_refs }),
            Box::new(RepoPendingInfoField { pending_changes }),
            Box::new(RepoVersionInfoField { version }),
            Box::new(RepoCreatedInfoField { creation_date }),
            Box::new(LanguagesInfoField {
                languages,
                true_color,
                info_color: text_colors.info,
            }),
            Box::new(DependenciesInfoField { dependencies }),
            Box::new(AuthorsInfoField { authors }),
            Box::new(RepoLastChangeInfoField { last_change }),
            Box::new(RepoContributorsInfoField { contributors }),
            Box::new(RepoUrlInfoField { repo_url }),
            Box::new(RepoCommitsInfoField { number_of_commits }),
            Box::new(RepoLocInfoField { lines_of_code }),
            Box::new(RepoSizeInfoField {
                repo_size,
                file_count,
            }),
            Box::new(LicenseInfoField { license }),
        ];

        Ok(Self {
            title,
            disabled_fields: config.disabled_fields.clone(),
            info_fields,
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
        let mut state = serializer.serialize_struct("Info", 15)?;

        for info_field in self.info_fields.iter() {
            if let Some(info_field_value) = info_field.get(&self.disabled_fields) {
                state.serialize_field(info_field_value.r#type.as_str(), &info_field_value.value)?;
            }
        }

        state.end()
    }
}
