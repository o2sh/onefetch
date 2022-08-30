use super::{
    git::Repo,
    info_field::{InfoField, InfoFieldValue, InfoType},
};
use anyhow::Result;
use serde::Serialize;

#[derive(Serialize)]
pub struct ProjectInfo {
    pub repo_name: String,
    pub number_of_tags: usize,
    pub number_of_branches: usize,
}

impl ProjectInfo {
    pub fn new(repo: &Repo) -> Result<Self> {
        let repo_name = repo.get_name()?;
        let number_of_tags = repo.get_number_of_tags()?;
        let number_of_branches = repo.get_number_of_branches()?;
        Ok(Self {
            repo_name,
            number_of_tags,
            number_of_branches,
        })
    }
}

impl std::fmt::Display for ProjectInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let branches_str = match self.number_of_branches {
            0 => String::new(),
            1 => String::from("1 branch"),
            _ => format!("{} branches", self.number_of_branches),
        };

        let tags_str = match self.number_of_tags {
            0 => String::new(),
            1 => String::from("1 tag"),
            _ => format!("{} tags", self.number_of_tags),
        };

        if tags_str.is_empty() && branches_str.is_empty() {
            write!(f, "{}", self.repo_name)
        } else if branches_str.is_empty() || tags_str.is_empty() {
            write!(f, "{} ({}{})", self.repo_name, tags_str, branches_str)
        } else {
            write!(f, "{} ({}, {})", self.repo_name, branches_str, tags_str)
        }
    }
}

impl InfoField for ProjectInfo {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: InfoType::Project,
            value: format!("{}", self),
        }
    }
    fn title(&self) -> String {
        String::from("Project")
    }
}
