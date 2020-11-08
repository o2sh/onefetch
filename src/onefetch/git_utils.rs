use crate::onefetch::{commit_info::CommitInfo, error::*};
use git2::{Repository, RepositoryOpenFlags};
use regex::Regex;
use std::path::Path;

pub fn get_repo_work_dir(repo: &Repository) -> Result<String> {
    if let Some(workdir) = work_dir(&repo)?.to_str() {
        Ok(workdir.to_string())
    } else {
        Err("invalid workdir".into())
    }
}

fn work_dir(repo: &Repository) -> Result<&Path> {
    repo.workdir().ok_or_else(|| "unable to query workdir".into())
}

pub fn get_repo_name_and_url(repo: &Repository) -> Result<(String, String)> {
    let config = repo.config()?;
    let mut remote_url = String::new();
    let mut repository_name = String::new();

    let remote_regex = Regex::new(r"remote\.[a-zA-Z0-9]+\.url").unwrap();

    for entry in &config.entries(None).unwrap() {
        let entry = entry?;
        if remote_regex.is_match(entry.name().unwrap()) {
            remote_url = entry.value().unwrap().to_string()
        };
    }

    let name_parts: Vec<&str> = remote_url.split('/').collect();

    if !name_parts.is_empty() {
        let mut i = 1;
        while repository_name.is_empty() && i <= name_parts.len() {
            repository_name = name_parts[name_parts.len() - i].to_string();
            i += 1;
        }
    }

    if repository_name.contains(".git") {
        let repo_name = repository_name.clone();
        let parts: Vec<&str> = repo_name.split(".git").collect();
        repository_name = parts[0].to_string();
    }

    Ok((repository_name, remote_url))
}

pub fn get_current_commit_info(repo: &Repository) -> Result<CommitInfo> {
    let head = repo.head()?;
    let head_oid = head.target().ok_or("")?;
    let refs = repo.references()?;
    let refs_info = refs
        .filter_map(|reference| match reference {
            Ok(reference) => match (reference.target(), reference.shorthand()) {
                (Some(oid), Some(shorthand)) if oid == head_oid => Some(if reference.is_tag() {
                    String::from("tags/") + shorthand
                } else {
                    String::from(shorthand)
                }),
                _ => None,
            },
            Err(_) => None,
        })
        .collect::<Vec<String>>();
    Ok(CommitInfo::new(head_oid, refs_info))
}

pub fn is_valid_repo(repo_path: &str) -> Result<bool> {
    let repo = Repository::open_ext(repo_path, RepositoryOpenFlags::empty(), Vec::<&Path>::new());

    Ok(repo.is_ok() && !repo?.is_bare())
}
