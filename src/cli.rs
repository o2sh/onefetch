use crate::info::deps::package_manager::PackageManager;
use crate::info::info_field::{InfoField, InfoFieldOff};
use crate::info::langs::language::{Language, LanguageType};
use crate::ui::image_backends;
use crate::ui::image_backends::ImageBackend;
use crate::ui::image_backends::ImageProtocol;
use crate::ui::printer::SerializationFormat;
use anyhow::{Context, Result};
use clap::AppSettings;
use clap::Parser;
use clap_complete::{generate, Generator, Shell};
use image::DynamicImage;
use regex::Regex;
use std::ffi;
use std::fmt;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::{convert::From, env, str::FromStr};
use strum::IntoEnumIterator;

const MAX_TERM_WIDTH: usize = 95;

#[derive(Parser)]
#[clap(version, about, hide_possible_values = true, long_about = None, rename_all = "kebab-case")]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
pub struct Config {
    /// Run as if onefetch was started in <input> instead of the current working directory
    #[clap(long, default_value = ".", value_parser, hide_default_value = true)]
    pub input: PathBuf,
    /// Takes a non-empty STRING as input to replace the ASCII logo
    ///
    /// It is possible to pass a generated STRING by command substitution
    ///
    /// For example:
    ///
    /// '--ascii-input "$(fortune | cowsay -W 25)'
    #[clap(long, value_name = "STRING", value_parser)]
    pub ascii_input: Option<String>,
    /// Which LANGUAGE's ascii art to print
    #[clap(long, short, value_name = "LANGUAGE", arg_enum, value_parser)]
    pub ascii_language: Option<Language>,
    /// Colors (X X X...) to print the ascii art
    #[clap(
        long,
        multiple_values = true,
        value_name = "X",
        short = 'c',
        value_parser
    )]
    pub ascii_colors: Vec<u8>,
    /// Allows you to disable FIELD(s) from appearing in the output
    #[clap(
        long,
        short,
        multiple_values = true,
        value_name = "FIELD",
        value_parser
    )]
    pub disabled_fields: Vec<InfoField>,
    /// Path to the IMAGE file
    #[clap(long, short, value_parser)]
    pub image: Option<PathBuf>,
    /// Which image BACKEND to use
    #[clap(long, value_parser)]
    pub image_protocol: Option<ImageProtocol>,
    /// VALUE of color resolution to use with SIXEL backend
    #[clap(long, value_name = "VALUE", value_parser)]
    pub color_resolution: Option<usize>,
    /// Turns off bold formatting
    #[clap(long, action)]
    pub no_bold: bool,
    /// Ignores merge commits
    #[clap(long, action)]
    pub no_merges: bool,
    /// Hides the color palette
    #[clap(long, action)]
    pub no_color_palette: bool,
    /// NUM of authors to be shown
    #[clap(
        long,
        short,
        default_value_t = 3,
        value_name = "NUM",
        value_parser,
        hide_default_value = true
    )]
    pub number_of_authors: usize,
    /// gnore all files & directories matching EXCLUDE
    #[clap(long, multiple_values = true, short, value_parser)]
    pub exclude: Vec<PathBuf>,
    /// Exclude [bot] commits. Use <REGEX> to override the default pattern
    #[clap(long, value_name = "REGEX", value_parser)]
    pub no_bots: Option<Regex>,
    /// Prints out supported languages
    #[clap(long, short, action)]
    pub languages: bool,
    /// Prints out supported package managers
    #[clap(long, short, action)]
    pub package_managers: bool,
    /// Outputs Onefetch in a specific format (json, yaml)
    #[clap(long, short, value_name = "FORMAT", value_parser)]
    pub output: Option<SerializationFormat>,
    /// Specify when to use true color (*auto*, never, always)
    ///
    /// If set to auto: true color will be enabled if supported by the terminal
    #[clap(
        long,
        default_value = "auto",
        value_name = "WHEN",
        value_parser,
        hide_default_value = true
    )]
    pub true_color: When,
    /// Specify when to show the logo (auto, never, *always*)
    ///
    /// If set to auto: the logo will be hidden if the terminal's width < 95
    #[clap(
        long,
        default_value = "always",
        value_name = "WHEN",
        value_parser,
        hide_default_value = true
    )]
    pub show_logo: When,
    /// Changes the text colors (X X X...)
    ///
    /// Goes in order of title, ~, underline, subtitle, colon, and info
    ///
    /// For example:
    ///
    /// '--text-colors 9 10 11 12 13 14'
    #[clap(long, short, multiple_values = true, value_name = "X", value_parser)]
    pub text_colors: Vec<u8>,
    /// Use ISO 8601 formatted timestamps
    #[clap(long, short = 'z', action)]
    pub iso_time: bool,
    /// Show the email address of each author
    #[clap(long, short = 'E', action)]
    pub email: bool,
    /// Count hidden files and directories
    #[clap(long, action)]
    pub include_hidden: bool,
    /// Filters output by language type (*programming*, *markup*, prose, data)
    #[clap(
        long,
        multiple_values = true,
        default_values = &["programming", "markup"],
        hide_default_value = true,
        short = 'T',
        value_parser
    )]
    pub r#type: Vec<LanguageType>,
    /// Prints out SHELL completion script
    #[clap(long, value_parser)]
    pub completion: Option<Shell>,
}

pub fn print_supported_languages() -> Result<()> {
    for l in Language::iter() {
        println!("{}", l);
    }

    Ok(())
}

pub fn print_supported_package_managers() -> Result<()> {
    for p in PackageManager::iter() {
        println!("{}", p);
    }

    Ok(())
}

pub fn is_truecolor_terminal() -> bool {
    env::var("COLORTERM")
        .map(|colorterm| colorterm == "truecolor" || colorterm == "24bit")
        .unwrap_or(false)
}

pub fn get_git_version() -> String {
    let version = Command::new("git").arg("--version").output();

    match version {
        Ok(v) => String::from_utf8_lossy(&v.stdout).replace('\n', ""),
        Err(_) => String::new(),
    }
}

#[derive(clap::ValueEnum, Clone)]
pub enum When {
    Auto,
    Never,
    Always,
}
