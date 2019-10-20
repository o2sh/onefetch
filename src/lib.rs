extern crate colored;
extern crate git2;
extern crate strum;
extern crate strum_macros;

use colored::*;
use git2::Repository;
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs,
    process::{Command, Stdio},
    result,
};

mod commit_info;
mod error;
mod language;
use commit_info::CommitInfo;
use error::{Error, Result};
use language::Language;

fn get_languages_stat(languages: &tokei::Languages) -> Option<HashMap<Language, f64>> {
    let mut stats = HashMap::new();

    let sum_language_code: usize = languages.iter().map(|(_, v)| v.code).sum();

    if sum_language_code == 0 {
        None
    } else {
        for (k, v) in languages.iter() {
            let code = v.code as f64;
            stats.insert(
                Language::from(*k),
                (code / sum_language_code as f64) * 100.00,
            );
        }
        Some(stats)
    }
}

fn project_license(dir: &str) -> Result<String> {
    let output = fs::read_dir(dir)
        .map_err(|_| Error::ReadDirectory)?
        .filter_map(result::Result::ok)
        .map(|entry| entry.path())
        .filter(
            |entry| {
                entry.is_file()
                    && entry
                        .file_name()
                        .map(OsStr::to_string_lossy)
                        .unwrap_or_else(|| "".into())
                        .starts_with("LICENSE")
            }, // TODO: multiple prefixes, like COPYING?
        )
        .map(|entry| {
            license::Kind::from_str(&fs::read_to_string(entry).unwrap_or_else(|_| "".into()))
        })
        .filter_map(result::Result::ok)
        .map(|license| license.name().to_string())
        .collect::<Vec<_>>()
        .join(", ");

    if output == "" {
        Ok("??".into())
    } else {
        Ok(output)
    }
}

fn get_version(dir: &str) -> Result<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .arg("describe")
        .arg("--abbrev=0")
        .arg("--tags")
        .output()
        .expect("Failed to execute git.");

    let output = String::from_utf8_lossy(&output.stdout);

    if output == "" {
        Ok("??".into())
    } else {
        Ok(output.to_string().replace('\n', ""))
    }
}

fn get_last_change(dir: &str) -> Result<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .arg("log")
        .arg("-1")
        .arg("--format=%cr")
        .output()
        .expect("Failed to execute git.");

    let output = String::from_utf8_lossy(&output.stdout);

    if output == "" {
        Ok("??".into())
    } else {
        Ok(output.to_string().replace('\n', ""))
    }
}

fn get_commits(dir: &str) -> Result<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .arg("rev-list")
        .arg("--count")
        .arg("HEAD")
        .output()
        .expect("Failed to execute git.");

    let output = String::from_utf8_lossy(&output.stdout);

    if output == "" {
        Ok("0".into())
    } else {
        Ok(output.to_string().replace('\n', ""))
    }
}

fn get_packed_size(dir: &str) -> Result<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .arg("count-objects")
        .arg("-vH")
        .output()
        .expect("Failed to execute git.");

    let output = String::from_utf8_lossy(&output.stdout);
    let lines = output.to_string();
    let size_line = lines
        .split("\n")
        .find(|line| line.starts_with("size-pack:"));

    let repo_size = match size_line {
        None => "??",
        Some(size_str) => &(size_str[11..]),
    };

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
        let files_list = lines.split("\n");
        let mut files_count: u128 = 0;
        for _file in files_list {
            files_count += 1;
        }
        files_count -= 1; // As splitting giving one line extra(blank).
        let res = repo_size.to_owned() + &(" (") + &(files_count.to_string()) + &(" files)");
        Ok(res.into())
    } else {
        let res = repo_size;
        Ok(res.into())
    }
}

fn is_git_installed() -> bool {
    Command::new("git")
        .arg("--version")
        .stdout(Stdio::null())
        .status()
        .is_ok()
}

// Return first n most active commiters as authors within this project.
fn get_authors(dir: &str, n: usize) -> Vec<(String, usize, usize)> {
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .arg("log")
        .arg("--format='%aN'")
        .output()
        .expect("Failed to execute git.");

    // create map for storing author name as a key and their commit count as value
    let mut authors = HashMap::new();
    let mut total_commits = 0;
    let output = String::from_utf8_lossy(&output.stdout);
    for line in output.lines() {
        let commit_count = authors.entry(line.to_string()).or_insert(0);
        *commit_count += 1;
        total_commits += 1;
    }

    // sort authors by commit count where the one with most commit count is first
    let mut authors: Vec<(String, usize)> = authors.into_iter().collect();
    authors.sort_by_key(|(_, c)| *c);
    authors.reverse();

    // truncate the vector so we only get the count of authors we specified as 'n'
    authors.truncate(n);

    // get only authors without their commit count
    // and string "'" prefix and suffix
    let authors: Vec<(String, usize, usize)> = authors
        .into_iter()
        .map(|(author, count)| {
            (
                author.trim_matches('\'').to_string(),
                count,
                count * 100 / total_commits,
            )
        })
        .collect();

    authors
}

fn get_current_commit_info(dir: &str) -> Result<CommitInfo> {
    let repo = Repository::open(dir).map_err(|_| Error::NotGitRepo)?;
    let head = repo.head().map_err(|_| Error::ReferenceInfoError)?;
    let head_oid = head.target().ok_or(Error::ReferenceInfoError)?;
    let refs = repo.references().map_err(|_| Error::ReferenceInfoError)?;
    let refs_info = refs
        .into_iter()
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

fn get_total_loc(languages: &tokei::Languages) -> usize {
    languages
        .values()
        .collect::<Vec<&tokei::Language>>()
        .iter()
        .fold(0, |sum, val| sum + val.code)
}

fn get_creation_time() -> Option<String> {
    let output = Command::new("git")
        .arg("log")
        .arg("--reverse")
        .arg("--pretty=oneline")
        .arg("--format=\"%ar\"")
        .output()
        .expect("Failed to execute git.");

    let output = String::from_utf8_lossy(&output.stdout);

    match output.lines().next() {
        Some(val) => Some(val.to_string().replace('"', "")),
        None => None,
    }
}

/// Convert from tokei LanguageType to known Language type .
impl From<tokei::LanguageType> for Language {
    fn from(language: tokei::LanguageType) -> Self {
        match language {
            tokei::LanguageType::Assembly => Language::Assembly,
            tokei::LanguageType::C => Language::C,
            tokei::LanguageType::Clojure => Language::Clojure,
            tokei::LanguageType::CoffeeScript => Language::CoffeeScript,
            tokei::LanguageType::Cpp => Language::Cpp,
            tokei::LanguageType::CSharp => Language::Csharp,
            tokei::LanguageType::Css => Language::CSS,
            tokei::LanguageType::Dart => Language::Dart,
            tokei::LanguageType::Elm => Language::Elm,
            tokei::LanguageType::Erlang => Language::Erlang,
            tokei::LanguageType::Forth => Language::Forth,
            tokei::LanguageType::Go => Language::Go,
            tokei::LanguageType::Haskell => Language::Haskell,
            tokei::LanguageType::Html => Language::HTML,
            tokei::LanguageType::Idris => Language::Idris,
            tokei::LanguageType::Java => Language::Java,
            tokei::LanguageType::Kotlin => Language::Kotlin,
            tokei::LanguageType::Lisp => Language::Lisp,
            tokei::LanguageType::Lua => Language::Lua,
            tokei::LanguageType::Nim => Language::Nim,
            tokei::LanguageType::ObjectiveC => Language::ObjectiveC,
            tokei::LanguageType::PureScript => Language::PureScript,
            tokei::LanguageType::Python => Language::Python,
            tokei::LanguageType::R => Language::R,
            tokei::LanguageType::Ruby => Language::Ruby,
            tokei::LanguageType::Rust => Language::Rust,
            tokei::LanguageType::Scala => Language::Scala,
            tokei::LanguageType::Sh => Language::Shell,
            tokei::LanguageType::Swift => Language::Swift,
            tokei::LanguageType::Tcl => Language::Tcl,
            tokei::LanguageType::TypeScript => Language::TypeScript,
            tokei::LanguageType::JavaScript => Language::JavaScript,
            tokei::LanguageType::Vue => Language::Vue,
            tokei::LanguageType::Perl => Language::Perl,
            tokei::LanguageType::Php => Language::Php,
            tokei::LanguageType::Zig => Language::Zig,
            _ => unimplemented!(),
        }
    }
}

fn get_all_language_types() -> Vec<tokei::LanguageType> {
    vec![
        tokei::LanguageType::Assembly,
        tokei::LanguageType::C,
        tokei::LanguageType::Clojure,
        tokei::LanguageType::CoffeeScript,
        tokei::LanguageType::Cpp,
        tokei::LanguageType::CSharp,
        tokei::LanguageType::Css,
        tokei::LanguageType::Dart,
        tokei::LanguageType::Elm,
        tokei::LanguageType::Erlang,
        tokei::LanguageType::Forth,
        tokei::LanguageType::Go,
        tokei::LanguageType::Haskell,
        tokei::LanguageType::Html,
        tokei::LanguageType::Idris,
        tokei::LanguageType::Java,
        tokei::LanguageType::Kotlin,
        tokei::LanguageType::Lisp,
        tokei::LanguageType::Lua,
        tokei::LanguageType::Nim,
        tokei::LanguageType::ObjectiveC,
        tokei::LanguageType::PureScript,
        tokei::LanguageType::Python,
        tokei::LanguageType::R,
        tokei::LanguageType::Ruby,
        tokei::LanguageType::Rust,
        tokei::LanguageType::Scala,
        tokei::LanguageType::Sh,
        tokei::LanguageType::Swift,
        tokei::LanguageType::Tcl,
        tokei::LanguageType::TypeScript,
        tokei::LanguageType::JavaScript,
        tokei::LanguageType::Vue,
        tokei::LanguageType::Perl,
        tokei::LanguageType::Php,
        tokei::LanguageType::Zig,
    ]
}

/// Returns the true length of a string after substracting the {n}
/// color declarations.
fn true_len(line: &str) -> usize {
    line.split('{')
        .fold(String::new(), |mut acc, s| {
            if s.len() > 2 {
                acc.push_str(&s.chars().skip(2).collect::<String>());
            } else {
                acc.push_str(s);
            }
            acc
        })
        .len()
}

fn count_newlines(s: &str) -> usize {
    bytecount::count(s.as_bytes(), b'\n')
}

/// Transforms a string with color format into one with proper
/// escape characters for color display.
///
/// Colors are specified with {0}, {1}... where the number represents
/// the nth element in the colors Vec provided to the function.
/// If there are more colors in the ascii than in the Vec it
/// defaults to white.
/// The usize in the tuple refers to the extra padding needed
/// which comes from the added escape characters.
fn colorize_str(line: &str, colors: Vec<Color>) -> (String, usize) {
    // Extract colors from string coded with {n}
    let mut colors_in_str: Vec<Color> = line.split('{').fold(Vec::new(), |mut acc, s| {
        if s.len() > 2 {
            let i = s.chars().nth(0).unwrap_or('0').to_digit(10).unwrap_or(0);
            acc.push(*colors.get(i as usize).unwrap_or(&Color::White));
        }
        acc
    });

    if colors_in_str.is_empty() {
        colors_in_str.push(match colors.get(0) {
            Some(&c) => c,
            None => Color::White,
        });
    }

    let mut colors_iter = colors_in_str.iter();

    let out_str = line.split('{').fold(String::new(), |mut acc, s| {
        if s.len() > 2 {
            let s: String = s.chars().skip(2).collect();
            let c = match colors_iter.next() {
                Some(&c) => c,
                None => Color::White,
            };
            acc.push_str(&format!("{}", s.color(c)));
        }
        acc
    });
    (out_str, colors_in_str.len() * 9)
}
