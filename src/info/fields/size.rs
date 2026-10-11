use gix::Repository;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Size {
    pub repo_size: u64,
    pub file_count: u64,
}

impl Size {
    pub fn new(repo: &Repository) -> Self {
        let (repo_size, file_count) = get_repo_size(repo);
        Self {
            repo_size,
            file_count,
        }
    }
}

fn get_repo_size(repo: &Repository) -> (u64, u64) {
    match repo.index() {
        Ok(index) => {
            let repo_size = index.entries().iter().map(|e| e.stat.size as u64).sum();
            (repo_size, index.entries().len() as u64)
        }
        _ => (0, 0),
    }
}
