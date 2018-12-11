extern crate colored;
extern crate git2;
extern crate license;
extern crate tokei;

use colored::Color;
use colored::*;
use git2::Repository;
use license::License;
use std::{
    convert::From,
    error,
    ffi::OsStr,
    fmt,
    fmt::Write,
    fs,
    process::{Command, Stdio},
    result,
    str::FromStr,
};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<error::Error>::from(format!($($tt)*))) }
}

type Result<T> = result::Result<T, Box<error::Error>>;

struct Info {
    project_name: String,
    language: Language,
    authors: Vec<String>,
    repo: String,
    number_of_lines: usize,
    license: String,
}

impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();
        let color = self.color();

        writeln!(
            buffer,
            "{}{}",
            "Project: ".color(color).bold(),
            self.project_name
        )?;
        writeln!(
            buffer,
            "{}{}",
            "Language: ".color(color).bold(),
            self.language
        )?;

        if !self.authors.is_empty() {
            let title = if self.authors.len() > 1 {
                "Authors: "
            } else {
                "Author: "
            };

            writeln!(buffer, "{}{}", title.color(color).bold(), self.authors[0])?;

            let title = " ".repeat(title.len());

            for author in self.authors.iter().skip(1) {
                writeln!(buffer, "{}{}", title.color(color).bold(), author)?;
            }
        }

        writeln!(buffer, "{}{}", "Repo: ".color(color).bold(), self.repo)?;
        writeln!(
            buffer,
            "{}{}",
            "Number of lines: ".color(color).bold(),
            self.number_of_lines
        )?;
        writeln!(
            buffer,
            "{}{}",
            "License: ".color(color).bold(),
            self.license
        )?;

        let logo = self.get_ascii();
        let mut lines = buffer.lines();
        let left_pad = logo.lines().map(|l| l.len()).max().unwrap_or(0);

        for a in logo.lines() {
            let b = match lines.next() {
                Some(line) => line,
                None => "",
            };

            writeln!(
                f,
                "{:width$} {}",
                a.color(color).bold(),
                b,
                width = left_pad
            )?;
        }

        Ok(())
    }
}

enum Language {
    C,
    Clojure,
    Cpp,
    Csharp,
    Go,
    Haskell,
    Java,
    Lisp,
    Lua,
    Python,
    R,
    Ruby,
    Rust,
    Scala,
    Shell,
    TypeScript,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Language::C => write!(f, "C"),
            Language::Clojure => write!(f, "Clojure"),
            Language::Cpp => write!(f, "C++"),
            Language::Csharp => write!(f, "C#"),
            Language::Go => write!(f, "Go"),
            Language::Haskell => write!(f, "Haskell"),
            Language::Java => write!(f, "Java"),
            Language::Lisp => write!(f, "Lisp"),
            Language::Lua => write!(f, "Lua"),
            Language::Python => write!(f, "Python"),
            Language::R => write!(f, "R"),
            Language::Ruby => write!(f, "Ruby"),
            Language::Rust => write!(f, "Rust"),
            Language::Scala => write!(f, "Scala"),
            Language::Shell => write!(f, "Shell"),
            Language::TypeScript => write!(f, "TypeScript"),
        }
    }
}

fn main() -> Result<()> {
    let tokei_langs = project_languages();
    let language = match get_dominant_language(&tokei_langs) {
        Some(language) => language,
        None => {
            return err!("Could not find any source code in this directory.");
        }
    };

    if !is_git_installed() {
        return err!("Could not execute git for project information.");
    }

    let authors = get_authors(3);
    let config: Configuration = match get_configuration() {
        Ok(config) => config,
        Err(_) => {
            return err!("Could not retrieve git configuration data");
        }
    };

    let license = match project_license() {
        Ok(l) => l,
        Err(_) => return err!("Could read directory ./"),
    };

    let info = Info {
        project_name: config.repository_name,
        language,
        authors,
        repo: config.repository_url,
        number_of_lines: get_total_loc(&tokei_langs),
        license,
    };

    println!("{}", info);
    Ok(())
}

fn project_languages() -> tokei::Languages {
    let mut languages = tokei::Languages::new();
    let required_languages = get_all_language_types();
    languages.get_statistics(&["."], vec![".git", "target"], Some(required_languages));
    languages
}

fn project_license() -> Result<String> {
    let output = fs::read_dir(".")?
        .filter_map(result::Result::ok)
        .map(|entry| entry.path())
        .filter(
            |entry| {
                entry.is_file()
                    && entry
                        .file_name()
                        .map(OsStr::to_string_lossy)
                        .unwrap_or("".into())
                        .starts_with("LICENSE")
            }, // TODO: multiple prefixes, like COPYING?
        )
        .map(|entry| license::Kind::from_str(&fs::read_to_string(entry).unwrap_or("".into())))
        .filter_map(result::Result::ok)
        .map(|license| license.name().to_string())
        .collect::<Vec<_>>()
        .join(", ");

    if output == "" {
        Ok("Unknown".into())
    } else {
        Ok(output)
    }
}

fn is_git_installed() -> bool {
    Command::new("git")
        .arg("--version")
        .stdout(Stdio::null())
        .status()
        .is_ok()
}

#[derive(Debug)]
struct Configuration {
    pub repository_name: String,
    pub repository_url: String,
}

fn get_configuration() -> Result<Configuration> {
    let repo = Repository::open("./")?;
    let config = repo.config()?;
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

    match remote_upstream {
        Some(url) => remote_url = url.clone(),
        None => (),
    };

    let url = remote_url.clone();
    let name_parts: Vec<&str> = url.split('/').collect();

    if name_parts.len() > 0 {
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

// Return first n most active commiters as authors within this project.
fn get_authors(n: usize) -> Vec<String> {
    use std::collections::HashMap;
    let output = Command::new("git")
        .arg("log")
        .arg("--format='%aN'")
        .output()
        .expect("Failed to execute git.");

    // create map for storing author name as a key and their commit count as value
    let mut authors = HashMap::new();
    let output = String::from_utf8_lossy(&output.stdout);
    for line in output.lines() {
        let commit_count = authors.entry(line.to_string()).or_insert(0);
        *commit_count += 1;
    }

    // sort authors by commit count where the one with most commit count is first
    let mut authors: Vec<(String, usize)> = authors.into_iter().collect();
    authors.sort_by_key(|(_, c)| *c);

    // truncate the vector so we only get the count of authors we specified as 'n'
    authors.truncate(n);

    // get only authors without their commit count
    // and string "'" prefix and suffix
    let authors: Vec<String> = authors
        .into_iter()
        .map(|(author, _)| author.trim_matches('\'').to_string())
        .collect();

    authors
}

/// Traverse current directory and search for dominant
/// language using tokei.
fn get_dominant_language(languages: &tokei::Languages) -> Option<Language> {
    languages
        .remove_empty()
        .iter()
        .max_by_key(|(_, v)| v.code)
        .map(|(k, _)| Language::from(**k))
}

fn get_total_loc(languages: &tokei::Languages) -> usize {
    languages
        .values()
        .collect::<Vec<&tokei::Language>>()
        .iter()
        .fold(0, |sum, val| sum + val.code)
}

/// Convert from tokei LanguageType to known Language type .
impl From<tokei::LanguageType> for Language {
    fn from(language: tokei::LanguageType) -> Self {
        match language {
            tokei::LanguageType::C => Language::C,
            tokei::LanguageType::Clojure => Language::Clojure,
            tokei::LanguageType::Cpp => Language::Cpp,
            tokei::LanguageType::CSharp => Language::Csharp,
            tokei::LanguageType::Go => Language::Go,
            tokei::LanguageType::Haskell => Language::Haskell,
            tokei::LanguageType::Java => Language::Java,
            tokei::LanguageType::Lisp => Language::Lisp,
            tokei::LanguageType::Lua => Language::Lua,
            tokei::LanguageType::Python => Language::Python,
            tokei::LanguageType::R => Language::R,
            tokei::LanguageType::Ruby => Language::Ruby,
            tokei::LanguageType::Rust => Language::Rust,
            tokei::LanguageType::Scala => Language::Scala,
            tokei::LanguageType::Sh => Language::Shell,
            tokei::LanguageType::TypeScript => Language::TypeScript,
            _ => unimplemented!(),
        }
    }
}

fn get_all_language_types() -> Vec<tokei::LanguageType> {
    vec![
        tokei::LanguageType::C,
        tokei::LanguageType::Clojure,
        tokei::LanguageType::Cpp,
        tokei::LanguageType::CSharp,
        tokei::LanguageType::Go,
        tokei::LanguageType::Haskell,
        tokei::LanguageType::Java,
        tokei::LanguageType::Lisp,
        tokei::LanguageType::Lua,
        tokei::LanguageType::Python,
        tokei::LanguageType::R,
        tokei::LanguageType::Ruby,
        tokei::LanguageType::Rust,
        tokei::LanguageType::Scala,
        tokei::LanguageType::Sh,
        tokei::LanguageType::TypeScript,
    ]
}

impl Info {
    pub fn get_ascii(&self) -> &str {
        match self.language {
            Language::C => include_str!("../resources/c.ascii"),
            Language::Clojure => include_str!("../resources/clojure.ascii"),
            Language::Cpp => include_str!("../resources/cpp.ascii"),
            Language::Csharp => include_str!("../resources/csharp.ascii"),
            Language::Go => include_str!("../resources/go.ascii"),
            Language::Haskell => include_str!("../resources/haskell.ascii"),
            Language::Java => include_str!("../resources/java.ascii"),
            Language::Lisp => include_str!("../resources/lisp.ascii"),
            Language::Lua => include_str!("../resources/lua.ascii"),
            Language::Python => include_str!("../resources/python.ascii"),
            Language::R => include_str!("../resources/r.ascii"),
            Language::Ruby => include_str!("../resources/ruby.ascii"),
            Language::Rust => include_str!("../resources/rust.ascii"),
            Language::Scala => include_str!("../resources/scala.ascii"),
            Language::Shell => include_str!("../resources/shell.ascii"),
            Language::TypeScript => include_str!("../resources/typescript.ascii"),
            // _ => include_str!("../resources/unknown.ascii"),
        }
    }

    fn color(&self) -> Color {
        match self.language {
            Language::C => Color::Cyan,
            Language::Clojure => Color::Cyan,
            Language::Cpp => Color::Yellow,
            Language::Csharp => Color::White,
            Language::Go => Color::White,
            Language::Haskell => Color::Cyan,
            Language::Java => Color::Green,
            Language::Lisp => Color::Yellow,
            Language::Lua => Color::Blue,
            Language::Python => Color::Magenta,
            Language::R => Color::Blue,
            Language::Ruby => Color::Magenta,
            Language::Rust => Color::Cyan,
            Language::Scala => Color::Blue,
            Language::Shell => Color::Green,
            Language::TypeScript => Color::Cyan,
        }
    }
}
