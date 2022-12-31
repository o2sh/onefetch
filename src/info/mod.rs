use self::git::Commits;
use self::info_field::{InfoField, InfoType};
use self::langs::language::Language;
use self::langs::language::LanguagesInfo;
use self::repo::author::AuthorsInfo;
use self::repo::commits::CommitsInfo;
use self::repo::contributors::ContributorsInfo;
use self::repo::created::CreatedInfo;
use self::repo::dependencies::DependenciesInfo;
use self::repo::description::DescriptionInfo;
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
use crate::cli::{is_truecolor_terminal, Config, NumberSeparator, When};
use crate::ui::get_ascii_colors;
use crate::ui::text_colors::TextColors;
use anyhow::{Context, Result};
use num_format::ToFormattedString;
use onefetch_manifest::Manifest;
use owo_colors::{DynColors, OwoColorize, Style};
use serde::Serialize;
use std::path::Path;

mod git;
pub mod info_field;
pub mod langs;
mod repo;
#[cfg(test)]
pub mod test;
pub mod title;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    title: Title,
    info_fields: Vec<Box<dyn InfoField>>,
    #[serde(skip_serializing)]
    disabled_fields: Vec<InfoType>,
    #[serde(skip_serializing)]
    text_colors: TextColors,
    #[serde(skip_serializing)]
    no_color_palette: bool,
    #[serde(skip_serializing)]
    no_title: bool,
    #[serde(skip_serializing)]
    no_bold: bool,
    #[serde(skip_serializing)]
    pub dominant_language: Language,
    #[serde(skip_serializing)]
    pub ascii_colors: Vec<DynColors>,
}

impl std::fmt::Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        //Title
        if !self.no_title {
            write!(f, "{}", self.title)?;
        }

        //Info lines
        for info_field in self
            .info_fields
            .iter()
            .filter(|x| x.has_value(&self.disabled_fields))
        {
            self.write_styled_info_line(
                f,
                &info_field.title(),
                &info_field.value(),
                info_field.should_color(),
            )?
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
        let manifest = Self::get_manifest(&repo_path)?;
        let description = DescriptionInfo::new(manifest.as_ref());
        let pending = PendingInfo::new(&git_repo)?;
        let repo_url = UrlInfo::new(&git_repo)?;
        let project = ProjectInfo::new(
            &git_repo,
            &repo_url.repo_url,
            manifest.as_ref(),
            config.number_separator,
        )?;
        let head = HeadInfo::new(&git_repo)?;
        let version = VersionInfo::new(&git_repo, manifest.as_ref())?;
        let size = SizeInfo::new(&git_repo, config.number_separator);
        let license = LicenseInfo::new(&repo_path, manifest.as_ref())?;
        let mut commits = Commits::new(
            git_repo,
            config.no_merges,
            &config.no_bots,
            config.number_of_authors,
            config.email,
            config.number_separator,
        )?;
        let created = CreatedInfo::new(config.iso_time, &commits);
        let languages = LanguagesInfo::new(
            languages,
            true_color,
            config.number_of_languages,
            text_colors.info,
        );
        let dependencies = DependenciesInfo::new(manifest.as_ref(), config.number_separator);
        let authors = AuthorsInfo::new(text_colors.info, &mut commits);
        let last_change = LastChangeInfo::new(config.iso_time, &commits);
        let contributors =
            ContributorsInfo::new(&commits, config.number_of_authors, config.number_separator);
        let commits = CommitsInfo::new(&commits, config.number_separator);
        let lines_of_code = LocInfo::new(lines_of_code, config.number_separator);

        let info_fields: Vec<Box<dyn InfoField>> = vec![
            Box::new(project),
            Box::new(description),
            Box::new(head),
            Box::new(pending),
            Box::new(version),
            Box::new(created),
            Box::new(languages),
            Box::new(dependencies),
            Box::new(authors),
            Box::new(last_change),
            Box::new(contributors),
            Box::new(repo_url),
            Box::new(commits),
            Box::new(lines_of_code),
            Box::new(size),
            Box::new(license),
        ];
        Ok(Self {
            title,
            info_fields,
            disabled_fields: config.disabled_fields.clone(),
            text_colors,
            dominant_language,
            ascii_colors,
            no_color_palette: config.no_color_palette,
            no_title: config.no_title,
            no_bold: config.no_bold,
        })
    }

    fn get_manifest(repo_path: &Path) -> Result<Option<Manifest>> {
        let manifests = onefetch_manifest::get_manifests(repo_path)?;

        if manifests.is_empty() {
            Ok(None)
        } else {
            Ok(manifests.first().cloned())
        }
    }

    fn write_styled_info_line(
        &self,
        f: &mut std::fmt::Formatter,
        subtitle: &str,
        info: &str,
        should_color_info: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{} {}",
            &self.style_subtitle(subtitle),
            &self.style_info(info, should_color_info)
        )
    }

    fn style_info(&self, info: &str, with_color: bool) -> String {
        if with_color {
            let info_style = get_style(false, self.text_colors.info);
            format!("{}", info.style(info_style))
        } else {
            info.into()
        }
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

fn format_number<T: ToFormattedString + std::fmt::Display>(
    number: T,
    number_separator: NumberSeparator,
) -> String {
    number.to_formatted_string(&number_separator.get_format())
}

fn get_style(is_bold: bool, color: DynColors) -> Style {
    let mut style = Style::new().color(color);
    if is_bold {
        style = style.bold();
    }
    style
}

#[cfg(test)]
mod tests {
    use super::*;
    use owo_colors::AnsiColors;

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
    fn test_format_number() {
        assert_eq!(
            &format_number(1_000_000, NumberSeparator::Comma),
            "1,000,000"
        );
        assert_eq!(
            &format_number(1_000_000, NumberSeparator::Space),
            "1\u{202f}000\u{202f}000"
        );
        assert_eq!(
            &format_number(1_000_000, NumberSeparator::Underscore),
            "1_000_000"
        );
        assert_eq!(&format_number(1_000_000, NumberSeparator::Plain), "1000000");
    }
}
