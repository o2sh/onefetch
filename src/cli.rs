use crate::info::langs::language::{Language, LanguageType};
use crate::info::utils::info_field::InfoType;
use crate::ui::printer::SerializationFormat;
use anyhow::Result;
use clap::builder::PossibleValuesParser;
use clap::builder::TypedValueParser as _;
use clap::{value_parser, Command, Parser, ValueHint};
use clap_complete::{generate, Generator, Shell};
use num_format::CustomFormat;
use onefetch_image::ImageProtocol;
use onefetch_manifest::ManifestType;
use regex::Regex;
use serde::Serialize;
use std::env;
use std::io;
use std::path::PathBuf;
use std::str::FromStr;
use strum::IntoEnumIterator;

const COLOR_RESOLUTIONS: [&str; 5] = ["16", "32", "64", "128", "256"];

#[derive(Clone, Debug, Parser, PartialEq, Eq)]
#[command(version, about, long_about = None, rename_all = "kebab-case")]
pub struct Config {
    /// Run as if onefetch was started in <input> instead of the current working directory
    #[arg(default_value = ".", hide_default_value = true, value_hint = ValueHint::DirPath)]
    pub input: PathBuf,
    /// Takes a non-empty STRING as input to replace the ASCII logo
    ///
    /// It is possible to pass a generated STRING by command substitution
    ///
    /// For example:
    ///
    /// '--ascii-input "$(fortune | cowsay -W 25)"'
    #[arg(long, value_name = "STRING", value_hint = ValueHint::CommandString)]
    pub ascii_input: Option<String>,
    /// Which LANGUAGE's ascii art to print
    #[arg(
        long,
        short,
        value_name = "LANGUAGE",
        value_enum,
        hide_possible_values = true
    )]
    pub ascii_language: Option<Language>,
    /// Colors (X X X...) to print the ascii art
    #[arg(
        long,
        num_args = 1..,
        value_name = "X",
        short = 'c',
        value_parser = value_parser!(u8).range(..16),
    )]
    pub ascii_colors: Vec<u8>,
    /// Allows you to disable FIELD(s) from appearing in the output
    #[arg(
        long,
        short,
        num_args = 1..,
        hide_possible_values = true,
        value_enum,
        value_name = "FIELD"
    )]
    pub disabled_fields: Vec<InfoType>,
    /// Path to the IMAGE file
    #[arg(long, short, value_hint = ValueHint::FilePath)]
    pub image: Option<PathBuf>,
    /// Which image protocol to use
    #[arg(long, value_enum, requires = "image")]
    pub image_protocol: Option<ImageProtocol>,
    /// VALUE of color resolution to use with SIXEL backend
    #[arg(
        long,
        value_name = "VALUE",
        requires = "image",
        default_value_t = 16usize,
        value_parser = PossibleValuesParser::new(COLOR_RESOLUTIONS)
            .map(|s| s.parse::<usize>().unwrap())
    )]
    pub color_resolution: usize,
    /// Turns off bold formatting
    #[arg(long)]
    pub no_bold: bool,
    /// Ignores merge commits
    #[arg(long)]
    pub no_merges: bool,
    /// Hides the color palette
    #[arg(long)]
    pub no_color_palette: bool,
    /// Hides the title
    #[arg(long)]
    pub no_title: bool,
    /// Maximum NUM of authors to be shown
    #[arg(long, default_value_t = 3usize, value_name = "NUM")]
    pub number_of_authors: usize,
    /// Maximum NUM of languages to be shown
    #[arg(long, default_value_t = 6usize, value_name = "NUM")]
    pub number_of_languages: usize,
    /// Ignore all files & directories matching EXCLUDE
    #[arg(long, short, num_args = 1.., value_hint = ValueHint::AnyPath)]
    pub exclude: Vec<PathBuf>,
    /// Exclude [bot] commits. Use <REGEX> to override the default pattern
    #[arg(long, value_name = "REGEX")]
    pub no_bots: Option<Option<MyRegex>>,
    /// Prints out supported languages
    #[arg(long, short)]
    pub languages: bool,
    /// Prints out supported package managers
    #[arg(long, short)]
    pub package_managers: bool,
    /// Outputs Onefetch in a specific format
    #[arg(long, short, value_name = "FORMAT", value_enum)]
    pub output: Option<SerializationFormat>,
    /// Specify when to use true color
    ///
    /// If set to auto: true color will be enabled if supported by the terminal
    #[arg(long, default_value = "auto", value_name = "WHEN", value_enum)]
    pub true_color: When,
    /// Specify when to show the logo
    ///
    /// If set to auto: the logo will be hidden if the terminal's width < 95
    #[arg(long, default_value = "always", value_name = "WHEN", value_enum)]
    pub show_logo: When,
    /// Changes the text colors (X X X...)
    ///
    /// Goes in order of title, ~, underline, subtitle, colon, and info
    ///
    /// For example:
    ///
    /// '--text-colors 9 10 11 12 13 14'
    #[arg(
        long,
        short,
        value_name = "X",
        value_parser = value_parser!(u8).range(..16),
        num_args = 1..=6
    )]
    pub text_colors: Vec<u8>,
    /// Use ISO 8601 formatted timestamps
    #[arg(long, short = 'z')]
    pub iso_time: bool,
    /// Which thousands SEPARATOR to use
    #[arg(long, value_name = "SEPARATOR", default_value = "plain", value_enum)]
    pub number_separator: NumberSeparator,
    /// Show the email address of each author
    #[arg(long, short = 'E')]
    pub email: bool,
    /// Count hidden files and directories
    #[arg(long)]
    pub include_hidden: bool,
    /// Filters output by language type
    #[arg(
        long,
        num_args = 1..,
        default_values = &["programming", "markup"],
        short = 'T',
        value_enum,
    )]
    pub r#type: Vec<LanguageType>,
    /// If provided, outputs the completion file for given SHELL
    #[arg(long = "generate", value_name = "SHELL", value_enum)]
    pub completion: Option<Shell>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
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
            no_title: Default::default(),
            number_of_authors: 3,
            number_of_languages: 6,
            exclude: Default::default(),
            no_bots: Default::default(),
            languages: Default::default(),
            package_managers: Default::default(),
            output: Default::default(),
            true_color: When::Auto,
            show_logo: When::Always,
            text_colors: Default::default(),
            iso_time: Default::default(),
            number_separator: NumberSeparator::Plain,
            email: Default::default(),
            include_hidden: Default::default(),
            r#type: vec![LanguageType::Programming, LanguageType::Markup],
            completion: Default::default(),
        }
    }
}

pub fn print_supported_languages() -> Result<()> {
    for l in Language::iter() {
        println!("{l}");
    }

    Ok(())
}

pub fn print_supported_package_managers() -> Result<()> {
    for p in ManifestType::iter() {
        println!("{p}");
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

#[derive(clap::ValueEnum, Clone, PartialEq, Eq, Debug)]
pub enum When {
    Auto,
    Never,
    Always,
}

#[derive(clap::ValueEnum, Clone, PartialEq, Eq, Debug, Serialize, Copy)]
pub enum NumberSeparator {
    Plain,
    Comma,
    Space,
    Underscore,
}

impl NumberSeparator {
    fn separator(&self) -> &'static str {
        match self {
            Self::Plain => "",
            Self::Comma => ",",
            Self::Space => "\u{202f}",
            Self::Underscore => "_",
        }
    }

    pub fn get_format(&self) -> CustomFormat {
        num_format::CustomFormat::builder()
            .grouping(num_format::Grouping::Standard)
            .separator(self.separator())
            .build()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_config() {
        let config: Config = Default::default();
        assert_eq!(config, Config::parse_from(&["onefetch"]))
    }

    #[test]
    fn test_custom_config() {
        let config: Config = Config {
            number_of_authors: 4,
            input: PathBuf::from("/tmp/folder"),
            no_merges: true,
            ascii_colors: vec![5, 0],
            disabled_fields: vec![InfoType::Version, InfoType::URL],
            show_logo: When::Never,
            ascii_language: Some(Language::Lisp),
            ..Default::default()
        };

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
                "url",
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
