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
#[cfg(target_os = "linux")]
use image_backends::ImageBackend;
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
    pending: bool,
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
    Pending,
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

    #[cfg(target_os = "linux")]
    let possible_backends = ["kitty", "sixel"];
    #[cfg(not(target_os = "linux"))]
    let possible_backends = [];

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
            Arg::with_name("ascii-language")
                .short("a")
                .long("ascii-language")
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
            Arg::with_name("disable-field")
                .short("f")
                .long("disable-field")
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
            Arg::with_name("image-backend")
                .long("image-backend")
                .takes_value(true)
                .possible_values(&possible_backends)
                .help("Overrides image backend detection"),
        )
        .arg(
            Arg::with_name("no-merges")
                .short("m")
                .long("no-merges")
                .help("Prevents merge commits from being counted"),
        )
        .arg(
            Arg::with_name("no-color-blocks")
                .short("k")
                .long("no-color-blocks")
                .help("Hide the color blocks"),
        )
        .arg(
            Arg::with_name("authors-number")
                .short("A")
                .long("authors-number")
                .takes_value(true)
                .help("Defines the number of authors to be shown"),
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

    let custom_logo: Language = if let Some(ascii_language) = matches.value_of("ascii-language") {
        Language::from_str(&ascii_language.to_lowercase()).unwrap()
    } else {
        Language::Unknown
    };
    let mut disable_fields = InfoFieldOn {
        ..Default::default()
    };

    matches
        .values_of("disable-field")
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
                InfoFields::Pending => disable_fields.pending = true,
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
    let image_backend = if custom_image.is_some() {
        if let Some(backend_name) = matches.value_of("image-backend") {
            #[cfg(target_os = "linux")]
            let backend = Some(match backend_name {
                "kitty" => Box::new(image_backends::kitty::KittyBackend::new()) as Box<dyn ImageBackend>,
                "sixel" => Box::new(image_backends::sixel::SixelBackend::new()) as Box<dyn ImageBackend>,
                _ => unreachable!()
            });
            #[cfg(not(target_os = "linux"))]
            let backend = None;
            backend
        } else {
            crate::image_backends::get_best_backend()
        }
    } else {
        None
    };

    let no_merges = matches.is_present("no-merges");

    let color_blocks_flag = matches.is_present("no-color-blocks");

    let author_number: usize = if let Some(value) = matches.value_of("authors-number") {
        usize::from_str(value).unwrap()
    } else {
        3
    };

    let info = Info::new(
        &dir,
        custom_logo,
        custom_colors,
        disable_fields,
        bold_flag,
        custom_image,
        image_backend,
        no_merges,
        color_blocks_flag,
        author_number,
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
