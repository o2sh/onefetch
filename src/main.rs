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

use clap::{App, Arg};
use colored::*;
use std::{
    convert::From,
    process::{Command, Stdio},
    result,
    str::FromStr,
};
use strum::{EnumCount, IntoEnumIterator};

mod ascii_art;
mod commit_info;
mod error;
mod info;
mod language;

use ascii_art::AsciiArt;
use commit_info::CommitInfo;
use error::Error;
use info::Info;
use language::Language;

type Result<T> = result::Result<T, Error>;

#[derive(Default)]
pub struct InfoFieldOn {
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
        .arg(
            Arg::with_name("directory")
                .short("d")
                .long("dir")
                .takes_value(true)
                .default_value("."),
        )
        .arg(
            Arg::with_name("ascii_language")
                .short("a")
                .long("ascii_language")
                .takes_value(true)
                .default_value("")
                .help("Overrides showing the dominant language ascii logo"),
        )
        .arg(
            Arg::with_name("disable_field")
                .long("disable")
                .multiple(true)
                .takes_value(true)
                .case_insensitive(true)
                .default_value("")
                .hide_default_value(true)
                .help(&format!(
                    "Disable fields to show\nPossible values: {:?}",
                    &InfoFields::iter()
                        .take(InfoFields::count() - 1)
                        .map(|field| field.into())
                        .collect::<Vec<&str>>()
                        .as_slice()
                )),
        )
        .arg(
            Arg::with_name("colors")
                .short("c")
                .long("colors")
                .multiple(true)
                .takes_value(true)
                .possible_values(&[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
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
                )),
        )
        .arg(
            Arg::with_name("supported")
                .short("s")
                .long("supported")
                .help("Prints a list of all supported languages"),
        )
        .get_matches();

    if matches.is_present("supported") {
        let list = Language::iter()
            .filter(|x| *x != Language::Unknown)
            .map(|x| x.to_string().color(x.get_colors()[0]).to_string())
            .collect::<Vec<String>>()
            .join(", ");

        print!("Supported languages:\n\n{}\n", list);
        std::process::exit(0);
    }

    let dir = String::from(matches.value_of("directory").unwrap());
    let custom_logo: Language =
        Language::from_str(&matches.value_of("ascii_language").unwrap().to_lowercase())
            .unwrap_or(Language::Unknown);
    let mut disable_fields = InfoFieldOn {
        ..Default::default()
    };

    matches
        .values_of("disable_field")
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

    let custom_colors: Vec<String> = if let Some(values) = matches.values_of("colors") {
        values.map(String::from).collect()
    } else {
        Vec::new()
    };

    let info = Info::new(&dir, custom_logo, custom_colors, disable_fields)?;

    print!("{}", info);
    Ok(())
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
