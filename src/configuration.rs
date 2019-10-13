use git2::Repository;

use error::{Error, Result};

#[derive(Debug)]
pub struct Configuration {
    pub repository_name: String,
    pub repository_url: String,
}

pub fn get_configuration(dir: &str) -> Result<Configuration> {
    let repo = Repository::open(dir).map_err(|_| Error::NotGitRepo)?;
    let config = repo.config().map_err(|_| Error::NoGitData)?;
    let mut remote_url = String::new();
    let mut repository_name = String::new();
    let mut remote_upstream: Option<String> = None;

    for entry in &config.entries(None).unwrap() {
        let entry = entry.unwrap();
        match entry.name().unwrap() {
            "remote.origin.url" => remote_url = entry.value().unwrap().to_string(),
            "remote.upstream.url" => remote_upstream = Some(entry.value().unwrap().to_string()),
            _ => (),
        }
    }

    if let Some(url) = remote_upstream {
        remote_url = url.clone();
    }

    let url = remote_url.clone();
    let name_parts: Vec<&str> = url.split('/').collect();

    if !name_parts.is_empty() {
        repository_name = name_parts[name_parts.len() - 1].to_string();
    }

    if repository_name.contains(".git") {
        let repo_name = repository_name.clone();
        let parts: Vec<&str> = repo_name.split(".git").collect();
        repository_name = parts[0].to_string();
    }

    Ok(Configuration {
        repository_name: repository_name.clone(),
        repository_url: name_parts.join("/"),
    })
}
