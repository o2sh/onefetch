use self::authors::AuthorsInfo;
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
use onefetch_manifest::Manifest;
use owo_colors::{DynColors, OwoColorize};
use serde::Serialize;
use std::path::Path;

mod authors;
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
            info_field.write_styled(f, self.no_bold, &self.text_colors)?;
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
    let repo_url = get_repo_url(
        &repo,
        cli_options.info.hide_token,
        cli_options.info.http_url,
    );

    let git_metrics = traverse_commit_graph(
        &repo,
        cli_options.info.no_bots.clone(),
        cli_options.info.churn_pool_size,
        cli_options.info.no_merges,
    )?;
    let true_color = match cli_options.ascii.true_color {
        When::Always => true,
        When::Never => false,
        When::Auto => is_truecolor_terminal(),
    };
    let dominant_language = langs::get_main_language(&loc_by_language);
    let ascii_colors = get_ascii_colors(
        cli_options.ascii.ascii_language.as_ref(),
        &dominant_language,
        &cli_options.ascii.ascii_colors,
        true_color,
    );

    let text_colors = TextColors::new(&cli_options.text_formatting.text_colors, ascii_colors[0]);
    let no_bold = cli_options.text_formatting.no_bold;
    let number_separator = cli_options.text_formatting.number_separator;
    let iso_time = cli_options.text_formatting.iso_time;
    let number_of_languages_to_display = cli_options.info.number_of_languages;
    let number_of_authors_to_display = cli_options.info.number_of_authors;
    let number_of_file_churns_to_display = cli_options.info.number_of_file_churns;
    let globs_to_exclude = &cli_options.info.exclude;
    let show_email = cli_options.info.email;

    Ok(InfoBuilder::new(cli_options)
        .title(&repo, no_bold, &text_colors)
        .project(&repo, &repo_url, manifest.as_ref(), number_separator)?
        .description(manifest.as_ref())
        .head(&repo)?
        .pending(&repo)?
        .version(&repo, manifest.as_ref())?
        .created(&git_metrics, iso_time)
        .languages(
            &loc_by_language,
            true_color,
            number_of_languages_to_display,
            &text_colors,
            cli_options,
        )
        .dependencies(manifest.as_ref(), number_separator)
        .authors(
            &git_metrics,
            number_of_authors_to_display,
            show_email,
            number_separator,
        )
        .last_change(&git_metrics, iso_time)
        .contributors(&git_metrics, number_of_authors_to_display, number_separator)
        .url(&repo_url)
        .commits(&git_metrics, repo.is_shallow(), number_separator)
        .churn(
            &git_metrics,
            number_of_file_churns_to_display,
            globs_to_exclude,
            number_separator,
        )?
        .loc(&loc_by_language, number_separator)
        .size(&repo, number_separator)
        .license(&repo_path, manifest.as_ref())?
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

    fn description(mut self, manifest: Option<&Manifest>) -> Self {
        if !self.disabled_fields.contains(&InfoType::Description) {
            let description = DescriptionInfo::new(manifest);
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
        manifest: Option<&Manifest>,
        number_separator: NumberSeparator,
    ) -> Result<Self> {
        if !self.disabled_fields.contains(&InfoType::Project) {
            let project = ProjectInfo::new(repo, repo_url, manifest, number_separator)?;
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

    fn version(mut self, repo: &Repository, manifest: Option<&Manifest>) -> Result<Self> {
        if !self.disabled_fields.contains(&InfoType::Version) {
            let version = VersionInfo::new(repo, manifest)?;
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

    fn license(mut self, repo_path: &Path, manifest: Option<&Manifest>) -> Result<Self> {
        if !self.disabled_fields.contains(&InfoType::License) {
            let license = LicenseInfo::new(repo_path, manifest)?;
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
        cli_options: &CliOptions,
    ) -> Self {
        if !self.disabled_fields.contains(&InfoType::Languages) {
            let languages = LanguagesInfo::new(
                loc_by_language,
                true_color,
                number_of_languages,
                text_colors.info,
                cli_options.visuals.nerd_fonts,
            );
            self.info_fields.push(Box::new(languages));
        }
        self
    }

    fn dependencies(
        mut self,
        manifest: Option<&Manifest>,
        number_separator: NumberSeparator,
    ) -> Self {
        if !self.disabled_fields.contains(&InfoType::Dependencies) {
            let dependencies = DependenciesInfo::new(manifest, number_separator);
            self.info_fields.push(Box::new(dependencies));
        }
        self
    }

    fn authors(
        mut self,
        git_metrics: &GitMetrics,
        number_of_authors_to_display: usize,
        show_email: bool,
        number_separator: NumberSeparator,
    ) -> Self {
        if !self.disabled_fields.contains(&InfoType::Authors) {
            let authors = AuthorsInfo::new(
                &git_metrics.number_of_commits_by_signature,
                git_metrics.total_number_of_commits,
                number_of_authors_to_display,
                show_email,
                number_separator,
            );
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
        number_of_authors_to_display: usize,
        number_separator: NumberSeparator,
    ) -> Self {
        if !self.disabled_fields.contains(&InfoType::Contributors) {
            let contributors = ContributorsInfo::new(
                git_metrics.total_number_of_authors,
                number_of_authors_to_display,
                number_separator,
            );
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

    fn churn(
        mut self,
        git_metrics: &GitMetrics,
        number_of_file_churns_to_display: usize,
        globs_to_exclude: &[String],
        number_separator: NumberSeparator,
    ) -> Result<Self> {
        if !self.disabled_fields.contains(&InfoType::Churn) {
            let churn = ChurnInfo::new(
                &git_metrics.number_of_commits_by_file_path,
                git_metrics.churn_pool_size,
                number_of_file_churns_to_display,
                globs_to_exclude,
                number_separator,
            )?;
            self.info_fields.push(Box::new(churn));
        }
        Ok(self)
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
