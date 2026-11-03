use self::builder::InfoBuilder;
use self::git::bots::BotRegex;
use self::git::{
    get_latest_tag, get_repo_url, get_work_dir, traverse_commit_graph, uses_reftables,
};
use self::manifest::get_manifest;
use self::title::Title;
use crate::language::{Language, LanguageType};
use anyhow::{Context, Result, bail};
use serde::Serialize;
use std::path::PathBuf;

mod builder;
pub(crate) mod fields;
pub mod git;
pub(crate) mod loc;
pub mod manifest;
pub(crate) mod title;

pub(crate) use fields::Field;
pub use fields::FieldKind;

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub(crate) title: Option<Title>,
    pub(crate) info_fields: Vec<Field>,
    #[serde(skip_serializing)]
    pub main_language: Option<Language>,
}

pub const DEFAULT_NUMBER_OF_AUTHORS: usize = 3;
pub const DEFAULT_NUMBER_OF_LANGUAGES: usize = 6;
pub const DEFAULT_NUMBER_OF_FILE_CHURNS: usize = 3;
pub const DEFAULT_ACTIVITY_MONTHS: usize = 12;

/// What to collect from the repository and which fields to show.
#[derive(Clone, Debug)]
pub struct InfoOptions {
    pub input: PathBuf,
    pub disabled_fields: Vec<FieldKind>,
    pub no_title: bool,
    pub number_of_authors: usize,
    pub number_of_languages: usize,
    pub number_of_file_churns: usize,
    pub churn_pool_size: Option<usize>,
    pub activity_months: usize,
    pub exclude: Vec<String>,
    pub no_bots: Option<BotRegex>,
    pub no_merges: bool,
    pub co_authors: bool,
    pub email: bool,
    pub http_url: bool,
    pub hide_token: bool,
    pub include_hidden: bool,
    pub language_types: Vec<LanguageType>,
}

pub fn build_info(options: &InfoOptions) -> Result<Info> {
    let repo = gix::discover(&options.input)?;
    if uses_reftables(&repo) {
        // TODO: remove once gitoxide supports reftable
        bail!("reftable repositories are not yet supported");
    }
    let repo_path = get_work_dir(&repo)?;
    // Compute LOC in a separate thread so it runs in parallel with commit-graph traversal.
    let loc_by_language_sorted_handle = std::thread::spawn({
        let globs_to_exclude = options.exclude.clone();
        let language_types = options.language_types.clone();
        let include_hidden = options.include_hidden;
        let workdir = repo_path.clone();
        move || {
            loc::get_loc_by_language_sorted(
                &workdir,
                &globs_to_exclude,
                &language_types,
                include_hidden,
            )
        }
    });
    let git_metrics = traverse_commit_graph(
        &repo,
        options.no_bots.clone(),
        options.churn_pool_size,
        options.no_merges,
        options.co_authors,
    )
    .context("Failed to traverse Git commit history")?;
    let manifest = get_manifest(&repo_path)?;
    let repo_url = get_repo_url(&repo, options.hide_token, options.http_url)
        .context("Failed to determine repository URL")?;
    let loc_by_language = loc_by_language_sorted_handle
        .join()
        .ok()
        .context("BUG: panic in language statistics thread")?;
    let main_language = loc_by_language.as_ref().map(|v| loc::get_main_language(v));
    let latest_tag = get_latest_tag(&repo)?;

    Ok(InfoBuilder::new(options)
        .title(&repo)
        .project(&repo, &repo_url, manifest.as_ref())?
        .description(manifest.as_ref())
        .head(&repo)?
        .pending(&repo)?
        .version(latest_tag.as_ref(), manifest.as_ref())
        .unreleased(&repo, latest_tag.as_ref())?
        .created(&git_metrics)
        .languages(loc_by_language.as_ref(), options.number_of_languages)
        .dependencies(manifest.as_ref())
        .authors(&git_metrics, options.number_of_authors, options.email)
        .last_change(&git_metrics)
        .activity(&git_metrics, options.activity_months)
        .contributors(&git_metrics, options.number_of_authors)
        .url(&repo_url)
        .commits(&git_metrics, repo.is_shallow()?)
        .churn(
            &git_metrics,
            options.number_of_file_churns,
            &options.exclude,
        )?
        .lines_of_code(loc_by_language.as_ref())
        .size(&repo)
        .license(&repo_path, manifest.as_ref())?
        .build(main_language))
}
