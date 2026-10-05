use crate::info::display_options::DisplayOptions;
use crate::info::{info_field::InfoField, text::Line};
use anyhow::Result;
use gix::{Repository, bstr::ByteSlice};
use onefetch_manifest::Manifest;
use serde::Serialize;
use std::ffi::OsStr;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInfo {
    pub repo_name: String,
    pub number_of_branches: usize,
    pub number_of_tags: usize,
}

impl ProjectInfo {
    pub fn new(repo: &Repository, repo_url: &str, manifest: Option<&Manifest>) -> Result<Self> {
        let repo_name = get_repo_name(repo_url, manifest)?;
        let number_of_branches = get_number_of_branches(repo)?;
        let number_of_tags = get_number_of_tags(repo)?;
        Ok(Self {
            repo_name,
            number_of_branches,
            number_of_tags,
        })
    }
}

fn get_repo_name(repo_url: &str, manifest: Option<&Manifest>) -> Result<String> {
    if repo_url.is_empty() {
        return Ok(String::default());
    }
    let url = gix::url::parse(repo_url)?;
    let path = gix::path::from_bstr(url.path.as_bstr());
    let repo_name = path
        .with_extension("")
        .file_name()
        .map(OsStr::to_string_lossy)
        .map(std::borrow::Cow::into_owned)
        .unwrap_or_default();

    if repo_name.is_empty() {
        let repo_name_from_manifest = manifest.and_then(|m| m.name.clone()).unwrap_or_default();
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

#[typetag::serialize]
impl InfoField for ProjectInfo {
    fn value(&self, options: &DisplayOptions) -> Vec<Line> {
        if self.repo_name.is_empty() {
            return Vec::new();
        }

        let branches = match self.number_of_branches {
            0 => String::new(),
            1 => "1 branch".into(),
            _ => format!("{} branches", options.number(&self.number_of_branches)),
        };

        let tags = match self.number_of_tags {
            0 => String::new(),
            1 => "1 tag".into(),
            _ => format!("{} tags", options.number(&self.number_of_tags)),
        };

        let project = if tags.is_empty() && branches.is_empty() {
            self.repo_name.clone()
        } else if branches.is_empty() || tags.is_empty() {
            format!("{} ({}{})", self.repo_name, tags, branches)
        } else {
            format!("{} ({}, {})", self.repo_name, branches, tags)
        };
        vec![Line::from(project)]
    }

    fn key(&self) -> String {
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
        };

        assert_eq!(
            project_info.value(&DisplayOptions::default()),
            vec![Line::from("onefetch (3 branches, 2 tags)")]
        );
    }

    #[test]
    fn test_display_project_info_when_no_branches_no_tags() {
        let project_info = ProjectInfo {
            repo_name: "onefetch".to_string(),
            number_of_branches: 0,
            number_of_tags: 0,
        };

        assert_eq!(
            project_info.value(&DisplayOptions::default()),
            vec![Line::from("onefetch")]
        );
    }

    #[test]
    fn test_display_project_info_when_no_tags() {
        let project_info = ProjectInfo {
            repo_name: "onefetch".to_string(),
            number_of_branches: 3,
            number_of_tags: 0,
        };

        assert_eq!(
            project_info.value(&DisplayOptions::default()),
            vec![Line::from("onefetch (3 branches)")]
        );
    }

    #[test]
    fn test_display_project_info_when_no_branches() {
        let project_info = ProjectInfo {
            repo_name: "onefetch".to_string(),
            number_of_branches: 0,
            number_of_tags: 2,
        };

        assert_eq!(
            project_info.value(&DisplayOptions::default()),
            vec![Line::from("onefetch (2 tags)")]
        );
    }

    #[test]
    fn test_display_project_info_when_one_branch_one_tag() {
        let project_info = ProjectInfo {
            repo_name: "onefetch".to_string(),
            number_of_branches: 1,
            number_of_tags: 1,
        };

        assert_eq!(
            project_info.value(&DisplayOptions::default()),
            vec![Line::from("onefetch (1 branch, 1 tag)")]
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
        };

        assert!(project_info.value(&DisplayOptions::default()).is_empty());
    }
}
