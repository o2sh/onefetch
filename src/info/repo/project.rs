use crate::{
    cli::NumberFormat,
    info::{
        format_number,
        info_field::{InfoField, InfoType},
    },
};
use anyhow::Result;
use git_repository::{bstr::ByteSlice, Repository};
use onefetch_manifest::Manifest;
use serde::Serialize;
use std::ffi::OsStr;

#[derive(Serialize)]
pub struct ProjectInfo {
    pub repo_name: String,
    pub number_of_branches: String,
    pub number_of_tags: String,
}

impl ProjectInfo {
    pub fn new(
        repo: &Repository,
        repo_url: &str,
        manifest: Option<&Manifest>,
        number_format: Option<&NumberFormat>,
    ) -> Result<Self> {
        let repo_name = get_repo_name(repo_url, manifest)?;
        let number_of_branches = format_number(get_number_of_branches(repo)?, number_format);
        let number_of_tags = format_number(get_number_of_tags(repo)?, number_format);
        Ok(Self {
            repo_name,
            number_of_branches,
            number_of_tags,
        })
    }
}

fn get_repo_name(repo_url: &str, manifest: Option<&Manifest>) -> Result<String> {
    let url = git_repository::url::parse(repo_url.into())?;
    let path = git_repository::path::from_bstr(url.path.as_bstr());
    let repo_name = path
        .with_extension("")
        .file_name()
        .map(OsStr::to_string_lossy)
        .map(|s| s.into_owned())
        .unwrap_or_default();

    if repo_name.is_empty() {
        let repo_name_from_manifest = match manifest {
            Some(m) => m.name.clone(),
            None => String::new(),
        };
        Ok(repo_name_from_manifest)
    } else {
        Ok(repo_name)
    }
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
            let branches_str = match self.number_of_branches.as_str() {
                "0" => String::new(),
                "1" => "1 branch".into(),
                _ => format!("{} branches", self.number_of_branches),
            };

            let tags_str = match self.number_of_tags.as_str() {
                "0" => String::new(),
                "1" => "1 tag".into(),
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
        "Project".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_project_info() {
        let project_info = ProjectInfo {
            repo_name: "onefetch".to_string(),
            number_of_branches: "3".to_string(),
            number_of_tags: "2".to_string(),
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
            number_of_branches: "0".to_string(),
            number_of_tags: "0".to_string(),
        };

        assert_eq!(project_info.value(), "onefetch".to_string());
    }

    #[test]
    fn test_display_project_info_when_no_tags() {
        let project_info = ProjectInfo {
            repo_name: "onefetch".to_string(),
            number_of_branches: "3".to_string(),
            number_of_tags: "0".to_string(),
        };

        assert_eq!(project_info.value(), "onefetch (3 branches)".to_string());
    }

    #[test]
    fn test_display_project_info_when_no_branches() {
        let project_info = ProjectInfo {
            repo_name: "onefetch".to_string(),
            number_of_branches: "0".to_string(),
            number_of_tags: "2".to_string(),
        };

        assert_eq!(project_info.value(), "onefetch (2 tags)".to_string());
    }

    #[test]
    fn test_display_project_info_when_one_branche_one_tag() {
        let project_info = ProjectInfo {
            repo_name: "onefetch".to_string(),
            number_of_branches: "1".to_string(),
            number_of_tags: "1".to_string(),
        };

        assert_eq!(
            project_info.value(),
            "onefetch (1 branch, 1 tag)".to_string()
        );
    }

    #[test]
    fn test_get_repo_name_when_no_remote() -> Result<()> {
        let repo_name = get_repo_name("", None)?;
        assert!(repo_name.is_empty());

        Ok(())
    }

    #[test]
    fn test_display_project_info_when_no_repo_name() {
        let project_info = ProjectInfo {
            repo_name: "".to_string(),
            number_of_branches: "0".to_string(),
            number_of_tags: "0".to_string(),
        };

        assert!(project_info.value().is_empty());
    }
}
