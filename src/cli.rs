use crate::info::deps::package_manager::PackageManager;
use crate::info::info_field::InfoType;
use crate::info::langs::language::{Language, LanguageType};
use crate::ui::image_backends::ImageProtocol;
use crate::ui::printer::SerializationFormat;
use anyhow::Result;
use clap::AppSettings;
use clap::{Command, Parser, ValueHint};
use clap_complete::{generate, Generator, Shell};
use merge::Merge;
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{self, Visitor};
use std::env;
use std::io;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use strum::IntoEnumIterator;

#[derive(Clone, Debug, Parser, PartialEq, Eq, Deserialize, Serialize, Merge)]
#[clap(version, about, long_about = None, rename_all = "kebab-case")]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
pub struct Config {
    /// Run as if onefetch was started in <input> instead of the current working directory
    #[clap(default_value = ".", hide_default_value = true, value_hint = ValueHint::DirPath)]
    #[merge(skip)]
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
    #[merge(strategy = overwrite_vector)]
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
    #[merge(strategy = overwrite_vector)]
    pub disabled_fields: Vec<InfoType>,
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
    #[merge(strategy = overwrite)]
    pub color_resolution: usize,
    /// Turns off bold formatting
    #[clap(long)]
    #[merge(strategy = merge::bool::overwrite_false)]
    pub no_bold: bool,
    /// Ignores merge commits
    #[clap(long)]
    #[merge(strategy = merge::bool::overwrite_false)]
    pub no_merges: bool,
    /// Hides the color palette
    #[clap(long)]
    #[merge(strategy = merge::bool::overwrite_false)]
    pub no_color_palette: bool,
    /// NUM of authors to be shown
    #[clap(long, short, default_value_t = 3usize, value_name = "NUM")]
    #[merge(strategy = overwrite)]
    pub number_of_authors: usize,
    /// gnore all files & directories matching EXCLUDE
    #[clap(long, multiple_values = true, short, value_hint = ValueHint::AnyPath)]
    #[merge(strategy = overwrite_vector)]
    pub exclude: Vec<PathBuf>,
    /// Exclude [bot] commits. Use <REGEX> to override the default pattern
    #[clap(long, value_name = "REGEX")]
    pub no_bots: Option<Option<MyRegex>>,
    /// Prints out supported languages
    #[clap(long, short)]
    #[serde(skip)]
    #[merge(strategy = merge::bool::overwrite_false)]
    pub languages: bool,
    /// Prints out supported package managers
    #[clap(long, short)]
    #[serde(skip)]
    #[merge(strategy = merge::bool::overwrite_false)]
    pub package_managers: bool,
    /// Outputs Onefetch in a specific format
    #[clap(long, short, value_name = "FORMAT", arg_enum)]
    pub output: Option<SerializationFormat>,
    /// Specify when to use true color
    ///
    /// If set to auto: true color will be enabled if supported by the terminal
    #[clap(long, default_value = "auto", value_name = "WHEN", arg_enum)]
    #[merge(strategy = overwrite)]
    pub true_color: When,
    /// Specify when to show the logo
    ///
    /// If set to auto: the logo will be hidden if the terminal's width < 95
    #[clap(long, default_value = "always", value_name = "WHEN", arg_enum)]
    #[merge(strategy = overwrite)]
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
    #[merge(strategy = overwrite_vector)]
    pub text_colors: Vec<u8>,
    /// Use ISO 8601 formatted timestamps
    #[clap(long, short = 'z')]
    #[merge(strategy = merge::bool::overwrite_false)]
    pub iso_time: bool,
    /// Show the email address of each author
    #[clap(long, short = 'E')]
    #[merge(strategy = merge::bool::overwrite_false)]
    pub email: bool,
    /// Count hidden files and directories
    #[clap(long)]
    #[merge(strategy = merge::bool::overwrite_false)]
    pub include_hidden: bool,
    /// Filters output by language type
    #[clap(
        long,
        multiple_values = true,
        default_values = &["programming", "markup"],
        short = 'T',
        arg_enum,
    )]
    #[merge(strategy = overwrite_vector)]
    pub r#type: Vec<LanguageType>,
    /// Specify a custom path to a config file.
    /// Default config is located at ${HOME}/.config/onefetch/config.conf.
    #[clap(long, value_hint = ValueHint::AnyPath)]
    #[merge(skip)]
    pub config_path: Option<PathBuf>,
    /// If provided, outputs the completion file for given SHELL
    #[clap(long = "generate", value_name = "SHELL", arg_enum)]
    #[serde(skip)]
    pub completion: Option<Shell>,
}

impl Default for Config {
    fn default() -> Self { Config {
        input: PathBuf::from("."),
        ascii_input: Default::default(),
        ascii_language: Default::default(),
        ascii_colors: Default::default(),
        disabled_fields: Default::default(),
        image: Default::default(),
        image_protocol: Default::default(),
        color_resolution: 16,
        no_bold: Default::default(),
        no_merges: Default::default(),
        no_color_palette: Default::default(),
        number_of_authors: 3,
        exclude: Default::default(),
        no_bots: Default::default(),
        languages: Default::default(),
        package_managers: Default::default(),
        output: Default::default(),
        true_color: When::Auto,
        show_logo: When::Always,
        text_colors: Default::default(),
        iso_time: Default::default(),
        email: Default::default(),
        include_hidden: Default::default(),
        r#type: vec![LanguageType::Programming, LanguageType::Markup],
        config_path: Default::default(),
        completion: Default::default(),
    } }
}

fn overwrite<T>(left: &mut T, right: T) {
    *left = right;
}

fn overwrite_vector<T>(left: &mut Vec<T>, mut right: Vec<T>) {
    left.clear();
    left.append(&mut right);
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

#[derive(clap::ValueEnum, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum When {
    Auto,
    Never,
    Always,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config, Config::parse_from(&["onefetch"]))
    }

    #[test]
    fn test_custom_config() {
        let mut config = Config::default();
        config.number_of_authors = 4;
        config.input = PathBuf::from("/tmp/folder");
        config.no_merges = true;
        config.ascii_colors = vec![5, 0];
        config.disabled_fields = vec![InfoType::Version, InfoType::Repo];
        config.show_logo = When::Never;
        config.ascii_language = Some(Language::Lisp);

        assert_eq!(
            config,
            Config::parse_from(&[
                "onefetch",
                "/tmp/folder",
                "--number-of-authors",
                "4",
                "--no-merges",
                "--ascii-colors",
                "5",
                "0",
                "--disabled-fields",
                "version",
                "repo",
                "--show-logo",
                "never",
                "--ascii-language",
                "lisp"
            ])
        )
    }

    #[test]
    fn test_config_with_image_protocol_but_no_image() {
        assert!(Config::try_parse_from(&["onefetch", "--image-protocol", "sixel"]).is_err())
    }

    #[test]
    fn test_config_with_color_resolution_but_no_image() {
        assert!(Config::try_parse_from(&["onefetch", "--color-resolution", "32"]).is_err())
    }

    #[test]
    fn test_config_with_ascii_colors_but_out_of_bounds() {
        assert!(Config::try_parse_from(&["onefetch", "--ascii-colors", "17"]).is_err())
    }

    #[test]
    fn test_config_with_text_colors_but_out_of_bounds() {
        assert!(Config::try_parse_from(&["onefetch", "--text-colors", "17"]).is_err())
    }
}

#[derive(Clone, Debug)]
pub struct MyRegex(pub Regex);

impl Eq for MyRegex {}

impl PartialEq for MyRegex {
    fn eq(&self, other: &MyRegex) -> bool {
        self.0.as_str() == other.0.as_str()
    }
}

impl FromStr for MyRegex {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(MyRegex(Regex::new(s)?))
    }
}

impl Serialize for MyRegex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

struct MyRegexVisitor;

impl<'de> Visitor<'de> for MyRegexVisitor {
    type Value = MyRegex;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a str representation of a regex")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match MyRegex::from_str(s) {
            Ok(regex) => Ok(regex),
            Err(error) => Err(de::Error::custom(error))
        }
    }
}

impl<'de> Deserialize<'de> for MyRegex {
    fn deserialize<D>(deserializer: D) -> Result<MyRegex, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(MyRegexVisitor)
    }
}
