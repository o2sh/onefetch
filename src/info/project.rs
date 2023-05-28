use crate::{
    cli::NumberSeparator,
    info::{format_number, utils::info_field::InfoField},
};
use anyhow::Result;
use gix::{bstr::ByteSlice, Repository};
use onefetch_manifest::Manifest;
use serde::Serialize;
use std::ffi::OsStr;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInfo {
    pub repo_name: String,
    pub number_of_branches: usize,
    pub number_of_tags: usize,
    #[serde(skip_serializing)]
    number_separator: NumberSeparator,
}

impl ProjectInfo {
    pub fn new(
        repo: &Repository,
        repo_url: &str,
        manifest: Option<&Manifest>,
        number_separator: NumberSeparator,
    ) -> Result<Self> {
        let repo_name = get_repo_name(repo_url, manifest)?;
        let number_of_branches = get_number_of_branches(repo)?;
        let number_of_tags = get_number_of_tags(repo)?;
        Ok(Self {
            repo_name,
            number_of_branches,
            number_of_tags,
            number_separator,
        })
    }
}

fn get_repo_name(repo_url: &str, manifest: Option<&Manifest>) -> Result<String> {
    if repo_url.is_empty() {
        return Ok(String::default());
    }
    let url = gix::url::parse(repo_url.into())?;
    let path = gix::path::from_bstr(url.path.as_bstr());
    let repo_name = path
        .with_extension("")
        .file_name()
        .map(OsStr::to_string_lossy)
        .map(std::borrow::Cow::into_owned)
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
    number_of_branches = number_of_branches.saturating_sub(1); //Exclude origin/HEAD -> origin/main
    Ok(number_of_branches)
}

impl std::fmt::Display for ProjectInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.repo_name.is_empty() {
            Ok(())
        } else {
            let branches_str = match self.number_of_branches {
                0 => String::new(),
                1 => "1 branch".into(),
                _ => format!(
                    "{} branches",
                    format_number(&self.number_of_branches, self.number_separator)
                ),
            };

            let tags_str = match self.number_of_tags {
                0 => String::new(),
                1 => "1 tag".into(),
                _ => format!(
                    "{} tags",
                    format_number(&self.number_of_tags, self.number_separator)
                ),
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

#[typetag::serialize]
impl InfoField for ProjectInfo {
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
            number_of_branches: 3,
            number_of_tags: 2,
            number_separator: NumberSeparator::Plain,
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
            number_separator: NumberSeparator::Plain,
        };

        assert_eq!(project_info.value(), "onefetch".to_string());
    }

    #[test]
    fn test_display_project_info_when_no_tags() {
        let project_info = ProjectInfo {
            repo_name: "onefetch".to_string(),
            number_of_branches: 3,
            number_of_tags: 0,
            number_separator: NumberSeparator::Plain,
        };

        assert_eq!(project_info.value(), "onefetch (3 branches)".to_string());
    }

    #[test]
    fn test_display_project_info_when_no_branches() {
        let project_info = ProjectInfo {
            repo_name: "onefetch".to_string(),
            number_of_branches: 0,
            number_of_tags: 2,
            number_separator: NumberSeparator::Plain,
        };

        assert_eq!(project_info.value(), "onefetch (2 tags)".to_string());
    }

    #[test]
    fn test_display_project_info_when_one_branch_one_tag() {
        let project_info = ProjectInfo {
            repo_name: "onefetch".to_string(),
            number_of_branches: 1,
            number_of_tags: 1,
            number_separator: NumberSeparator::Plain,
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
            repo_name: String::new(),
            number_of_branches: 0,
            number_of_tags: 0,
            number_separator: NumberSeparator::Plain,
        };

        assert!(project_info.value().is_empty());
    }
}
