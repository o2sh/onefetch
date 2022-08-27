use super::{
    head::HeadRefs,
    info_field::{FieldType, InfoField, InfoFieldValue},
};

pub struct RepoUrlInfoField {
    pub repo_url: String,
}

impl InfoField for RepoUrlInfoField {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: FieldType::Repo,
            value: self.repo_url.to_string(),
        }
    }
}

pub struct RepoCreatedInfoField {
    pub creation_date: String,
}

impl InfoField for RepoCreatedInfoField {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: FieldType::Created,
            value: self.creation_date.to_string(),
        }
    }
}

pub struct RepoPendingInfoField {
    pub pending_changes: String,
}

impl InfoField for RepoPendingInfoField {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: FieldType::Pending,
            value: self.pending_changes.to_string(),
        }
    }
}

pub struct RepoLastChangeInfoField {
    pub last_change: String,
}

impl InfoField for RepoLastChangeInfoField {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: FieldType::LastChange,
            value: self.last_change.to_string(),
        }
    }
}

pub struct RepoCommitsInfoField {
    pub number_of_commits: String,
}

impl InfoField for RepoCommitsInfoField {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: FieldType::Commits,
            value: self.number_of_commits.to_string(),
        }
    }
}

pub struct RepoSizeInfoField {
    pub repo_size: String,
    pub file_count: u64,
}

impl std::fmt::Display for RepoSizeInfoField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.file_count {
            0 => write!(f, "{}", &self.repo_size),
            _ => {
                write!(f, "{} ({} files)", self.repo_size, self.file_count)
            }
        }
    }
}

impl InfoField for RepoSizeInfoField {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: FieldType::Size,
            value: self.to_string(),
        }
    }
}

pub struct RepoLocInfoField {
    pub lines_of_code: usize,
}

impl InfoField for RepoLocInfoField {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: FieldType::LinesOfCode,
            value: self.lines_of_code.to_string(),
        }
    }
}

pub struct RepoHeadInfoField {
    pub head_refs: HeadRefs,
}

impl InfoField for RepoHeadInfoField {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: FieldType::Head,
            value: self.head_refs.to_string(),
        }
    }
}

pub struct RepoVersionInfoField {
    pub version: String,
}

impl InfoField for RepoVersionInfoField {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: FieldType::Version,
            value: self.version.to_string(),
        }
    }
}

pub struct RepoContributorsInfoField {
    pub contributors: usize,
}

impl InfoField for RepoContributorsInfoField {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: FieldType::Contributors,
            value: self.contributors.to_string(),
        }
    }
}
