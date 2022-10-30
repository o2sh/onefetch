use crate::info::info_field::{InfoField, InfoType};
use anyhow::Result;
use git_repository::{bstr::ByteSlice, Repository};
use serde::Serialize;
use std::ffi::OsStr;

#[derive(Serialize)]
pub struct ProjectInfo {
    pub repo_name: String,
    pub number_of_branches: usize,
    pub number_of_tags: usize,
}

impl ProjectInfo {
    pub fn new(repo: &Repository, repo_url: &str) -> Result<Self> {
        let repo_name = get_repo_name(repo_url)?.unwrap_or_default();
        let number_of_branches = get_number_of_branches(repo)?;
        let number_of_tags = get_number_of_tags(repo)?;
        Ok(Self {
            repo_name,
            number_of_branches,
            number_of_tags,
        })
    }
}

fn get_repo_name(repo_url: &str) -> Result<Option<String>> {
    let url = git_repository::url::parse(repo_url.into())?;
    let path = git_repository::path::from_bstr(url.path.as_bstr());
    let repo_name = path
        .with_extension("")
        .file_name()
        .map(OsStr::to_string_lossy)
        .map(|s| s.into_owned());
    Ok(repo_name)
}

// This collects the repo size excluding .git
fn get_number_of_tags(repo: &Repository) -> Result<usize> {
    Ok(repo.references()?.tags()?.count())
}

fn get_number_of_branches(repo: &Repository) -> Result<usize> {
    let mut number_of_branches = repo.references()?.remote_branches()?.count();
    if number_of_branches > 0 {
        //Exclude origin/HEAD -> origin/main
        number_of_branches -= 1;
    }
    Ok(number_of_branches)
}

impl std::fmt::Display for ProjectInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.repo_name.is_empty() {
            Ok(())
        } else {
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
}

impl InfoField for ProjectInfo {
    const TYPE: InfoType = InfoType::Project;

    fn value(&self) -> String {
        self.to_string()
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
            project_info.value(),
            "onefetch (3 branches, 2 tags)".to_string()
        );
    }

    #[test]
    fn test_display_project_info_when_no_branches_no_tags() {
        let project_info = ProjectInfo {
            repo_name: "onefetch".to_string(),
            number_of_branches: 0,
            number_of_tags: 0,
        };

        assert_eq!(project_info.value(), "onefetch".to_string());
    }

    #[test]
    fn test_display_project_info_when_no_tags() {
        let project_info = ProjectInfo {
            repo_name: "onefetch".to_string(),
            number_of_branches: 3,
            number_of_tags: 0,
        };

        assert_eq!(project_info.value(), "onefetch (3 branches)".to_string());
    }

    #[test]
    fn test_display_project_info_when_no_branches() {
        let project_info = ProjectInfo {
            repo_name: "onefetch".to_string(),
            number_of_branches: 0,
            number_of_tags: 2,
        };

        assert_eq!(project_info.value(), "onefetch (2 tags)".to_string());
    }

    #[test]
    fn test_display_project_info_when_one_branche_one_tag() {
        let project_info = ProjectInfo {
            repo_name: "onefetch".to_string(),
            number_of_branches: 1,
            number_of_tags: 1,
        };

        assert_eq!(
            project_info.value(),
            "onefetch (1 branch, 1 tag)".to_string()
        );
    }

    #[test]
    fn test_get_repo_name_when_no_remote() -> Result<()> {
        let repo_name = get_repo_name("")?;
        assert!(repo_name.is_none());

        Ok(())
    }

    #[test]
    fn test_display_project_info_when_no_repo_name() {
        let project_info = ProjectInfo {
            repo_name: "".to_string(),
            number_of_branches: 0,
            number_of_tags: 0,
        };

        assert!(project_info.value().is_empty());
    }
}
