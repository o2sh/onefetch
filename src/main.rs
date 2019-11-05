extern crate bytecount;

extern crate askalono;
extern crate colored;
extern crate git2;
extern crate tokei;
#[macro_use]
extern crate clap;
extern crate strum;
#[macro_use]
extern crate strum_macros;
extern crate image;

#[cfg(target_os = "windows")]
extern crate ansi_term;

#[cfg(target_os = "linux")]
extern crate libc;

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
mod image_backends;
mod info;
mod language;
mod license;

use ascii_art::AsciiArt;
use commit_info::CommitInfo;
use error::Error;
use info::Info;
use language::Language;

type Result<T> = result::Result<T, Error>;

#[derive(Default)]
pub struct InfoFieldOn {
    git_info: bool,
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
    GitInfo,
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
    #[cfg(target_os = "windows")]
    let enabled = ansi_term::enable_ansi_support().is_ok();

    #[cfg(not(target_os = "windows"))]
    let enabled = true;

    if enabled {
        colored::control::set_override(true);
    }

    if !is_git_installed() {
        return Err(Error::GitNotInstalled);
    }

    let possible_languages: Vec<String> = Language::iter()
        .filter(|language| *language != Language::Unknown)
        .map(|language| language.to_string().to_lowercase())
        .collect();

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
                .possible_values(
                    &possible_languages
                        .iter()
                        .map(|l| l.as_str())
                        .collect::<Vec<&str>>(),
                )
                .case_insensitive(true)
                .help("Overrides showing the dominant language ascii logo."),
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
            Arg::with_name("no-bold")
                .short("b")
                .long("no-bold")
                .help("Turns off bold formatting for the logo and all labels"),
        )
        .arg(
            Arg::with_name("languages")
                .short("l")
                .long("languages")
                .help("Prints out supported languages"),
        )
        .arg(
            Arg::with_name("image")
                .short("i")
                .long("image")
                .takes_value(true)
                .help("Sets a custom image to use instead of the ascii logo"),
        )
        .arg(
            Arg::with_name("no-merges")
                .short("m")
                .long("no-merges")
                .help("Prevents merge commits from being counted"),
        )
        .get_matches();

    if matches.is_present("languages") {
        let iterator = Language::iter().filter(|x| *x != Language::Unknown);

        for l in iterator {
            println!("{}", l);
        }
        std::process::exit(0);
    }

    let dir = String::from(matches.value_of("directory").unwrap());
    let custom_logo: Language = if let Some(ascii_language) = matches.value_of("ascii_language") {
        Language::from_str(&ascii_language.to_lowercase()).unwrap()
    } else {
        Language::Unknown
    };
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
                InfoFields::GitInfo => disable_fields.git_info = true,
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

    let bold_flag = !matches.is_present("no-bold");

    let custom_image = if let Some(image_path) = matches.value_of("image") {
        Some(image::open(image_path).map_err(|_| Error::ImageLoadError)?)
    } else {
        None
    };

    let no_merges = matches.is_present("no-merges");

    let info = Info::new(
        &dir,
        custom_logo,
        custom_colors,
        disable_fields,
        bold_flag,
        custom_image,
        no_merges,
    )?;

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
