use crate::onefetch::error::*;
use std::process::Command;

pub fn get_git_history(dir: &str, no_merges: bool) -> Result<Vec<String>> {
    let mut args = vec!["-C", dir, "log"];
    if no_merges {
        args.push("--no-merges");
    }

    args.push("--pretty=%cr\t%ae\t%an");

    let output = Command::new("git").args(args).output()?;

    let output = String::from_utf8_lossy(&output.stdout);
    Ok(output.lines().map(|x| x.to_string()).collect::<Vec<_>>())
}

pub fn get_authors(git_history: &[String], n: usize) -> Vec<(String, usize, usize)> {
    let mut authors = std::collections::HashMap::new();
    let mut author_name_by_email = std::collections::HashMap::new();
    let mut total_commits = 0;
    for line in git_history {
        let author_email = line.split('\t').collect::<Vec<_>>()[1].to_string();
        let author_name = line.split('\t').collect::<Vec<_>>()[2].to_string();
        let commit_count = authors.entry(author_email.to_string()).or_insert(0);
        author_name_by_email.entry(author_email.to_string()).or_insert(author_name);
        *commit_count += 1;
        total_commits += 1;
    }

    let mut authors: Vec<(String, usize)> = authors.into_iter().collect();
    authors.sort_by(|(_, a_count), (_, b_count)| b_count.cmp(a_count));

    authors.truncate(n);

    let authors: Vec<(String, usize, usize)> = authors
        .into_iter()
        .map(|(author, count)| {
            (
                author_name_by_email.get(&author).unwrap().trim_matches('\'').to_string(),
                count,
                count * 100 / total_commits,
            )
        })
        .collect();

    authors
}

pub fn get_date_of_last_commit(git_history: &[String]) -> Result<String> {
    let last_commit = git_history.first();

    let output = match last_commit {
        Some(date) => date.split('\t').collect::<Vec<_>>()[0].to_string(),
        None => "".into(),
    };

    Ok(output)
}

pub fn get_creation_date(git_history: &[String]) -> Result<String> {
    let first_commit = git_history.last();

    let output = match first_commit {
        Some(creation_time) => creation_time.split('\t').collect::<Vec<_>>()[0].to_string(),
        None => "".into(),
    };

    Ok(output)
}

pub fn get_number_of_commits(git_history: &[String]) -> String {
    let number_of_commits = git_history.len();
    number_of_commits.to_string()
}

pub fn get_packed_size(repo_size: String, files_count: Option<u64>) -> Result<String> {
    match files_count {
        Some(files_count) => {
            let res = format!("{} ({} files)", repo_size, files_count.to_string());
            Ok(res)
        }
        None => {
            let res = repo_size;
            Ok(res.into())
        }
    }
}

pub fn get_repo_size(dir: &str) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .arg("count-objects")
        .arg("-vH")
        .output()
        .expect("Failed to execute git.");

    let output = String::from_utf8_lossy(&output.stdout);
    let lines = output.to_string();
    let size_line = lines.split('\n').find(|line| line.starts_with("size-pack:"));

    let repo_size = match size_line {
        None => String::new(),
        Some(size_str) => String::from(&(size_str[11..])),
    };
    repo_size
}

pub fn get_files_count(dir: &str) -> Option<u64> {
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .arg("ls-files")
        .output()
        .expect("Failed to execute git.");
    // To check if command executed successfully or not
    let error = &output.stderr;

    if error.is_empty() {
        let output = String::from_utf8_lossy(&output.stdout);

        let lines = output.to_string();
        let files_list = lines.split('\n');
        let mut files_count: u64 = 0;
        for _file in files_list {
            files_count += 1;
        }
        files_count -= 1; // As splitting giving one line extra(blank).
        Some(files_count)
    } else {
        None
    }
}
