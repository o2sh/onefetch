use crate::i18n::locale_keys::cli::*;
use crate::info::langs::language::{Language, LanguageType};
use crate::info::utils::info_field::InfoType;
use crate::tr;
use crate::ui::printer::SerializationFormat;
use anyhow::Result;
use clap::builder::TypedValueParser as _;
use clap::builder::{PossibleValuesParser, Styles};
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

// TODO: check if short help requested more efficiently
use std::sync::LazyLock;
static IS_SHORT: LazyLock<bool> = LazyLock::new(|| {
    let args = std::env::args();
    let mut v = false;
    for value in args {
        if value == "-h" {
            v = true;
            break;
        } else if value == "--help" {
            break;
        } else {
        }
    }
    v
});

#[derive(Clone, Debug, Parser, PartialEq, Eq)]
#[command(
    about = tr!(ABOUT),
    version,
    disable_help_flag = true,
    disable_version_flag = true,
    help_template = format!("\
        {{before-help}}{{about-with-newline}}\
        \n{}{}:{} {{usage}}
        \n{{all-args}}{{after-help}}\
        ", 
        Styles::default().get_usage().render(),
        tr!(usage::HEADER),
        Styles::default().get_usage().render_reset()
    ),
    next_help_heading = tr!(arguments::HEADER),
    override_usage = format!("onefetch [{}] [{}]", tr!(options::HEADER).to_owned().to_uppercase(), tr!(value::INPUT))
)]
pub struct CliOptions {
    #[arg(
        default_value = ".", 
        hide_default_value = true,
        value_hint = ValueHint::DirPath,
        help = tr!(arguments::INPUT),
        value_name = tr!(value::INPUT)
    )]
    pub input: PathBuf,
    #[arg(
        action = if *IS_SHORT { clap::ArgAction::HelpShort } else { clap::ArgAction::HelpLong },
        long,
        short,
        help = tr!(options::HELP, short => &*IS_SHORT),
        help_heading = tr!(options::HEADER)
    )]
    pub help: Option<bool>,
    #[arg(
        action = clap::ArgAction::Version,
        long,
        short = 'V',
        help = tr!(options::VERSION),
        help_heading = tr!(options::HEADER)
    )]
    pub version: Option<bool>,
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
#[command(next_help_heading = tr!(info::HEADING))]
pub struct InfoCliOptions {
    #[arg(
        long,
        short,
        help = tr!(info::DISABLED_FIELDS),
        num_args = 1..,
        hide_possible_values = true,
        value_enum,
        value_name = tr!(value::FIELD)
    )]
    pub disabled_fields: Vec<InfoType>,
    #[arg(long, help = tr!(info::NO_TITLE))]
    pub no_title: bool,
    #[arg(long, default_value_t = 3usize, value_name = tr!(value::NUM), hide_default_value = true)]
    #[arg(
        help = tr!(info::number_of_authors::SHORT, def => 3),
        long_help = tr!(info::number_of_authors::LONG, def => 3),
        hide_default_value = true
    )]
    pub number_of_authors: usize,
    #[arg(long, default_value_t = 6usize, value_name = tr!(value::NUM))]
    #[arg(
        help = tr!(info::number_of_languages::SHORT, def => 6),
        long_help = tr!(info::number_of_languages::LONG, def => 6),
        hide_default_value = true
    )]
    pub number_of_languages: usize,
    #[arg(long, default_value_t = 3usize, value_name = tr!(value::NUM))]
    #[arg(
        help = tr!(info::number_of_file_churns::SHORT, def => 3),
        long_help = tr!(info::number_of_file_churns::LONG, def => 3),
        hide_default_value = true
    )]
    pub number_of_file_churns: usize,
    #[arg(long, value_name = tr!(value::NUM))]
    #[arg(
        help = tr!(info::churn_pool_size::SHORT),
        long_help = tr!(info::churn_pool_size::LONG)
    )]
    pub churn_pool_size: Option<usize>,
    #[arg(long, short, num_args = 1.., help = tr!(info::EXCLUDE), value_name = tr!(value::EXCLUDE))]
    pub exclude: Vec<String>,
    #[arg(
        long,
        num_args = 0..=1,
        require_equals = true,
        default_missing_value = NO_BOTS_DEFAULT_REGEX_PATTERN,
        value_name = tr!(value::REGEX),
        help = tr!(info::NO_BOTS)
    )]
    pub no_bots: Option<MyRegex>,
    #[arg(long, help = tr!(info::NO_MERGES))]
    pub no_merges: bool,
    #[arg(long, short = 'E', help = tr!(info::EMAIL))]
    pub email: bool,
    #[arg(long, help = tr!(info::HTTP_URL))]
    pub http_url: bool,
    #[arg(long, help = tr!(info::HIDE_TOKEN))]
    pub hide_token: bool,
    #[arg(long, help = tr!(info::INCLUDE_HIDDEN))]
    pub include_hidden: bool,
    #[arg(
        long,
        num_args = 1..,
        value_name = tr!(value::TYPE),
        default_values = &["programming", "markup"],
        short = 'T',
        value_enum,
    )]
    #[arg(
        help = tr!(info::tipe::SHORT, def => "programming, markup", pos => "programming, markup, prose, data"),
        long_help = tr!(info::tipe::LONG, def => "programming, markup", pos => "programming, markup, prose, data"),
        hide_default_value = true,
        hide_possible_values = true,
    )]
    pub r#type: Vec<LanguageType>,
}

#[derive(Clone, Debug, Args, PartialEq, Eq)]
#[command(next_help_heading = tr!(ascii::HEADING))]
pub struct AsciiCliOptions {
    #[arg(long, value_name = tr!(value::STRING), value_hint = ValueHint::CommandString, help = tr!(ascii::ASCII_INPUT))]
    pub ascii_input: Option<String>,
    #[arg(
        long,
        num_args = 1..,
        value_name = "X",
        short = 'c',
        value_parser = value_parser!(u8).range(..16),
        help = tr!(ascii::ASCII_COLORS)
    )]
    pub ascii_colors: Vec<u8>,
    #[arg(
        long,
        short,
        value_name = tr!(value::LANGUAGE),
        value_enum,
        hide_possible_values = true,
        help = tr!(ascii::ASCII_LANGUAGE)
    )]
    pub ascii_language: Option<Language>,
    #[arg(long, default_value = "auto", value_name = tr!(value::WHEN), value_enum, help = tr!(ascii::TRUE_COLOR))]
    pub true_color: When,
}

#[derive(Clone, Debug, Args, PartialEq, Eq)]
#[command(next_help_heading = tr!(image::HEADING))]
pub struct ImageCliOptions {
    #[arg(long, short, value_name = tr!(value::IMAGE), value_hint = ValueHint::FilePath, help = tr!(image::IMAGE))]
    pub image: Option<PathBuf>,
    #[arg(long, value_enum, requires = "image", value_name = tr!(value::PROTOCOL), help = tr!(image::IMAGE_PROTOCOL))]
    pub image_protocol: Option<ImageProtocol>,
    #[arg(
        long,
        value_name = tr!(value::VALUE),
        requires = "image",
        default_value_t = 16usize,
        value_parser = PossibleValuesParser::new(COLOR_RESOLUTIONS)
            .map(|s| s.parse::<usize>().unwrap()),
        help = tr!(image::COLOR_RESOLUTION)
    )]
    pub color_resolution: usize,
}

#[derive(Clone, Debug, Args, PartialEq, Eq)]
#[command(next_help_heading = tr!(text::HEADING))]
pub struct TextForamttingCliOptions {
    #[arg(
        long,
        short,
        value_name = "X",
        value_parser = value_parser!(u8).range(..16),
        num_args = 1..=6,
        help = tr!(text::COLORS)
    )]
    pub text_colors: Vec<u8>,
    #[arg(long, short = 'z', help = tr!(text::ISO_TIME))]
    pub iso_time: bool,
    #[arg(long, value_name = tr!(value::SEPARATOR), default_value = "plain", value_enum, help = tr!(text::NUMBER_SEPARATOR))]
    pub number_separator: NumberSeparator,
    #[arg(long, help = tr!(text::NO_BOLD))]
    pub no_bold: bool,
}
#[derive(Clone, Debug, Args, PartialEq, Eq, Default)]
#[command(next_help_heading = tr!(visuals::HEADING))]
pub struct VisualsCliOptions {
    #[arg(long, help = tr!(visuals::NO_COLOR_PALETTE))]
    pub no_color_palette: bool,
    #[arg(long, help = tr!(visuals::NO_ART))]
    pub no_art: bool,
    #[arg(long, help = tr!(visuals::NERD_FONTS))]
    pub nerd_fonts: bool,
}

#[derive(Clone, Debug, Args, PartialEq, Eq, Default)]
#[command(next_help_heading = tr!(dev::HEADING))]
pub struct DeveloperCliOptions {
    #[arg(long, short, value_name = tr!(value::FORMAT), value_enum, help = tr!(dev::OUTPUT))]
    pub output: Option<SerializationFormat>,
    #[arg(long = "generate", value_name = tr!(value::SHELL), value_enum, help = tr!(dev::COMPLETION))]
    pub completion: Option<Shell>,
}

#[derive(Clone, Debug, Args, PartialEq, Eq, Default)]
#[command(next_help_heading = tr!(other::HEADING))]
pub struct OtherCliOptions {
    #[arg(long, short, help = tr!(other::LANGUAGES))]
    pub languages: bool,
    #[arg(long, short, help = tr!(other::PACKAGE_MANAGERS))]
    pub package_managers: bool,
}

impl Default for CliOptions {
    fn default() -> CliOptions {
        CliOptions {
            help: None,
            version: None,
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
