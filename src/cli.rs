use crate::info::deps::package_manager::PackageManager;
use crate::info::info_field::InfoField;
use crate::info::langs::language::{Language, LanguageType};
use crate::ui::image_backends::ImageProtocol;
use crate::ui::printer::SerializationFormat;
use anyhow::Result;
use clap::AppSettings;
use clap::{Command, Parser, ValueHint};
use clap_complete::{generate, Generator, Shell};
use regex::Regex;
use std::env;
use std::io;
use std::path::PathBuf;
use strum::IntoEnumIterator;

#[derive(Parser)]
#[clap(version, about, long_about = None, rename_all = "kebab-case")]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
pub struct Config {
    /// Run as if onefetch was started in <input> instead of the current working directory
    #[clap(default_value = ".", hide_default_value = true, value_hint = ValueHint::DirPath)]
    pub input: PathBuf,
    /// Takes a non-empty STRING as input to replace the ASCII logo
    ///
    /// It is possible to pass a generated STRING by command substitution
    ///
    /// For example:
    ///
    /// '--ascii-input "$(fortune | cowsay -W 25)'
    #[clap(long, value_name = "STRING", value_hint = ValueHint::CommandString)]
    pub ascii_input: Option<String>,
    /// Which LANGUAGE's ascii art to print
    #[clap(
        long,
        short,
        value_name = "LANGUAGE",
        arg_enum,
        hide_possible_values = true
    )]
    pub ascii_language: Option<Language>,
    /// Colors (X X X...) to print the ascii art
    #[clap(
        long,
        multiple_values = true,
        value_name = "X",
        short = 'c',
        value_parser = clap::value_parser!(u8).range(..16),
    )]
    pub ascii_colors: Vec<u8>,
    /// Allows you to disable FIELD(s) from appearing in the output
    #[clap(
        long,
        short,
        multiple_values = true,
        hide_possible_values = true,
        arg_enum,
        value_name = "FIELD"
    )]
    pub disabled_fields: Vec<InfoField>,
    /// Path to the IMAGE file
    #[clap(long, short, value_hint = ValueHint::FilePath)]
    pub image: Option<PathBuf>,
    /// Which image protocol to use
    #[clap(long, arg_enum, requires = "image")]
    pub image_protocol: Option<ImageProtocol>,
    /// VALUE of color resolution to use with SIXEL backend
    #[clap(
        long,
        value_name = "VALUE",
        requires = "image",
        default_value_t = 16usize,
        possible_values = ["16", "32", "64", "128", "256"],
    )]
    pub color_resolution: usize,
    /// Turns off bold formatting
    #[clap(long)]
    pub no_bold: bool,
    /// Ignores merge commits
    #[clap(long)]
    pub no_merges: bool,
    /// Hides the color palette
    #[clap(long)]
    pub no_color_palette: bool,
    /// NUM of authors to be shown
    #[clap(long, short, default_value_t = 3usize, value_name = "NUM")]
    pub number_of_authors: usize,
    /// gnore all files & directories matching EXCLUDE
    #[clap(long, multiple_values = true, short, value_hint = ValueHint::AnyPath)]
    pub exclude: Vec<PathBuf>,
    /// Exclude [bot] commits. Use <REGEX> to override the default pattern
    #[clap(long, value_name = "REGEX")]
    pub no_bots: Option<Option<Regex>>,
    /// Prints out supported languages
    #[clap(long, short)]
    pub languages: bool,
    /// Prints out supported package managers
    #[clap(long, short)]
    pub package_managers: bool,
    /// Outputs Onefetch in a specific format
    #[clap(long, short, value_name = "FORMAT", arg_enum)]
    pub output: Option<SerializationFormat>,
    /// Specify when to use true color
    ///
    /// If set to auto: true color will be enabled if supported by the terminal
    #[clap(long, default_value = "auto", value_name = "WHEN", arg_enum)]
    pub true_color: When,
    /// Specify when to show the logo
    ///
    /// If set to auto: the logo will be hidden if the terminal's width < 95
    #[clap(long, default_value = "always", value_name = "WHEN", arg_enum)]
    pub show_logo: When,
    /// Changes the text colors (X X X...)
    ///
    /// Goes in order of title, ~, underline, subtitle, colon, and info
    ///
    /// For example:
    ///
    /// '--text-colors 9 10 11 12 13 14'
    #[clap(
        long,
        short = 't',
        multiple_values = true,
        value_name = "X",
        value_parser = clap::value_parser!(u8).range(..16),
        max_values = 6
    )]
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
    /// Filters output by language type
    #[clap(
        long,
        multiple_values = true,
        default_values = &["programming", "markup"],
        short = 'T',
        arg_enum,
    )]
    pub r#type: Vec<LanguageType>,
    /// If provided, outputs the completion file for given SHELL
    #[clap(long = "generate", value_name = "SHELL", arg_enum)]
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
    let version = std::process::Command::new("git").arg("--version").output();

    match version {
        Ok(v) => String::from_utf8_lossy(&v.stdout).replace('\n', ""),
        Err(_) => String::new(),
    }
}

pub fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

#[derive(clap::ValueEnum, Clone)]
pub enum When {
    Auto,
    Never,
    Always,
}
