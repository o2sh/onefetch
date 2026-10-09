use super::fields::InfoField;
use super::fields::InfoKind;
use super::fields::authors::AuthorsInfo;
use super::fields::churn::ChurnInfo;
use super::fields::commits::CommitsInfo;
use super::fields::contributors::ContributorsInfo;
use super::fields::created::CreatedInfo;
use super::fields::dependencies::DependenciesInfo;
use super::fields::description::DescriptionInfo;
use super::fields::head::HeadInfo;
use super::fields::languages::LanguagesInfo;
use super::fields::last_change::LastChangeInfo;
use super::fields::license::LicenseInfo;
use super::fields::lines_of_code::LocInfo;
use super::fields::pending::PendingInfo;
use super::fields::project::ProjectInfo;
use super::fields::size::SizeInfo;
use super::fields::url::UrlInfo;
use super::fields::version::VersionInfo;
use super::title::Title;
use super::{Info, InfoOptions};
use crate::git::metrics::GitMetrics;
use crate::language::Language;
use anyhow::Result;
use gix::Repository;
use onefetch_manifest::Manifest;
use std::path::Path;

pub struct InfoBuilder {
    title: Option<Title>,
    info_fields: Vec<Box<dyn InfoField>>,
    disabled_fields: Vec<InfoKind>,
    no_title: bool,
}

impl InfoBuilder {
    pub fn new(options: &InfoOptions) -> Self {
        Self {
            title: None,
            info_fields: Vec::new(),
            disabled_fields: options.disabled_fields.clone(),
            no_title: options.no_title,
        }
    }

    pub fn title(mut self, repo: &Repository) -> Self {
        if !self.no_title {
            self.title = Some(Title::new(repo));
        }
        self
    }

    pub fn description(mut self, manifest: Option<&Manifest>) -> Self {
        if !self.disabled_fields.contains(&InfoKind::Description) {
            let description = DescriptionInfo::new(manifest);
            self.info_fields.push(Box::new(description));
        }
        self
    }

    pub fn pending(mut self, repo: &Repository) -> Result<Self> {
        if !self.disabled_fields.contains(&InfoKind::Pending) {
            let pending = PendingInfo::new(repo)?;
            self.info_fields.push(Box::new(pending));
        }
        Ok(self)
    }

    pub fn url(mut self, repo_url: &str) -> Self {
        if !self.disabled_fields.contains(&InfoKind::URL) {
            let repo_url = UrlInfo::new(repo_url);
            self.info_fields.push(Box::new(repo_url));
        }
        self
    }

    pub fn project(
        mut self,
        repo: &Repository,
        repo_url: &str,
        manifest: Option<&Manifest>,
    ) -> Result<Self> {
        if !self.disabled_fields.contains(&InfoKind::Project) {
            let project = ProjectInfo::new(repo, repo_url, manifest)?;
            self.info_fields.push(Box::new(project));
        }
        Ok(self)
    }

    pub fn head(mut self, repo: &Repository) -> Result<Self> {
        if !self.disabled_fields.contains(&InfoKind::Head) {
            let head = HeadInfo::new(repo)?;
            self.info_fields.push(Box::new(head));
        }
        Ok(self)
    }

    pub fn version(mut self, repo: &Repository, manifest: Option<&Manifest>) -> Result<Self> {
        if !self.disabled_fields.contains(&InfoKind::Version) {
            let version = VersionInfo::new(repo, manifest)?;
            self.info_fields.push(Box::new(version));
        }
        Ok(self)
    }

    pub fn size(mut self, repo: &Repository) -> Self {
        if !self.disabled_fields.contains(&InfoKind::Size) {
            let size = SizeInfo::new(repo);
            self.info_fields.push(Box::new(size));
        }
        self
    }

    pub fn license(mut self, repo_path: &Path, manifest: Option<&Manifest>) -> Result<Self> {
        if !self.disabled_fields.contains(&InfoKind::License) {
            let license = LicenseInfo::new(repo_path, manifest)?;
            self.info_fields.push(Box::new(license));
        }
        Ok(self)
    }

    pub fn created(mut self, git_metrics: &GitMetrics) -> Self {
        if !self.disabled_fields.contains(&InfoKind::Created) {
            let created = CreatedInfo::new(git_metrics);
            self.info_fields.push(Box::new(created));
        }
        self
    }

    pub fn languages(mut self, loc_by_language_opt: Option<&Vec<(Language, usize)>>) -> Self {
        if !self.disabled_fields.contains(&InfoKind::Languages)
            && let Some(loc_by_language) = loc_by_language_opt
        {
            let languages = LanguagesInfo::new(loc_by_language);
            self.info_fields.push(Box::new(languages));
        }
        self
    }

    pub fn dependencies(mut self, manifest: Option<&Manifest>) -> Self {
        if !self.disabled_fields.contains(&InfoKind::Dependencies) {
            let dependencies = DependenciesInfo::new(manifest);
            self.info_fields.push(Box::new(dependencies));
        }
        self
    }

    pub fn authors(
        mut self,
        git_metrics: &GitMetrics,
        number_of_authors_to_display: usize,
        show_email: bool,
    ) -> Self {
        if !self.disabled_fields.contains(&InfoKind::Authors) {
            let authors = AuthorsInfo::new(
                &git_metrics.number_of_commits_by_identity,
                git_metrics.total_number_of_commits,
                number_of_authors_to_display,
                show_email,
            );
            self.info_fields.push(Box::new(authors));
        }
        self
    }

    pub fn last_change(mut self, git_metrics: &GitMetrics) -> Self {
        if !self.disabled_fields.contains(&InfoKind::LastChange) {
            let last_change = LastChangeInfo::new(git_metrics);
            self.info_fields.push(Box::new(last_change));
        }
        self
    }

    pub fn contributors(mut self, git_metrics: &GitMetrics) -> Self {
        if !self.disabled_fields.contains(&InfoKind::Contributors) {
            let contributors = ContributorsInfo::new(git_metrics.total_number_of_authors);
            self.info_fields.push(Box::new(contributors));
        }
        self
    }

    pub fn commits(mut self, git_metrics: &GitMetrics, is_shallow: bool) -> Self {
        if !self.disabled_fields.contains(&InfoKind::Commits) {
            let commits = CommitsInfo::new(git_metrics, is_shallow);
            self.info_fields.push(Box::new(commits));
        }
        self
    }

    pub fn churn(
        mut self,
        git_metrics: &GitMetrics,
        number_of_file_churns_to_display: usize,
        globs_to_exclude: &[String],
    ) -> Result<Self> {
        if !self.disabled_fields.contains(&InfoKind::Churn) {
            let churn = ChurnInfo::new(
                &git_metrics.number_of_commits_by_file_path,
                git_metrics.churn_pool_size,
                number_of_file_churns_to_display,
                globs_to_exclude,
            )?;
            self.info_fields.push(Box::new(churn));
        }
        Ok(self)
    }

    pub fn lines_of_code(mut self, loc_by_language_opt: Option<&Vec<(Language, usize)>>) -> Self {
        if !self.disabled_fields.contains(&InfoKind::LinesOfCode)
            && let Some(loc_by_language) = loc_by_language_opt
        {
            let lines_of_code = LocInfo::new(loc_by_language);
            self.info_fields.push(Box::new(lines_of_code));
        }
        self
    }

    pub fn build(self, dominant_language: Option<Language>) -> Info {
        Info {
            title: self.title,
            info_fields: self.info_fields,
            dominant_language,
        }
    }
}
