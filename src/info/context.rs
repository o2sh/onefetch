use super::InfoOptions;
use super::manifest::Manifest;
use crate::git::metrics::GitMetrics;
use crate::language::Language;
use gix::Repository;
use std::path::PathBuf;

/// Everything read from the repository once and shared by the fields.
pub(crate) struct Context<'a> {
    pub repo: Repository,
    pub repo_path: PathBuf,
    pub repo_url: String,
    pub manifest: Option<Manifest>,
    pub git_metrics: GitMetrics,
    /// Sorted by lines of code, largest first. `None` when no code was found.
    pub loc_by_language: Option<Vec<(Language, usize)>>,
    pub options: &'a InfoOptions,
}
