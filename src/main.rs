extern crate bytecount;
extern crate colored;
extern crate git2;
extern crate license;
extern crate tokei;
#[macro_use]
extern crate clap;
extern crate strum;
#[macro_use]
extern crate strum_macros;

#[cfg(target = "windows")]
extern crate ansi_term;

use colored::Color;
use colored::*;
use git2::{Repository, Oid};
use license::License;
use clap::{App, Arg};
use std::{
    collections::HashMap,
    convert::From,
    ffi::OsStr,
    fmt,
    fmt::Write,
    fs,
    process::{Command, Stdio},
    result,
    str::FromStr,
};
use strum::{IntoEnumIterator, EnumCount};

mod ascii_art;
use ascii_art::AsciiArt;

type Result<T> = result::Result<T, Error>;

struct Info {
    project_name: String,
    current_commit: CommitInfo,
    version: String,
    creation_date: String,
    dominant_language: Language,
    languages: Vec<(Language, f64)>,
    authors: Vec<(String, usize, usize)>,
    last_change: String,
    repo: String,
    commits: String,
    repo_size: String,
    number_of_lines: usize,
    license: String,
    custom_logo: Language,
    custom_colors: Vec<String>,
    disable_fields: InfoFieldOn,
}

impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();
        let color = match self.colors().get(0) {
            Some(&c) => c,
            None => Color::White,
        };

        if !self.disable_fields.project {
            writeln!(
                buffer,
                "{}{}",
                "Project: ".color(color).bold(),
                self.project_name
            )?;
        }

        if !self.disable_fields.head {
            writeln!(
                buffer,
                "{}{}",
                "HEAD: ".color(color).bold(),
                self.current_commit
            )?;
        }

        if !self.disable_fields.version {
            writeln!(
                buffer,
                "{}{}",
                "Version: ".color(color).bold(),
                self.version
            )?;
        }

        if !self.disable_fields.created {
            writeln!(
                buffer,
                "{}{}",
                "Created: ".color(color).bold(),
                self.creation_date
            )?;
        }

        if !self.disable_fields.languages && !self.languages.is_empty() {
            if self.languages.len() > 1 {
                let title = "Languages: ";
                let pad = " ".repeat(title.len());
                let mut s = String::from("");
                for (cnt, language) in self.languages.iter().enumerate() {
                    let formatted_number = format!("{:.*}", 2, language.1);
                    if cnt != 0 && cnt % 3 == 0 {
                        s = s + &format!("\n{}{} ({} %) ", pad, language.0, formatted_number);
                    } else {
                        s = s + &format!("{} ({} %) ", language.0, formatted_number);
                    }
                }
                writeln!(buffer, "{}{}", title.color(color).bold(), s)?;
            } else {
                let title = "Language: ";
                writeln!(
                    buffer,
                    "{}{}",
                    title.color(color).bold(),
                    self.dominant_language
                )?;
            };
        }

        if !self.disable_fields.authors && !self.authors.is_empty() {
            let title = if self.authors.len() > 1 {
                "Authors: "
            } else {
                "Author: "
            };

            writeln!(buffer, "{}{}% {} {}", title.color(color).bold(), self.authors[0].2, self.authors[0].0, self.authors[0].1)?;

            let title = " ".repeat(title.len());

            for author in self.authors.iter().skip(1) {
                writeln!(buffer, "{}{}% {} {}", title.color(color).bold(), author.2, author.0, author.1)?;
            }
        }

        if !self.disable_fields.last_change {
            writeln!(
                buffer,
                "{}{}",
                "Last change: ".color(color).bold(),
                self.last_change
            )?;
        }

        if !self.disable_fields.repo {
            writeln!(buffer, "{}{}", "Repo: ".color(color).bold(), self.repo)?;
        }

        if !self.disable_fields.commits {
            writeln!(
                buffer,
                "{}{}",
                "Commits: ".color(color).bold(),
                self.commits
            )?;
        }

        if !self.disable_fields.lines_of_code {
            writeln!(
                buffer,
                "{}{}",
                "Lines of code: ".color(color).bold(),
                self.number_of_lines
            )?;
        }

        if !self.disable_fields.size {
            writeln!(
                buffer,
                "{}{}",
                "Size: ".color(color).bold(),
                self.repo_size
            )?;
        }

        if !self.disable_fields.license {
            writeln!(
                buffer,
                "{}{}",
                "License: ".color(color).bold(),
                self.license
            )?;
        }

        writeln!(
           buffer,
           "\n{0}{1}{2}{3}{4}{5}{6}{7}\n{8}{9}{10}{11}{12}{13}{14}{15}",
           "   ".on_black(),
           "   ".on_red(),
           "   ".on_green(),
           "   ".on_yellow(),
           "   ".on_blue(),
           "   ".on_magenta(),
           "   ".on_cyan(),
           "   ".on_white(),
           "   ".on_bright_black(),
           "   ".on_bright_red(),
           "   ".on_bright_green(),
           "   ".on_bright_yellow(),
           "   ".on_bright_blue(),
           "   ".on_bright_magenta(),
           "   ".on_bright_cyan(),
           "   ".on_bright_white(),
        )?;


        let mut logo_lines = AsciiArt::new(self.get_ascii(), self.colors());
        let mut info_lines = buffer.lines();
        let center_pad = "   ";
        loop {
            match (logo_lines.next(), info_lines.next()) {
                (Some(logo_line), Some(info_line)) => 
                    writeln!(f, "{}{}{:^}", logo_line, center_pad, info_line)?,
                (Some(logo_line), None) => 
                    writeln!(f, "{}", logo_line)?,
                (None, Some(info_line)) =>
                    writeln!(f, "{:<width$}{}{:^}", "", center_pad, info_line, width = logo_lines.width())?,
                (None, None) => { writeln!(f, "\n")?; break },
            }
        };

        Ok(())
    }
}

struct CommitInfo {
    commit: Oid,
    refs: Vec<String>,
}

impl CommitInfo {
    fn new(commit: Oid, refs: Vec<String>) -> CommitInfo {
        CommitInfo { commit, refs }
    }
}

impl fmt::Display for CommitInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let short_commit = self.commit.to_string().chars().take(7).collect::<String>();
        if self.refs.len() > 0 {
            let refs_str = self.refs.iter().map(|ref_name| {
                ref_name.as_str()
            }).collect::<Vec<&str>>().join(", ");
            write!(f, "{} ({})", short_commit, refs_str)
        } else {
            write!(f, "{}", short_commit)
        }
    }
}

#[derive(Default)]
struct InfoFieldOn {
    project: bool,
    head: bool,
    version: bool,
    created: bool,
    languages: bool,
    authors: bool,
    last_change: bool,
    repo: bool,
    commits: bool,
    lines_of_code: bool,
    size: bool,
    license: bool,
}

#[derive(PartialEq, Eq, EnumString, EnumCount, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
enum InfoFields {
    Project,
    HEAD,
    Version,
    Created,
    Languages,
    Authors,
    LastChange,
    Repo,
    Commits,
    LinesOfCode,
    Size,
    License,
    UnrecognizedField,
}

#[derive(PartialEq, Eq, Hash, Clone, EnumString)]
#[strum(serialize_all = "lowercase")]
enum Language {
    Assembly,
    C,
    Clojure,
    CoffeeScript,
    Cpp,
    Csharp,
    CSS,
    Dart,
    Elixir,
    Elm,
    Erlang,
    Forth,
    Go,
    Haskell,
    HTML,
    Idris,
    Java,
    JavaScript,
    Kotlin,
    Lisp,
    Lua,
    Nim,
    ObjectiveC,
    Perl,
    Php,
    PureScript,
    Python,
    R,
    Ruby,
    Rust,
    Scala,
    Shell,
    Swift,
    Tcl,
    Tex,
    TypeScript,
    Vue,
    XML,
    Zig,
    Unknown
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Language::Assembly => write!(f, "Assembly"),
            Language::C => write!(f, "C"),
            Language::Clojure => write!(f, "Clojure"),
            Language::CoffeeScript => write!(f, "CoffeeScript"),
            Language::Cpp => write!(f, "C++"),
            Language::Csharp => write!(f, "C#"),
            Language::CSS => write!(f, "CSS"),
            Language::Dart => write!(f, "Dart"),
            Language::Elixir => write!(f, "Elixir"),
            Language::Elm => write!(f, "Elm"),
            Language::Erlang => write!(f, "Erlang"),
            Language::Forth => write!(f, "Forth"),
            Language::Go => write!(f, "Go"),
            Language::Haskell => write!(f, "Haskell"),
            Language::HTML => write!(f, "HTML"),
            Language::Idris => write!(f, "Idris"),
            Language::Java => write!(f, "Java"),
            Language::JavaScript => write!(f, "JavaScript"),
            Language::Kotlin => write!(f, "Kotlin"),
            Language::Lisp => write!(f, "Lisp"),
            Language::Lua => write!(f, "Lua"),
            Language::Nim => write!(f, "Nim"),
            Language::ObjectiveC => write!(f, "Objective-C"),
            Language::PureScript => write!(f, "PureScript"),
            Language::Python => write!(f, "Python"),
            Language::R => write!(f, "R"),
            Language::Ruby => write!(f, "Ruby"),
            Language::Rust => write!(f, "Rust"),
            Language::Scala => write!(f, "Scala"),
            Language::Shell => write!(f, "Shell"),
            Language::Swift => write!(f, "Swift"),
            Language::Perl => write!(f, "Perl"),
            Language::Php => write!(f, "Php"),
            Language::Tcl => write!(f, "Tcl"),
            Language::Tex => write!(f, "Tex"),
            Language::TypeScript => write!(f, "TypeScript"),
            Language::Vue => write!(f, "Vue"),
            Language::XML => write!(f, "XML"),
            Language::Zig => write!(f, "Zig"),
            Language::Unknown => write!(f, "Unknown"),
        }
    }
}

fn main() -> Result<()> {

    #[cfg(target = "windows")]
    let enabled = ansi_term::enable_ansi_support().is_ok();

    #[cfg(not(target = "windows"))]
    let enabled = true;

    if !enabled {
        colored::control::set_override(false);
    }


    if !is_git_installed() {
        return Err(Error::GitNotInstalled);
    }

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author("o2sh <ossama-hjaji@live.fr>")
        .about(crate_description!())
        .arg(Arg::with_name("directory")
            .short("d")
            .long("dir")
            .takes_value(true)
            .default_value("."))
        .arg(Arg::with_name("ascii_language")
            .short("a")
            .long("ascii_language")
            .takes_value(true)
            .default_value("")
            .help("Overrides showing the dominant language ascii logo"))
        .arg(Arg::with_name("disable_field")
            .long("disable")
            .multiple(true)
            .takes_value(true)
            .case_insensitive(true)
            .default_value("")
            .hide_default_value(true)
            .help(&format!("Disable fields to show\nPossible values: {:?}",
                &InfoFields::iter()
                    .take(InfoFields::count() - 1)
                    .map(|field| field.into())
                    .collect::<Vec<&str>>()
                    .as_slice())))
        .arg(Arg::with_name("colors")
            .short("c")
            .long("colors")
            .multiple(true)
            .takes_value(true)
            .possible_values(&[
                "0",
                "1",
                "2",
                "3",
                "4",
                "5",
                "6",
                "7",
                "8",
                "9",
                "10",
                "11",
                "12",
                "13",
                "14",
                "15",
                ])
            .hide_possible_values(true)
            .help(&format!(
                "Specifies a preferred color set. Unspecified colors will remain as default.
Possible values: [{0}{1}{2}{3}{4}{5}{6}{7}{8}{9}{10}{11}{12}{13}{14}{15}]",
                "0".black(),
                "1".red(),
                "2".green(),
                "3".yellow(),
                "4".blue(),
                "5".magenta(),
                "6".cyan(),
                "7".white(),
                "8".bright_black(),
                "9".bright_red(),
                "10".bright_green(),
                "11".bright_yellow(),
                "12".bright_blue(),
                "13".bright_magenta(),
                "14".bright_cyan(),
                "15".bright_white(),
            )))
        .get_matches();
    let dir = String::from(matches.value_of("directory").unwrap());
    let custom_logo: Language = Language::from_str(
        &matches
            .value_of("ascii_language")
            .unwrap()
            .to_lowercase())
        .unwrap_or(Language::Unknown);
    let mut disable_fields = InfoFieldOn { ..Default::default() };

    matches.values_of("disable_field")
        .unwrap()
        .map(String::from)
        .for_each(|field: String| {
            let item = InfoFields::from_str(field.to_lowercase().as_str())
                .unwrap_or(InfoFields::UnrecognizedField);

            match item {
                InfoFields::Project => disable_fields.project = true,
                InfoFields::HEAD => disable_fields.head = true,
                InfoFields::Version => disable_fields.version = true,
                InfoFields::Created => disable_fields.created = true,
                InfoFields::Languages => disable_fields.languages = true,
                InfoFields::Authors => disable_fields.authors = true,
                InfoFields::LastChange => disable_fields.last_change = true,
                InfoFields::Repo => disable_fields.repo = true,
                InfoFields::Commits => disable_fields.commits = true,
                InfoFields::LinesOfCode => disable_fields.lines_of_code = true,
                InfoFields::Size => disable_fields.size = true,
                InfoFields::License => disable_fields.license = true,
                _ => (),
            }
        });

    let tokei_langs = project_languages(&dir);
    let languages_stat = get_languages_stat(&tokei_langs).ok_or(Error::SourceCodeNotFound)?;
    let mut languages_stat_vec: Vec<(_, _)> = languages_stat.into_iter().collect();
    languages_stat_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());
    let dominant_language = languages_stat_vec[0].0.clone();

    let authors = get_authors(&dir, 3);
    let current_commit_info = get_current_commit_info(&dir)?;
    let config = get_configuration(&dir)?;
    let version = get_version(&dir)?;
    let commits = get_commits(&dir)?;
    let repo_size = get_packed_size(&dir)?;
    let last_change = get_last_change(&dir)?;
    let creation_date = get_creation_time().unwrap();
    let custom_colors: Vec<String> = if let Some(values) = matches.values_of("colors") {
        values.map(String::from).collect()
    } else {
        Vec::new()
    };

    let info = Info {
        project_name: config.repository_name,
        current_commit: current_commit_info,
        version,
        creation_date: creation_date,
        dominant_language,
        languages: languages_stat_vec,
        authors,
        last_change,
        repo: config.repository_url,
        commits,
        repo_size,
        number_of_lines: get_total_loc(&tokei_langs),
        license: project_license(&dir)?,
        custom_logo,
        custom_colors,
        disable_fields,
    };

    print!("{}", info);
    Ok(())
}

fn project_languages(dir: &str) -> tokei::Languages {
    use tokei::Config;

    let mut languages = tokei::Languages::new();
    let required_languages = get_all_language_types();
    let tokei_config = Config {
        types: Some(required_languages),
        ..Config::default()
    };
    languages.get_statistics(&[&dir], &[".git", "target"], &tokei_config);
    languages
}

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
                    && !(entry
                        .file_name()
                        .map(OsStr::to_string_lossy)
                        .iter()
                        .filter(|x| x.starts_with("LICENSE") || x.starts_with("COPYING"))
                        .collect::<Vec<_>>()
                        .is_empty())
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
    let size_line = lines.split("\n").find(|line| {
        line.starts_with("size-pack:")
    });

    let repo_size = match size_line {
        None => "??",
        Some(size_str) => &(size_str[11..])
    };

    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .arg("ls-files")
        .output()
        .expect("Failed to execute git.");
    // To check if command executed successfully or not
    let error = &output.stderr;

    if error.is_empty(){
        let output = String::from_utf8_lossy(&output.stdout);

        let lines = output.to_string();
        let files_list = lines.split("\n");
        let mut files_count:u128 = 0;
        for _file in files_list {
            files_count+=1;
        }
        files_count-=1; // As splitting giving one line extra(blank).
        let res = repo_size.to_owned() + &(" (") + &(files_count.to_string()) + &(" files)");
        Ok(res.into())
    }
    else{
        let res =repo_size;
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

#[derive(Debug)]
struct Configuration {
    pub repository_name: String,
    pub repository_url: String,
}

fn get_configuration(dir: &str) -> Result<Configuration> {
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
        .map(|(author, count)| (author.trim_matches('\'').to_string(), count, count*100/total_commits))
        .collect();

    authors
}

fn get_current_commit_info(dir: &str) -> Result<CommitInfo> {
    let repo = Repository::open(dir).map_err(|_| Error::NotGitRepo)?;
    let head = repo.head().map_err(|_| Error::ReferenceInfoError)?;
    let head_oid = head.target().ok_or(Error::ReferenceInfoError)?;
    let refs = repo.references().map_err(|_| Error::ReferenceInfoError)?;
    let refs_info = refs.into_iter().filter_map(|reference| {
        match reference {
            Ok(reference) => {
                match (reference.target(), reference.shorthand()) {
                    (Some(oid), Some(shorthand)) if oid == head_oid => {
                        Some(if reference.is_tag() {
                            String::from("tags/") + shorthand
                        } else {
                            String::from(shorthand)
                        })
                    },
                    _ => None
                }
            },
            Err(_) => None,
        }
    }).collect::<Vec<String>>();
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
        None => None
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
            tokei::LanguageType::Elixir => Language::Elixir,
            tokei::LanguageType::Elm => Language::Elm,
            tokei::LanguageType::Erlang => Language::Erlang,
            tokei::LanguageType::Forth => Language::Forth,
            tokei::LanguageType::Go => Language::Go,
            tokei::LanguageType::Haskell => Language::Haskell,
            tokei::LanguageType::Html => Language::HTML,
            tokei::LanguageType::Idris => Language::Idris,
            tokei::LanguageType::Java => Language::Java,
            tokei::LanguageType::JavaScript => Language::JavaScript,
            tokei::LanguageType::Kotlin => Language::Kotlin,
            tokei::LanguageType::Lisp => Language::Lisp,
            tokei::LanguageType::Lua => Language::Lua,
            tokei::LanguageType::Nim => Language::Nim,
            tokei::LanguageType::ObjectiveC => Language::ObjectiveC,
            tokei::LanguageType::Perl => Language::Perl,
            tokei::LanguageType::Php => Language::Php,
            tokei::LanguageType::PureScript => Language::PureScript,
            tokei::LanguageType::Python => Language::Python,
            tokei::LanguageType::R => Language::R,
            tokei::LanguageType::Ruby => Language::Ruby,
            tokei::LanguageType::Rust => Language::Rust,
            tokei::LanguageType::Scala => Language::Scala,
            tokei::LanguageType::Sh => Language::Shell,
            tokei::LanguageType::Swift => Language::Swift,
            tokei::LanguageType::Tcl => Language::Tcl,
            tokei::LanguageType::Tex => Language::Tex,
            tokei::LanguageType::TypeScript => Language::TypeScript,
            tokei::LanguageType::Vue => Language::Vue,
            tokei::LanguageType::Xml => Language::XML,
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
        tokei::LanguageType::Elixir,
        tokei::LanguageType::Elm,
        tokei::LanguageType::Erlang,
        tokei::LanguageType::Forth,
        tokei::LanguageType::Go,
        tokei::LanguageType::Haskell,
        tokei::LanguageType::Html,
        tokei::LanguageType::Idris,
        tokei::LanguageType::Java,
        tokei::LanguageType::JavaScript,
        tokei::LanguageType::Kotlin,
        tokei::LanguageType::Lisp,
        tokei::LanguageType::Lua,
        tokei::LanguageType::Nim,
        tokei::LanguageType::ObjectiveC,
        tokei::LanguageType::Perl,
        tokei::LanguageType::Php,
        tokei::LanguageType::PureScript,
        tokei::LanguageType::Python,
        tokei::LanguageType::R,
        tokei::LanguageType::Ruby,
        tokei::LanguageType::Rust,
        tokei::LanguageType::Scala,
        tokei::LanguageType::Sh,
        tokei::LanguageType::Swift,
        tokei::LanguageType::Tcl,
        tokei::LanguageType::Tex,
        tokei::LanguageType::TypeScript,
        tokei::LanguageType::Vue,
        tokei::LanguageType::Xml,
        tokei::LanguageType::Zig,
    ]
}

impl Info {
    pub fn get_ascii(&self) -> &str {
        let language =
            if let Language::Unknown = self.custom_logo {
                &self.dominant_language
            } else {
                &self.custom_logo
            };

        match language {
            Language::Assembly => include_str!("../resources/assembly.ascii"),
            Language::C => include_str!("../resources/c.ascii"),
            Language::Clojure => include_str!("../resources/clojure.ascii"),
            Language::CoffeeScript => include_str!("../resources/coffeescript.ascii"),
            Language::Cpp => include_str!("../resources/cpp.ascii"),
            Language::Csharp => include_str!("../resources/csharp.ascii"),
            Language::CSS => include_str!("../resources/css.ascii"),
            Language::Dart => include_str!("../resources/dart.ascii"),
            Language::Elixir => include_str!("../resources/elixir.ascii"),
            Language::Elm => include_str!("../resources/elm.ascii"),
            Language::Erlang => include_str!("../resources/erlang.ascii"),
            Language::Forth => include_str!("../resources/forth.ascii"),
            Language::Go => include_str!("../resources/go.ascii"),
            Language::Haskell => include_str!("../resources/haskell.ascii"),
            Language::HTML => include_str!("../resources/html.ascii"),
            Language::Idris => include_str!("../resources/idris.ascii"),
            Language::Java => include_str!("../resources/java.ascii"),
            Language::JavaScript => include_str!("../resources/javascript.ascii"),
            Language::Kotlin => include_str!("../resources/kotlin.ascii"),
            Language::Lisp => include_str!("../resources/lisp.ascii"),
            Language::Lua => include_str!("../resources/lua.ascii"),
            Language::Nim => include_str!("../resources/nim.ascii"),
            Language::ObjectiveC => include_str!("../resources/objectivec.ascii"),
            Language::Perl => include_str!("../resources/perl.ascii"),
            Language::Php => include_str!("../resources/php.ascii"),
            Language::PureScript => include_str!("../resources/purescript.ascii"),
            Language::Python => include_str!("../resources/python.ascii"),
            Language::R => include_str!("../resources/r.ascii"),
            Language::Ruby => include_str!("../resources/ruby.ascii"),
            Language::Rust => include_str!("../resources/rust.ascii"),
            Language::Scala => include_str!("../resources/scala.ascii"),
            Language::Shell => include_str!("../resources/shell.ascii"),
            Language::Swift => include_str!("../resources/swift.ascii"),
            Language::Tcl => include_str!("../resources/tcl.ascii"),
            Language::Tex => include_str!("../resources/tex.ascii"),
            Language::TypeScript => include_str!("../resources/typescript.ascii"),
            Language::Vue => include_str!("../resources/vue.ascii"),
            Language::XML => include_str!("../resources/xml.ascii"),
            Language::Zig => include_str!("../resources/zig.ascii"),
            Language::Unknown => include_str!("../resources/unknown.ascii"),
            // _ => include_str!("../resources/unknown.ascii"),
        }
    }

    fn colors(&self) -> Vec<Color> {
        let language =
            if let Language::Unknown = self.custom_logo {
                &self.dominant_language
            } else {
                &self.custom_logo
            };

       let colors = match language {
            Language::Assembly => vec![Color::Cyan],
            Language::C => vec![Color::BrightBlue, Color::Blue],
            Language::Clojure => vec![Color::BrightBlue, Color::BrightGreen],
            Language::CoffeeScript => vec![Color::Red],
            Language::Cpp => vec![Color::Yellow, Color::Cyan],
            Language::Csharp => vec![Color::White],
            Language::CSS => vec![Color::Blue, Color::White],
            Language::Dart => vec![Color::BrightBlue, Color::BrightCyan],
            Language::Elixir => vec![Color::Magenta],
            Language::Elm => vec![Color::BrightBlack, Color::Green, Color::Yellow, Color::Cyan],
            Language::Erlang => vec![Color::BrightRed],
            Language::Forth => vec![Color::BrightRed],
            Language::Go => vec![Color::White],
            Language::Haskell => vec![Color::BrightBlue, Color::BrightMagenta, Color::Blue],
            Language::HTML => vec![Color::Red, Color::White],
            Language::Idris => vec![Color::Red],
            Language::Java => vec![Color::BrightBlue, Color::Red],
            Language::JavaScript => vec![Color::BrightYellow],
            Language::Kotlin => vec![Color::Blue, Color::Yellow, Color::Magenta],
            Language::Lisp => vec![Color::Yellow],
            Language::Lua => vec![Color::Blue],
            Language::Nim => vec![Color::Yellow, Color::BrightWhite],
            Language::ObjectiveC => vec![Color::BrightBlue, Color::Blue],
            Language::Perl => vec![Color::BrightBlue],
            Language::Php => vec![Color::BrightWhite],
            Language::PureScript => vec![Color::White],
            Language::Python => vec![Color::Blue, Color::Yellow],
            Language::R => vec![Color::White, Color::Blue],
            Language::Ruby => vec![Color::Magenta],
            Language::Rust => vec![Color::White, Color::BrightRed],
            Language::Scala => vec![Color::Blue],
            Language::Shell => vec![Color::Green],
            Language::Swift => vec![Color::BrightRed],
            Language::Tcl => vec![Color::Blue, Color::White, Color::BrightBlue],
            Language::Tex => vec![Color::White, Color::Black],
            Language::TypeScript => vec![Color::Cyan],
            Language::Vue => vec![Color::BrightGreen, Color::Blue],
            Language::XML => vec![Color::Yellow, Color::BrightBlack, Color::BrightGreen],
            Language::Zig => vec![Color::Yellow],
            Language::Unknown => vec![Color::White],
        };

        let colors: Vec<Color> = colors.iter().enumerate().map(|(index, default_color)| {
            if let Some(color_num) = self.custom_colors.get(index) {
                if let Some(color) = num_to_color(color_num) {
                    return color;
                }
            }
            *default_color
        }).collect();
        colors
    }
}

/// Custom error type
enum Error {
    /// Sourcecode could be located
    SourceCodeNotFound,
    /// Git is not installed or did not function properly
    GitNotInstalled,
    /// Did not find any git data in the directory
    NoGitData,
    /// An IO error occoured while reading ./
    ReadDirectory,
    /// Not in a Git Repo
    NotGitRepo,
    /// Error while getting branch info
    ReferenceInfoError,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let content = match self {
            Error::SourceCodeNotFound => "Could not find any source code in this directory",
            Error::GitNotInstalled => "Git failed to execute",
            Error::NoGitData => "Could not retrieve git configuration data",
            Error::ReadDirectory => "Could not read directory",
            Error::NotGitRepo => "This is not a Git Repo",
            Error::ReferenceInfoError => "Error while retrieving reference information",
        };
        write!(f, "{}", content)
    }
}

fn num_to_color(num: &str) -> Option<Color> {
    let color = match num {
        "0" => Color::Black,
        "1" => Color::Red,
        "2" => Color::Green,
        "3" => Color::Yellow,
        "4" => Color::Blue,
        "5" => Color::Magenta,
        "6" => Color::Cyan,
        "7" => Color::White,
        "8" => Color::BrightBlack,
        "9" => Color::BrightRed,
        "10" => Color::BrightGreen,
        "11" => Color::BrightYellow,
        "12" => Color::BrightBlue,
        "13" => Color::BrightMagenta,
        "14" => Color::BrightCyan,
        "15" => Color::BrightWhite,
        _ => return None,
    };
    Some(color)
}
