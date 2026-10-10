use crate::git::bots::{BotRegex, NO_BOTS_DEFAULT_REGEX_PATTERN};
use crate::info::display_options::{
    DEFAULT_NUMBER_OF_AUTHORS, DEFAULT_NUMBER_OF_LANGUAGES, DisplayOptions, NumberSeparator,
};
use crate::info::{InfoKind, InfoOptions};
use crate::language::{Language, LanguageType};
use crate::ui::printer::SerializationFormat;
use clap::builder::PossibleValuesParser;
use clap::builder::Styles;
use clap::builder::TypedValueParser as _;
use clap::builder::styling::AnsiColor;
use clap::{Args, Parser, ValueHint, value_parser};
use clap_complete::Shell;
use onefetch_image::ImageProtocol;
use std::env;
use std::path::PathBuf;

const COLOR_RESOLUTIONS: [&str; 5] = ["16", "32", "64", "128", "256"];
const DEFAULT_NUMBER_OF_FILE_CHURNS: usize = 3;

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .usage(AnsiColor::Green.on_default())
    .literal(AnsiColor::Green.on_default())
    .placeholder(AnsiColor::Green.on_default());

#[derive(Clone, Debug, Parser, PartialEq, Eq)]
#[command(version, about)]
#[command(styles = STYLES)]
pub struct Cli {
    /// Run as if onefetch was started in <input> instead of the current working directory
    #[arg(default_value = ".", hide_default_value = true, value_hint = ValueHint::DirPath)]
    pub input: PathBuf,
    #[command(flatten)]
    pub info: InfoArgs,
    #[command(flatten)]
    pub text_formatting: TextFormattingArgs,
    #[command(flatten)]
    pub ascii: AsciiArgs,
    #[command(flatten)]
    pub image: ImageArgs,
    #[command(flatten)]
    pub visuals: VisualsArgs,
    #[command(flatten)]
    pub developer: DeveloperArgs,
    #[command(flatten)]
    pub other: OtherArgs,
}

#[derive(Clone, Debug, Args, PartialEq, Eq)]
#[command(next_help_heading = "INFO")]
pub struct InfoArgs {
    /// Allows you to disable FIELD(s) from appearing in the output
    #[arg(
        long,
        short,
        num_args = 1..,
        hide_possible_values = true,
        value_enum,
        value_name = "FIELD"
    )]
    pub disabled_fields: Vec<InfoKind>,
    /// Hides the title
    #[arg(long)]
    pub no_title: bool,
    /// Maximum NUM of authors to be shown
    #[arg(long, default_value_t = DEFAULT_NUMBER_OF_AUTHORS, value_name = "NUM")]
    pub number_of_authors: usize,
    /// Maximum NUM of languages to be shown
    #[arg(long, default_value_t = DEFAULT_NUMBER_OF_LANGUAGES, value_name = "NUM")]
    pub number_of_languages: usize,
    /// Maximum NUM of file churns to be shown
    #[arg(long, default_value_t = DEFAULT_NUMBER_OF_FILE_CHURNS, value_name = "NUM")]
    pub number_of_file_churns: usize,
    /// Minimum NUM of commits from HEAD used to compute the churn summary
    ///
    /// By default, the actual value is non-deterministic due to time-based computation
    /// and will be shown in the field label as "Churn (NUM)"
    #[arg(long, value_name = "NUM")]
    pub churn_pool_size: Option<usize>,
    /// Ignore all files & directories matching EXCLUDE
    #[arg(long, short, num_args = 1..)]
    pub exclude: Vec<String>,
    /// Exclude [bot] commits. Use <REGEX> to override the default pattern
    #[arg(
        long,
        num_args = 0..=1,
        require_equals = true,
        default_missing_value = NO_BOTS_DEFAULT_REGEX_PATTERN,
        value_name = "REGEX"
    )]
    pub no_bots: Option<BotRegex>,
    /// Ignores merge commits
    #[arg(long)]
    pub no_merges: bool,
    /// Show the email address of each author
    #[arg(long, short = 'E')]
    pub email: bool,
    /// Display repository URL as HTTP
    #[arg(long)]
    pub http_url: bool,
    /// Hide token in repository URL
    #[arg(long)]
    pub hide_token: bool,
    /// Count hidden files and directories
    #[arg(long)]
    pub include_hidden: bool,
    /// Filters output by language type
    #[arg(
        long = "type",
        num_args = 1..,
        default_values = &["programming", "markup"],
        short = 'T',
        value_enum,
        value_name = "TYPE",
    )]
    pub language_types: Vec<LanguageType>,
}

#[derive(Clone, Debug, Args, PartialEq, Eq)]
#[command(next_help_heading = "ASCII")]
pub struct AsciiArgs {
    /// Takes a non-empty STRING as input to replace the ASCII logo
    ///
    /// It is possible to pass a generated STRING by command substitution
    ///
    /// For example:
    ///
    /// '--ascii-input "$(fortune | cowsay -W 25)"'
    #[arg(long, value_name = "STRING", value_hint = ValueHint::CommandString)]
    pub ascii_input: Option<String>,
    /// Colors (X X X...) to print the ascii art
    #[arg(
        long,
        num_args = 1..,
        value_name = "X",
        short = 'c',
        value_parser = value_parser!(u8).range(..16),
    )]
    pub ascii_colors: Vec<u8>,
    /// Which LANGUAGE's ascii art to print
    #[arg(
        long,
        short,
        value_name = "LANGUAGE",
        value_enum,
        hide_possible_values = true
    )]
    pub ascii_language: Option<Language>,
    /// Specify when to use true color
    ///
    /// If set to auto: true color will be enabled if supported by the terminal
    #[arg(long, default_value = "auto", value_name = "WHEN", value_enum)]
    pub true_color: When,
}

#[derive(Clone, Debug, Args, PartialEq, Eq)]
#[command(next_help_heading = "IMAGE")]
pub struct ImageArgs {
    /// Path to the IMAGE file
    #[arg(long, short, value_hint = ValueHint::FilePath)]
    pub image: Option<PathBuf>,
    /// Which image PROTOCOL to use
    #[arg(long, value_enum, requires = "image", value_name = "PROTOCOL")]
    pub image_protocol: Option<ImageProtocol>,
    /// VALUE of color resolution to use with SIXEL backend
    #[arg(
        long,
        value_name = "VALUE",
        requires = "image",
        default_value_t = 64usize,
        value_parser = PossibleValuesParser::new(COLOR_RESOLUTIONS)
            .map(|s| s.parse::<usize>().unwrap())
    )]
    pub color_resolution: usize,
}

#[derive(Clone, Debug, Args, PartialEq, Eq)]
#[command(next_help_heading = "TEXT FORMATTING")]
pub struct TextFormattingArgs {
    /// Changes the text colors (X X X...)
    ///
    /// Goes in order of title, ~, underline, key, separator, and value
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
    /// Turns off bold formatting
    #[arg(long)]
    pub no_bold: bool,
}
#[derive(Clone, Debug, Args, PartialEq, Eq, Default)]
#[command(next_help_heading = "VISUALS")]
pub struct VisualsArgs {
    /// Hides the color palette
    #[arg(long)]
    pub no_color_palette: bool,
    /// Hides the ascii art or image if provided
    #[arg(long)]
    pub no_art: bool,
    /// Use Nerd Font icons
    ///
    /// Replaces language chips with Nerd Font icons
    #[arg(long)]
    pub nerd_fonts: bool,
}

#[derive(Clone, Debug, Args, PartialEq, Eq, Default)]
#[command(next_help_heading = "DEVELOPER")]
pub struct DeveloperArgs {
    /// Outputs Onefetch in a specific format
    #[arg(long, short, value_name = "FORMAT", value_enum)]
    pub output: Option<SerializationFormat>,
    /// If provided, outputs the completion file for given SHELL
    #[arg(long = "generate", value_name = "SHELL", value_enum)]
    pub completion: Option<Shell>,
}

#[derive(Clone, Debug, Args, PartialEq, Eq, Default)]
#[command(next_help_heading = "OTHER")]
pub struct OtherArgs {
    /// Prints out supported languages
    #[arg(long, short)]
    pub languages: bool,
    /// Prints out supported package managers
    #[arg(long, short)]
    pub package_managers: bool,
}

impl Default for Cli {
    fn default() -> Cli {
        Cli {
            input: PathBuf::from("."),
            info: InfoArgs::default(),
            text_formatting: TextFormattingArgs::default(),
            visuals: VisualsArgs::default(),
            ascii: AsciiArgs::default(),
            image: ImageArgs::default(),
            developer: DeveloperArgs::default(),
            other: OtherArgs::default(),
        }
    }
}

impl Default for InfoArgs {
    fn default() -> Self {
        InfoArgs {
            number_of_authors: DEFAULT_NUMBER_OF_AUTHORS,
            number_of_languages: DEFAULT_NUMBER_OF_LANGUAGES,
            number_of_file_churns: DEFAULT_NUMBER_OF_FILE_CHURNS,
            churn_pool_size: Option::default(),
            exclude: Vec::default(),
            no_bots: Option::default(),
            no_merges: Default::default(),
            email: Default::default(),
            http_url: Default::default(),
            hide_token: Default::default(),
            include_hidden: Default::default(),
            language_types: vec![LanguageType::Programming, LanguageType::Markup],
            disabled_fields: Vec::default(),
            no_title: Default::default(),
        }
    }
}

impl Default for TextFormattingArgs {
    fn default() -> Self {
        TextFormattingArgs {
            text_colors: Vec::default(),
            iso_time: Default::default(),
            number_separator: NumberSeparator::Plain,
            no_bold: Default::default(),
        }
    }
}

impl Default for AsciiArgs {
    fn default() -> Self {
        AsciiArgs {
            ascii_input: Option::default(),
            ascii_colors: Vec::default(),
            ascii_language: Option::default(),
            true_color: When::Auto,
        }
    }
}
impl Default for ImageArgs {
    fn default() -> Self {
        ImageArgs {
            image: Option::default(),
            image_protocol: Option::default(),
            color_resolution: 64,
        }
    }
}

pub fn is_truecolor_terminal() -> bool {
    env::var("COLORTERM")
        .map(|colorterm| colorterm == "truecolor" || colorterm == "24bit")
        .unwrap_or(false)
}

impl Cli {
    pub fn true_color(&self) -> bool {
        match self.ascii.true_color {
            When::Always => true,
            When::Never => false,
            When::Auto => is_truecolor_terminal(),
        }
    }
}

impl From<&Cli> for InfoOptions {
    fn from(cli: &Cli) -> Self {
        Self {
            input: cli.input.clone(),
            disabled_fields: cli.info.disabled_fields.clone(),
            no_title: cli.info.no_title,
            number_of_authors: cli.info.number_of_authors,
            number_of_file_churns: cli.info.number_of_file_churns,
            churn_pool_size: cli.info.churn_pool_size,
            exclude: cli.info.exclude.clone(),
            no_bots: cli.info.no_bots.clone(),
            no_merges: cli.info.no_merges,
            email: cli.info.email,
            http_url: cli.info.http_url,
            hide_token: cli.info.hide_token,
            include_hidden: cli.info.include_hidden,
            language_types: cli.info.language_types.clone(),
        }
    }
}

impl From<&Cli> for DisplayOptions {
    fn from(cli: &Cli) -> Self {
        Self {
            number_separator: cli.text_formatting.number_separator,
            iso_time: cli.text_formatting.iso_time,
            true_color: cli.true_color(),
            nerd_fonts: cli.visuals.nerd_fonts,
            no_color_palette: cli.visuals.no_color_palette,
            number_of_languages: cli.info.number_of_languages,
            number_of_authors: cli.info.number_of_authors,
        }
    }
}

#[derive(clap::ValueEnum, Clone, PartialEq, Eq, Debug)]
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
        let config: Cli = Cli::default();
        assert_eq!(config, Cli::parse_from(["onefetch"]));
    }

    #[test]
    fn test_custom_config() {
        let config: Cli = Cli {
            input: PathBuf::from("/tmp/folder"),
            info: InfoArgs {
                number_of_authors: 4,
                no_merges: true,
                disabled_fields: vec![InfoKind::Version, InfoKind::URL],
                ..Default::default()
            },
            ascii: AsciiArgs {
                ascii_colors: vec![5, 0],
                ascii_language: Some(Language::Lisp),
                ..Default::default()
            },
            visuals: VisualsArgs {
                no_art: true,
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(
            config,
            Cli::parse_from([
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
        assert!(Cli::try_parse_from(["onefetch", "--image-protocol", "sixel"]).is_err())
    }

    #[test]
    fn test_config_with_color_resolution_but_no_image() {
        assert!(Cli::try_parse_from(["onefetch", "--color-resolution", "32"]).is_err())
    }

    #[test]
    fn test_config_with_ascii_colors_but_out_of_bounds() {
        assert!(Cli::try_parse_from(["onefetch", "--ascii-colors", "17"]).is_err())
    }

    #[test]
    fn test_config_with_text_colors_but_out_of_bounds() {
        assert!(Cli::try_parse_from(["onefetch", "--text-colors", "17"]).is_err())
    }
}
