use crate::info::{
    git::Repo,
    info_field::{InfoField, InfoFieldValue, InfoType},
};

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
