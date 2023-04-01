use self::author::AuthorsInfo;
use self::commits::CommitsInfo;
use self::contributors::ContributorsInfo;
use self::created::CreatedInfo;
use self::dependencies::DependenciesInfo;
use self::description::DescriptionInfo;
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
use self::url::UrlInfo;
use self::utils::git::Commits;
use self::utils::info_field::{InfoField, InfoType};
use self::version::VersionInfo;
use crate::cli::{is_truecolor_terminal, CliOptions, NumberSeparator, When};
use crate::ui::get_ascii_colors;
use crate::ui::text_colors::TextColors;
use anyhow::{Context, Result};
use num_format::ToFormattedString;
use onefetch_manifest::Manifest;
use owo_colors::{DynColors, OwoColorize, Style};
use serde::Serialize;
use std::path::Path;

mod author;
mod commits;
mod contributors;
mod created;
mod dependencies;
mod description;
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
    pub fn new(cli_options: &CliOptions) -> Result<Self> {
        let git_repo = gix::discover(&cli_options.input)?;
        let repo_path = get_work_dir(&git_repo)?;

        let loc_by_language_sorted_handle = std::thread::spawn({
            let ignored_directories = cli_options.info.exclude.clone();
            let language_types = cli_options.info.r#type.clone();
            let include_hidden = cli_options.info.include_hidden;
            let workdir = repo_path.clone();
            move || {
                langs::get_loc_by_language_sorted(
                    &workdir,
                    &ignored_directories,
                    &language_types,
                    include_hidden,
                )
            }
        });

        let loc_by_language = loc_by_language_sorted_handle
            .join()
            .ok()
            .context("BUG: panic in language statistics thread")??;
        let dominant_language = langs::get_main_language(&loc_by_language);
        let true_color = match cli_options.ascii.true_color {
            When::Always => true,
            When::Never => false,
            When::Auto => is_truecolor_terminal(),
        };
        let ascii_colors = get_ascii_colors(
            &cli_options.ascii.ascii_language,
            &dominant_language,
            &cli_options.ascii.ascii_colors,
            true_color,
        );

        let text_colors =
            TextColors::new(&cli_options.text_formatting.text_colors, ascii_colors[0]);
        let title = Title::new(
            &git_repo,
            text_colors.title,
            text_colors.tilde,
            text_colors.underline,
            !cli_options.text_formatting.no_bold,
        );
        let manifest = get_manifest(&repo_path)?;
        let description = DescriptionInfo::new(manifest.as_ref());
        let pending = PendingInfo::new(&git_repo)?;
        let repo_url = UrlInfo::new(&git_repo)?;
        let project = ProjectInfo::new(
            &git_repo,
            &repo_url.repo_url,
            manifest.as_ref(),
            cli_options.text_formatting.number_separator,
        )?;
        let head = HeadInfo::new(&git_repo)?;
        let version = VersionInfo::new(&git_repo, manifest.as_ref())?;
        let size = SizeInfo::new(&git_repo, cli_options.text_formatting.number_separator);
        let license = LicenseInfo::new(&repo_path, manifest.as_ref())?;
        let mut commits = Commits::new(
            git_repo,
            cli_options.info.no_merges,
            &cli_options.info.no_bots,
            cli_options.info.number_of_authors,
            cli_options.info.email,
            cli_options.text_formatting.number_separator,
        )?;
        let created = CreatedInfo::new(cli_options.text_formatting.iso_time, &commits);
        let languages = LanguagesInfo::new(
            &loc_by_language,
            true_color,
            cli_options.info.number_of_languages,
            text_colors.info,
        );
        let dependencies = DependenciesInfo::new(
            manifest.as_ref(),
            cli_options.text_formatting.number_separator,
        );
        let authors = AuthorsInfo::new(text_colors.info, &mut commits);
        let last_change = LastChangeInfo::new(cli_options.text_formatting.iso_time, &commits);
        let contributors = ContributorsInfo::new(
            &commits,
            cli_options.info.number_of_authors,
            cli_options.text_formatting.number_separator,
        );
        let commits = CommitsInfo::new(&commits, cli_options.text_formatting.number_separator);
        let lines_of_code = LocInfo::new(
            &loc_by_language,
            cli_options.text_formatting.number_separator,
        );

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
            disabled_fields: cli_options.info.disabled_fields.clone(),
            text_colors,
            dominant_language,
            ascii_colors,
            no_color_palette: cli_options.color_blocks.no_color_palette,
            no_title: cli_options.info.no_title,
            no_bold: cli_options.text_formatting.no_bold,
        })
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
