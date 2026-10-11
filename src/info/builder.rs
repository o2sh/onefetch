use super::fields::authors::Authors;
use super::fields::churn::Churn;
use super::fields::commits::Commits;
use super::fields::contributors::Contributors;
use super::fields::created::Created;
use super::fields::dependencies::Dependencies;
use super::fields::description::Description;
use super::fields::head::Head;
use super::fields::languages::Languages;
use super::fields::last_change::LastChange;
use super::fields::license::License;
use super::fields::lines_of_code::LinesOfCode;
use super::fields::pending::Pending;
use super::fields::project::Project;
use super::fields::size::Size;
use super::fields::url::Url;
use super::fields::version::Version;
use super::fields::{Field, FieldKind};
use super::manifest::Manifest;
use super::title::Title;
use super::{Info, InfoOptions};
use crate::git::metrics::GitMetrics;
use crate::language::Language;
use anyhow::Result;
use gix::Repository;
use std::path::Path;

pub struct InfoBuilder {
    title: Option<Title>,
    info_fields: Vec<Field>,
    disabled_fields: Vec<FieldKind>,
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
        if !self.disabled_fields.contains(&FieldKind::Description) {
            let description = Description::new(manifest);
            self.info_fields.push(Field::Description(description));
        }
        self
    }

    pub fn pending(mut self, repo: &Repository) -> Result<Self> {
        if !self.disabled_fields.contains(&FieldKind::Pending) {
            let pending = Pending::new(repo)?;
            self.info_fields.push(Field::Pending(pending));
        }
        Ok(self)
    }

    pub fn url(mut self, repo_url: &str) -> Self {
        if !self.disabled_fields.contains(&FieldKind::Url) {
            let repo_url = Url::new(repo_url);
            self.info_fields.push(Field::Url(repo_url));
        }
        self
    }

    pub fn project(
        mut self,
        repo: &Repository,
        repo_url: &str,
        manifest: Option<&Manifest>,
    ) -> Result<Self> {
        if !self.disabled_fields.contains(&FieldKind::Project) {
            let project = Project::new(repo, repo_url, manifest)?;
            self.info_fields.push(Field::Project(project));
        }
        Ok(self)
    }

    pub fn head(mut self, repo: &Repository) -> Result<Self> {
        if !self.disabled_fields.contains(&FieldKind::Head) {
            let head = Head::new(repo)?;
            self.info_fields.push(Field::Head(head));
        }
        Ok(self)
    }

    pub fn version(mut self, repo: &Repository, manifest: Option<&Manifest>) -> Result<Self> {
        if !self.disabled_fields.contains(&FieldKind::Version) {
            let version = Version::new(repo, manifest)?;
            self.info_fields.push(Field::Version(version));
        }
        Ok(self)
    }

    pub fn size(mut self, repo: &Repository) -> Self {
        if !self.disabled_fields.contains(&FieldKind::Size) {
            let size = Size::new(repo);
            self.info_fields.push(Field::Size(size));
        }
        self
    }

    pub fn license(mut self, repo_path: &Path, manifest: Option<&Manifest>) -> Result<Self> {
        if !self.disabled_fields.contains(&FieldKind::License) {
            let license = License::new(repo_path, manifest)?;
            self.info_fields.push(Field::License(license));
        }
        Ok(self)
    }

    pub fn created(mut self, git_metrics: &GitMetrics) -> Self {
        if !self.disabled_fields.contains(&FieldKind::Created) {
            let created = Created::new(git_metrics);
            self.info_fields.push(Field::Created(created));
        }
        self
    }

    pub fn languages(mut self, loc_by_language_opt: Option<&Vec<(Language, usize)>>) -> Self {
        if !self.disabled_fields.contains(&FieldKind::Languages)
            && let Some(loc_by_language) = loc_by_language_opt
        {
            let languages = Languages::new(loc_by_language);
            self.info_fields.push(Field::Languages(languages));
        }
        self
    }

    pub fn dependencies(mut self, manifest: Option<&Manifest>) -> Self {
        if !self.disabled_fields.contains(&FieldKind::Dependencies) {
            let dependencies = Dependencies::new(manifest);
            self.info_fields.push(Field::Dependencies(dependencies));
        }
        self
    }

    pub fn authors(
        mut self,
        git_metrics: &GitMetrics,
        number_of_authors_to_display: usize,
        show_email: bool,
    ) -> Self {
        if !self.disabled_fields.contains(&FieldKind::Authors) {
            let authors = Authors::new(
                &git_metrics.number_of_commits_by_identity,
                git_metrics.total_number_of_commits,
                number_of_authors_to_display,
                show_email,
            );
            self.info_fields.push(Field::Authors(authors));
        }
        self
    }

    pub fn last_change(mut self, git_metrics: &GitMetrics) -> Self {
        if !self.disabled_fields.contains(&FieldKind::LastChange) {
            let last_change = LastChange::new(git_metrics);
            self.info_fields.push(Field::LastChange(last_change));
        }
        self
    }

    pub fn contributors(mut self, git_metrics: &GitMetrics, number_of_authors: usize) -> Self {
        if !self.disabled_fields.contains(&FieldKind::Contributors) {
            let contributors =
                Contributors::new(git_metrics.total_number_of_authors, number_of_authors);
            self.info_fields.push(Field::Contributors(contributors));
        }
        self
    }

    pub fn commits(mut self, git_metrics: &GitMetrics, is_shallow: bool) -> Self {
        if !self.disabled_fields.contains(&FieldKind::Commits) {
            let commits = Commits::new(git_metrics, is_shallow);
            self.info_fields.push(Field::Commits(commits));
        }
        self
    }

    pub fn churn(
        mut self,
        git_metrics: &GitMetrics,
        number_of_file_churns_to_display: usize,
        globs_to_exclude: &[String],
    ) -> Result<Self> {
        if !self.disabled_fields.contains(&FieldKind::Churn) {
            let churn = Churn::new(
                &git_metrics.number_of_commits_by_file_path,
                git_metrics.churn_pool_size,
                number_of_file_churns_to_display,
                globs_to_exclude,
            )?;
            self.info_fields.push(Field::Churn(churn));
        }
        Ok(self)
    }

    pub fn lines_of_code(mut self, loc_by_language_opt: Option<&Vec<(Language, usize)>>) -> Self {
        if !self.disabled_fields.contains(&FieldKind::LinesOfCode)
            && let Some(loc_by_language) = loc_by_language_opt
        {
            let lines_of_code = LinesOfCode::new(loc_by_language);
            self.info_fields.push(Field::LinesOfCode(lines_of_code));
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
