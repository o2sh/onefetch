use self::context::Context as RepoContext;
use self::fields::url::get_repo_url;
use self::title::Title;
use crate::git::bots::BotRegex;
use crate::git::{get_work_dir, traverse_commit_graph, uses_reftables};
use crate::info::manifest::Manifest;
use crate::language::{Language, LanguageType};
use anyhow::{Context, Result, bail};
use serde::Serialize;
use std::path::{Path, PathBuf};
use strum::VariantArray;

mod context;
pub(crate) mod dates;
mod field;
pub mod fields;
pub(crate) mod loc;
pub mod manifest;
pub mod title;

pub use field::{Field, FieldKind};

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub title: Option<Title>,
    pub info_fields: Vec<Field>,
    #[serde(skip_serializing)]
    pub dominant_language: Option<Language>,
}

pub const DEFAULT_NUMBER_OF_AUTHORS: usize = 3;

/// What to collect from the repository and which fields to show.
#[derive(Clone, Debug)]
pub struct InfoOptions {
    pub input: PathBuf,
    pub disabled_fields: Vec<FieldKind>,
    pub no_title: bool,
    pub number_of_authors: usize,
    pub number_of_file_churns: usize,
    pub churn_pool_size: Option<usize>,
    pub exclude: Vec<String>,
    pub no_bots: Option<BotRegex>,
    pub no_merges: bool,
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
    )
    .context("Failed to traverse Git commit history")?;
    let manifest = get_manifest(&repo_path)?;
    let repo_url = get_repo_url(&repo, options.hide_token, options.http_url)
        .context("Failed to determine repository URL")?;
    let loc_by_language = loc_by_language_sorted_handle
        .join()
        .ok()
        .context("BUG: panic in language statistics thread")?;
    let dominant_language = loc_by_language.as_ref().map(|v| loc::get_main_language(v));

    let ctx = RepoContext {
        repo,
        repo_path,
        repo_url,
        manifest,
        git_metrics,
        loc_by_language,
        options,
    };

    let title = (!options.no_title).then(|| Title::new(&ctx.repo));
    let info_fields = FieldKind::VARIANTS
        .iter()
        .filter(|kind| !options.disabled_fields.contains(kind))
        .filter_map(|&kind| Field::collect(kind, &ctx).transpose())
        .collect::<Result<_>>()?;

    Ok(Info {
        title,
        info_fields,
        dominant_language,
    })
}

fn get_manifest(repo_path: &Path) -> Result<Option<Manifest>> {
    let manifests = manifest::get_manifests(repo_path)?;

    if manifests.is_empty() {
        Ok(None)
    } else {
        Ok(manifests.first().cloned())
    }
}
