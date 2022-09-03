use crate::info::{
    git::Repo,
    info_field::{InfoField, InfoFieldValue, InfoType},
};
use anyhow::Result;
use serde::Serialize;

#[derive(Serialize)]
pub struct ProjectInfo {
    pub repo_name: String,
    pub number_of_branches: usize,
    pub number_of_tags: usize,
}

impl ProjectInfo {
    pub fn new(repo: &Repo) -> Result<Self> {
        let repo_name = repo.get_name()?;
        let number_of_branches = repo.get_number_of_branches()?;
        let number_of_tags = repo.get_number_of_tags()?;
        Ok(Self {
            repo_name,
            number_of_branches,
            number_of_tags,
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
            value: self.to_string(),
        }
    }
    fn title(&self) -> String {
        String::from("Project")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_project_info() {
        let project_info = ProjectInfo {
            repo_name: "onefetch".to_string(),
            number_of_branches: 3,
            number_of_tags: 2,
        };

        assert_eq!(
            project_info.value().value,
            "onefetch (3 branches, 2 tags)".to_string()
        );
    }

    #[test]
    fn test_display_project_info_no_branches_no_tags() {
        let project_info = ProjectInfo {
            repo_name: "onefetch".to_string(),
            number_of_branches: 0,
            number_of_tags: 0,
        };

        assert_eq!(project_info.value().value, "onefetch".to_string());
    }

    #[test]
    fn test_display_project_info_no_tags() {
        let project_info = ProjectInfo {
            repo_name: "onefetch".to_string(),
            number_of_branches: 3,
            number_of_tags: 0,
        };

        assert_eq!(
            project_info.value().value,
            "onefetch (3 branches)".to_string()
        );
    }

    #[test]
    fn test_display_project_info_no_branches() {
        let project_info = ProjectInfo {
            repo_name: "onefetch".to_string(),
            number_of_branches: 0,
            number_of_tags: 2,
        };

        assert_eq!(project_info.value().value, "onefetch (2 tags)".to_string());
    }

    #[test]
    fn test_display_project_info_one_branche_one_tag() {
        let project_info = ProjectInfo {
            repo_name: "onefetch".to_string(),
            number_of_branches: 1,
            number_of_tags: 1,
        };

        assert_eq!(
            project_info.value().value,
            "onefetch (1 branch, 1 tag)".to_string()
        );
    }
}
