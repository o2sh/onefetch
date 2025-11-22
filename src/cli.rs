use crate::tr;
use crate::info::langs::language::{Language, LanguageType};
use crate::info::utils::info_field::InfoType;
use crate::ui::printer::SerializationFormat;
use anyhow::Result;
use clap::builder::PossibleValuesParser;
use clap::builder::TypedValueParser as _;
use clap::{value_parser, Args, Command, Parser, ValueHint};
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
pub const NO_BOTS_DEFAULT_REGEX_PATTERN: &str = r"(?:-|\s)[Bb]ot$|\[[Bb]ot\]";

#[derive(Clone, Debug, Parser, PartialEq, Eq)]
#[command(version, about)]
pub struct CliOptions {
    #[arg(
        default_value = ".", 
        hide_default_value = true, 
        value_hint = ValueHint::DirPath,
        help = tr!("cli-input")
    )]
    pub input: PathBuf,
    #[command(flatten)]
    pub info: InfoCliOptions,
    #[command(flatten)]
    pub text_formatting: TextForamttingCliOptions,
    #[command(flatten)]
    pub ascii: AsciiCliOptions,
    #[command(flatten)]
    pub image: ImageCliOptions,
    #[command(flatten)]
    pub visuals: VisualsCliOptions,
    #[command(flatten)]
    pub developer: DeveloperCliOptions,
    #[command(flatten)]
    pub other: OtherCliOptions,
}

#[derive(Clone, Debug, Args, PartialEq, Eq)]
#[command(next_help_heading = tr!("cli-info-heading"))]
pub struct InfoCliOptions {
    #[arg(
        long,
        short,
        help = tr!("cli-info-disabled-fields"),
        num_args = 1..,
        hide_possible_values = true,
        value_enum,
        value_name = tr!("cli-value-field")
    )]
    pub disabled_fields: Vec<InfoType>,
    #[arg(long, help = tr!("cli-info-no-title"))]
    pub no_title: bool,
    #[arg(long, default_value_t = 3usize, value_name = tr!("cli-value-num"), help = tr!("cli-info-number-of-authors"))]
    pub number_of_authors: usize,
    #[arg(long, default_value_t = 6usize, value_name = tr!("cli-value-num"), help = tr!("cli-info-number-of-languages"))]
    pub number_of_languages: usize,
    #[arg(long, default_value_t = 3usize, value_name = tr!("cli-value-num"), help = tr!("cli-info-number-of-file-churns"))]
    pub number_of_file_churns: usize,
    #[arg(long, value_name = tr!("cli-value-num"), help = tr!("cli-info-churn-pool-size"))]
    pub churn_pool_size: Option<usize>,
    #[arg(long, short, num_args = 1.., help = tr!("cli-info-exclude"), value_name = tr!("cli-value-exclude"))]
    pub exclude: Vec<String>,
    #[arg(
        long,
        num_args = 0..=1,
        require_equals = true,
        default_missing_value = NO_BOTS_DEFAULT_REGEX_PATTERN,
        value_name = tr!("cli-value-regex"),
        help = tr!("cli-info-no-bots")
    )]
    pub no_bots: Option<MyRegex>,
    #[arg(long, help = tr!("cli-info-no-merges"))]
    pub no_merges: bool,
    #[arg(long, short = 'E', help = tr!("cli-info-email"))]
    pub email: bool,
    #[arg(long, help = tr!("cli-info-http-url"))]
    pub http_url: bool,
    #[arg(long, help = tr!("cli-info-hide-token"))]
    pub hide_token: bool,
    #[arg(long, help = tr!("cli-info-include-hidden"))]
    pub include_hidden: bool,
    #[arg(
        long,
        num_args = 1..,
        value_name = tr!("cli-value-type"),
        default_values = &["programming", "markup"],
        short = 'T',
        value_enum,
        help = tr!("cli-info-type")
    )]
    pub r#type: Vec<LanguageType>,
}

#[derive(Clone, Debug, Args, PartialEq, Eq)]
#[command(next_help_heading = tr!("cli-ascii-heading"))]
pub struct AsciiCliOptions {
    #[arg(long, value_name = tr!("cli-value-string"), value_hint = ValueHint::CommandString, help = tr!("cli-ascii-ascii-input"))]
    pub ascii_input: Option<String>,
    #[arg(
        long,
        num_args = 1..,
        value_name = "X",
        short = 'c',
        value_parser = value_parser!(u8).range(..16),
        help = tr!("cli-ascii-ascii-colors")
    )]
    pub ascii_colors: Vec<u8>,
    #[arg(
        long,
        short,
        value_name = tr!("cli-value-language"),
        value_enum,
        hide_possible_values = true,
        help = tr!("cli-ascii-ascii-language")
    )]
    pub ascii_language: Option<Language>,
    #[arg(long, default_value = "auto", value_name = tr!("cli-value-when"), value_enum, help = tr!("cli-ascii-true-color"))]
    pub true_color: When,
}

#[derive(Clone, Debug, Args, PartialEq, Eq)]
#[command(next_help_heading = "IMAGE")]
pub struct ImageCliOptions {
    /// Path to the IMAGE file
    #[arg(long, short, value_name = tr!("cli-value-image"), value_hint = ValueHint::FilePath)]
    pub image: Option<PathBuf>,
    /// Which image PROTOCOL to use
    #[arg(long, value_enum, requires = "image", value_name = tr!("cli-value-protocol"))]
    pub image_protocol: Option<ImageProtocol>,
    /// VALUE of color resolution to use with SIXEL backend
    #[arg(
        long,
        value_name = tr!("cli-value-value"),
        requires = "image",
        default_value_t = 16usize,
        value_parser = PossibleValuesParser::new(COLOR_RESOLUTIONS)
            .map(|s| s.parse::<usize>().unwrap())
    )]
    pub color_resolution: usize,
}

#[derive(Clone, Debug, Args, PartialEq, Eq)]
#[command(next_help_heading = tr!("cli-text-heading"))]
pub struct TextForamttingCliOptions {

    #[arg(
        long,
        short,
        value_name = "X",
        value_parser = value_parser!(u8).range(..16),
        num_args = 1..=6,
        help = tr!("cli-text-colors")
    )]
    pub text_colors: Vec<u8>,
    #[arg(long, short = 'z', help = tr!("cli-text-iso-time"))]
    pub iso_time: bool,
    #[arg(long, value_name = tr!("cli-value-separator"), default_value = "plain", value_enum, help = tr!("cli-text-number-separator"))]
    pub number_separator: NumberSeparator,
    #[arg(long, help = tr!("cli-text-no-bold"))]
    pub no_bold: bool,
}
#[derive(Clone, Debug, Args, PartialEq, Eq, Default)]
#[command(next_help_heading = tr!("cli-visuals-heading"))]
pub struct VisualsCliOptions {
    #[arg(long, help = tr!("cli-visuals-no-color-palette"))]
    pub no_color_palette: bool,
    #[arg(long, help = tr!("cli-visuals-no-art"))]
    pub no_art: bool,
    #[arg(long, help = tr!("cli-visuals-nerd-fonts"))]
    pub nerd_fonts: bool,
}

#[derive(Clone, Debug, Args, PartialEq, Eq, Default)]
#[command(next_help_heading = tr!("cli-dev-heading"))]
pub struct DeveloperCliOptions {
    #[arg(long, short, value_name = tr!("cli-value-format"), value_enum, help = tr!("cli-dev-output"))]
    pub output: Option<SerializationFormat>,
    #[arg(long = "generate", value_name = tr!("cli-value-shell"), value_enum, help = tr!("cli-dev-completion"))]
    pub completion: Option<Shell>,
}

#[derive(Clone, Debug, Args, PartialEq, Eq, Default)]
#[command(next_help_heading = tr!("cli-other-heading"))]
pub struct OtherCliOptions {
    #[arg(long, short, help = tr!("cli-other-languages"))]
    pub languages: bool,
    #[arg(long, short, help = tr!("cli-other-package-managers"))]
    pub package_managers: bool,
}

impl Default for CliOptions {
    fn default() -> CliOptions {
        CliOptions {
            input: PathBuf::from("."),
            info: InfoCliOptions::default(),
            text_formatting: TextForamttingCliOptions::default(),
            visuals: VisualsCliOptions::default(),
            ascii: AsciiCliOptions::default(),
            image: ImageCliOptions::default(),
            developer: DeveloperCliOptions::default(),
            other: OtherCliOptions::default(),
        }
    }
}

impl Default for InfoCliOptions {
    fn default() -> Self {
        InfoCliOptions {
            number_of_authors: 3,
            number_of_languages: 6,
            number_of_file_churns: 3,
            churn_pool_size: Option::default(),
            exclude: Vec::default(),
            no_bots: Option::default(),
            no_merges: Default::default(),
            email: Default::default(),
            http_url: Default::default(),
            hide_token: Default::default(),
            include_hidden: Default::default(),
            r#type: vec![LanguageType::Programming, LanguageType::Markup],
            disabled_fields: Vec::default(),
            no_title: Default::default(),
        }
    }
}

impl Default for TextForamttingCliOptions {
    fn default() -> Self {
        TextForamttingCliOptions {
            text_colors: Vec::default(),
            iso_time: Default::default(),
            number_separator: NumberSeparator::Plain,
            no_bold: Default::default(),
        }
    }
}

impl Default for AsciiCliOptions {
    fn default() -> Self {
        AsciiCliOptions {
            ascii_input: Option::default(),
            ascii_colors: Vec::default(),
            ascii_language: Option::default(),
            true_color: When::Auto,
        }
    }
}
impl Default for ImageCliOptions {
    fn default() -> Self {
        ImageCliOptions {
            image: Option::default(),
            image_protocol: Option::default(),
            color_resolution: 16,
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
    fn separator(self) -> &'static str {
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
        let config: CliOptions = CliOptions::default();
        assert_eq!(config, CliOptions::parse_from(["onefetch"]));
    }

    #[test]
    fn test_custom_config() {
        let config: CliOptions = CliOptions {
            input: PathBuf::from("/tmp/folder"),
            info: InfoCliOptions {
                number_of_authors: 4,
                no_merges: true,
                disabled_fields: vec![InfoType::Version, InfoType::URL],
                ..Default::default()
            },
            ascii: AsciiCliOptions {
                ascii_colors: vec![5, 0],
                ascii_language: Some(Language::Lisp),
                ..Default::default()
            },
            visuals: VisualsCliOptions {
                no_art: true,
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(
            config,
            CliOptions::parse_from([
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
                "--no-art",
                "--ascii-language",
                "lisp"
            ])
        );
    }

    #[test]
    fn test_config_with_image_protocol_but_no_image() {
        assert!(CliOptions::try_parse_from(["onefetch", "--image-protocol", "sixel"]).is_err())
    }

    #[test]
    fn test_config_with_color_resolution_but_no_image() {
        assert!(CliOptions::try_parse_from(["onefetch", "--color-resolution", "32"]).is_err())
    }

    #[test]
    fn test_config_with_ascii_colors_but_out_of_bounds() {
        assert!(CliOptions::try_parse_from(["onefetch", "--ascii-colors", "17"]).is_err())
    }

    #[test]
    fn test_config_with_text_colors_but_out_of_bounds() {
        assert!(CliOptions::try_parse_from(["onefetch", "--text-colors", "17"]).is_err())
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
