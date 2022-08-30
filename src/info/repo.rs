use super::{
    git::{Commits, Repo},
    info_field::{InfoField, InfoFieldValue, InfoType},
};
use anyhow::Result;

pub struct UrlInfo {
    pub repo_url: String,
}
impl UrlInfo {
    pub fn new(repo: &Repo) -> Result<Self> {
        let repo_url = repo.get_url()?;
        Ok(Self { repo_url })
    }
}

impl InfoField for UrlInfo {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: InfoType::Repo,
            value: self.repo_url.to_string(),
        }
    }
    fn title(&self) -> String {
        String::from("Repo")
    }
}

pub struct CreatedInfo {
    pub creation_date: String,
}

impl CreatedInfo {
    pub fn new(iso_time: bool, commits: &Commits) -> Self {
        let creation_date = commits.get_creation_date(iso_time);
        Self { creation_date }
    }
}

impl InfoField for CreatedInfo {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: InfoType::Created,
            value: self.creation_date.to_string(),
        }
    }
    fn title(&self) -> String {
        String::from("Created")
    }
}

pub struct PendingInfo {
    pub pending_changes: String,
}

impl InfoField for PendingInfo {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: InfoType::Pending,
            value: self.pending_changes.to_string(),
        }
    }
    fn title(&self) -> String {
        String::from("Pending")
    }
}

pub struct LastChangeInfo {
    pub last_change: String,
}

impl LastChangeInfo {
    pub fn new(iso_time: bool, commits: &Commits) -> Self {
        let last_change = commits.get_date_of_last_commit(iso_time);

        Self { last_change }
    }
}

impl InfoField for LastChangeInfo {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: InfoType::LastChange,
            value: self.last_change.to_string(),
        }
    }
    fn title(&self) -> String {
        String::from("Last change")
    }
}

pub struct CommitsInfo {
    pub number_of_commits: String,
}

impl CommitsInfo {
    pub fn new(commits: &Commits) -> Self {
        let number_of_commits = commits.count();
        Self { number_of_commits }
    }
}
impl InfoField for CommitsInfo {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: InfoType::Commits,
            value: self.number_of_commits.to_string(),
        }
    }
    fn title(&self) -> String {
        String::from("Commits")
    }
}

pub struct SizeInfo {
    pub repo_size: String,
    pub file_count: u64,
}

impl SizeInfo {
    pub fn new(repo: &Repo) -> Self {
        let (repo_size, file_count) = repo.get_repo_size();
        Self {
            repo_size,
            file_count,
        }
    }
}
impl std::fmt::Display for SizeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.file_count {
            0 => write!(f, "{}", &self.repo_size),
            _ => {
                write!(f, "{} ({} files)", self.repo_size, self.file_count)
            }
        }
    }
}

impl InfoField for SizeInfo {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: InfoType::Size,
            value: self.to_string(),
        }
    }
    fn title(&self) -> String {
        String::from("Size")
    }
}

pub struct LocInfo {
    pub lines_of_code: usize,
}

impl InfoField for LocInfo {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: InfoType::LinesOfCode,
            value: self.lines_of_code.to_string(),
        }
    }
    fn title(&self) -> String {
        String::from("Lines of code")
    }
}

pub struct VersionInfo {
    pub version: String,
}

impl VersionInfo {
    pub fn new(repo: &Repo) -> Result<Self> {
        let version = repo.get_version()?;
        Ok(Self { version })
    }
}
impl InfoField for VersionInfo {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: InfoType::Version,
            value: self.version.to_string(),
        }
    }
    fn title(&self) -> String {
        String::from("Version")
    }
}

pub struct ContributorsInfo {
    pub number_of_contributors: usize,
}

impl ContributorsInfo {
    pub fn new(commits: &Commits) -> Self {
        let contributors = commits.number_of_contributors();
        Self {
            number_of_contributors: contributors,
        }
    }
}
impl InfoField for ContributorsInfo {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: InfoType::Contributors,
            value: self.number_of_contributors.to_string(),
        }
    }
    fn title(&self) -> String {
        String::from("Contributors")
    }
}
