use crate::git::{get_git_username, get_git_version};
use gix::Repository;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    pub git_username: String,
    pub git_version: String,
}

impl Title {
    pub fn new(repo: &Repository) -> Self {
        Self {
            git_username: get_git_username(repo),
            git_version: get_git_version(),
        }
    }
}
