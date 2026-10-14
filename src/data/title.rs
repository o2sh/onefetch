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

fn get_git_username(repo: &Repository) -> String {
    repo.committer()
        .and_then(Result::ok)
        .map(|c| c.name.to_string())
        .unwrap_or_default()
}

fn get_git_version() -> String {
    let version = std::process::Command::new("git").arg("--version").output();

    match version {
        Ok(v) => String::from_utf8_lossy(&v.stdout).replace('\n', ""),
        Err(_) => String::new(),
    }
}
