use crate::info::deps::package_manager::PackageManager;
use crate::info::info_field::{InfoField, InfoFieldOff};
use crate::info::langs::language::{Language, LanguageType};
use crate::ui::image_backends;
use crate::ui::image_backends::ImageBackend;
use crate::ui::printer::SerializationFormat;
use anyhow::{Context, Result};
use clap::{
    crate_description, crate_name, crate_version, error as clap_error, value_parser, AppSettings,
    Arg, ValueHint,
};
use clap_complete::{generate, Generator, Shell};
use image::DynamicImage;
use regex::Regex;
use std::ffi;
use std::io;
use std::process::Command;
use std::{convert::From, env, str::FromStr};
use strum::IntoEnumIterator;

const MAX_TERM_WIDTH: usize = 95;

pub struct Config {
    pub repo_path: String,
    pub ascii_input: Option<String>,
    pub ascii_language: Option<Language>,
    pub ascii_colors: Vec<String>,
    pub disabled_fields: InfoFieldOff,
    pub no_bold: bool,
    pub image: Option<DynamicImage>,
    pub image_backend: Option<Box<dyn ImageBackend>>,
    pub image_color_resolution: usize,
    pub no_merges: bool,
    pub no_color_palette: bool,
    pub number_of_authors: usize,
    pub ignored_directories: Vec<String>,
    pub bot_regex_pattern: Option<Regex>,
    pub print_languages: bool,
    pub print_package_managers: bool,
    pub output: Option<SerializationFormat>,
    pub true_color: bool,
    pub art_off: bool,
    pub text_colors: Vec<String>,
    pub iso_time: bool,
    pub show_email: bool,
    pub include_hidden: bool,
    pub language_types: Vec<LanguageType>,
    pub completion: Option<Shell>,
}

impl Config {
    pub fn new() -> Result<Self> {
        let matches = build_cli().get_matches();

        let true_color: String = matches.get_one("true-color").cloned().unwrap();
        let true_color = match true_color.as_str() {
            "always" => true,
            "never" => false,
            "auto" => is_truecolor_terminal(),
            _ => unreachable!(),
        };

        let no_bold = matches.is_present("no-bold");
        let no_merges = matches.is_present("no-merges");
        let no_color_palette = matches.is_present("no-palette");
        let print_languages = matches.is_present("languages");
        let print_package_managers = matches.is_present("package-managers");
        let iso_time = matches.is_present("isotime");
        let show_email = matches.is_present("email");
        let include_hidden = matches.is_present("hidden");

        let output = matches
            .value_of("output")
            .map(SerializationFormat::from_str)
            .transpose()?;

        let fields_to_hide: Vec<InfoField> = matches
            .get_many("disable-fields")
            .map(|fields| fields.copied().collect())
            .unwrap_or(Vec::new());

        let disabled_fields = InfoFieldOff::from_info_fields(&fields_to_hide);

        let art_off = match matches.value_of("show-logo") {
            Some("always") => false,
            Some("never") => true,
            Some("auto") => {
                if let Some((width, _)) = term_size::dimensions_stdout() {
                    width < MAX_TERM_WIDTH
                } else {
                    false
                }
            }
            _ => unreachable!(),
        };

        let image = if let Some(image_path) = matches.value_of("image") {
            Some(image::open(image_path).with_context(|| "Could not load the specified image")?)
        } else {
            None
        };

        let image_backend = if image.is_some() {
            if let Some(backend_name) = matches.value_of("image-backend") {
                image_backends::get_image_backend(backend_name)
            } else {
                image_backends::get_best_backend()
            }
        } else {
            None
        };

        let image_color_resolution: String = matches.get_one("color-resolution").cloned().unwrap();
        let image_color_resolution = image_color_resolution.parse().unwrap();

        let repo_path = matches
            .value_of("input")
            .map(String::from)
            .with_context(|| "Failed to parse input directory")?;

        let ascii_input: Option<String> = matches.get_one("ascii-input").cloned();

        let ascii_language: Option<Language> = matches.get_one("ascii-language").copied();

        let ascii_colors: Vec<u8> = matches
            .get_many("ascii-colors")
            .map(|colors| colors.copied().collect())
            .unwrap_or(Vec::new());
        let ascii_colors: Vec<String> = ascii_colors.iter().map(|n| n.to_string()).collect();

        let text_colors: Vec<u8> = matches
            .get_many("text-colors")
            .map(|colors| colors.copied().collect())
            .unwrap_or(Vec::new());
        let text_colors: Vec<String> = text_colors.iter().map(|n| n.to_string()).collect();

        let number_of_authors: usize = *matches.get_one("authors-number").unwrap();

        let ignored_directories =
            if let Some(user_ignored_directories) = matches.values_of("exclude") {
                user_ignored_directories.map(String::from).collect()
            } else {
                Vec::new()
            };

        let bot_regex_pattern: Option<Regex> = matches.contains_id("no-bots").then(|| {
            matches
                .get_one::<Regex>("no-bots")
                .cloned()
                .map_or(Regex::from_str(r"\[bot\]").unwrap(), Into::into)
        });

        let language_types: Vec<LanguageType> =
            matches.get_many("type").unwrap().copied().collect();

        let completion: Option<Shell> = matches.get_one("completion").copied();

        Ok(Config {
            repo_path,
            ascii_input,
            ascii_language,
            ascii_colors,
            disabled_fields,
            no_bold,
            image,
            image_backend,
            image_color_resolution,
            no_merges,
            no_color_palette,
            number_of_authors,
            ignored_directories,
            bot_regex_pattern,
            print_languages,
            print_package_managers,
            output,
            true_color,
            art_off,
            text_colors,
            iso_time,
            show_email,
            include_hidden,
            language_types,
            completion,
        })
    }
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

pub fn print_completions<G: Generator>(gen: G) {
    let mut cmd = build_cli();
    let name = cmd.get_name().to_string();
    generate(gen, &mut cmd, name, &mut io::stdout());
}

pub fn build_cli() -> clap::Command<'static> {
    let possible_backends = ["kitty", "iterm", "sixel"];

    let color_values = value_parser!(u8).range(..16);

    clap::Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .setting(AppSettings::DeriveDisplayOrder)
        .hide_possible_values(true)
        .arg(
            Arg::new("input")
            .default_value(".")
            .hide_default_value(true)
            .help("Run as if onefetch was started in <input> instead of the current working directory.")
            .value_hint(ValueHint::DirPath)
        )
        .arg(
            Arg::new("output")
            .short('o')
            .long("output")
            .value_name("FORMAT")
            .help("Outputs Onefetch in a specific format (json, yaml).")
            .takes_value(true)
            .possible_values(
                SerializationFormat::iter()
                .map(|format| format.into())
                .collect::<Vec<&str>>())
        )
        .arg(
            Arg::new("languages")
            .short('l')
            .long("languages")
            .help("Prints out supported languages."),
        )
        .arg(
            Arg::new("package-managers")
            .short('p')
            .long("package-managers")
            .help("Prints out supported package managers."),
        )
        .arg(
            Arg::new("show-logo")
            .long("show-logo")
            .value_name("WHEN")
            .takes_value(true)
            .possible_values(&["auto", "never", "always"])
            .default_value("always")
            .hide_default_value(true)
            .help("Specify when to show the logo (auto, never, *always*).")
            .long_help(
                "Specify when to show the logo (auto, never, *always*). \n\
                If set to auto: the logo will be hidden if the terminal's width < 95.")
        )
        .arg(
            Arg::new("image")
            .short('i')
            .long("image")
            .value_name("IMAGE")
            .takes_value(true)
            .help("Path to the IMAGE file.")
            .value_hint(ValueHint::FilePath)
        )
        .arg(
            Arg::new("image-backend")
            .long("image-backend")
            .value_name("BACKEND")
            .takes_value(true)
            .requires("image")
            .possible_values(possible_backends)
            .help("Which image BACKEND to use."),
        )
        .arg(
            Arg::new("color-resolution")
            .long("color-resolution")
            .value_name("VALUE")
            .requires("image")
            .takes_value(true)
            .value_parser(["16", "32", "64", "128", "256"])
            .default_value("16")
            .help("VALUE of color resolution to use with SIXEL backend."),
        )
        .arg(
            Arg::new("ascii-language")
           .short('a')
           .value_name("LANGUAGE")
           .long("ascii-language")
           .takes_value(true)
           .ignore_case(true)
           .help("Which LANGUAGE's ascii art to print.")
           .value_parser(value_parser!(Language))
        )
        .arg(
            Arg::new("ascii-input")
            .long("ascii-input")
            .value_name("STRING")
            .takes_value(true)
            .help("Takes a non-empty STRING as input to replace the ASCII logo.")
            .long_help(
                "Takes a non-empty STRING as input to replace the ASCII logo. \
                It is possible to pass a generated STRING by command substitution. \n\
                For example:\n \
                '--ascii-input \"$(fortune | cowsay -W 25)\"'")
            .value_parser(clap::builder::NonEmptyStringValueParser::new()),
        )
        .arg(
            Arg::new("true-color")
            .long("true-color")
            .value_name("WHEN")
            .takes_value(true)
            .value_parser(["auto", "never", "always"])
            .default_value("auto")
            .hide_default_value(true)
            .help("Specify when to use true color (*auto*, never, always).")
            .long_help(
                "Specify when to use true color (*auto*, never, always). \n\
                If set to auto: true color will be enabled if supported by the terminal.")
        )
        .arg(
            Arg::new("ascii-colors")
            .short('c')
            .long("ascii-colors")
            .value_name("X")
            .multiple_values(true)
            .takes_value(true)
            .value_parser(color_values)
            .help("Colors (X X X...) to print the ascii art."),
        )
        .arg(
            Arg::new("text-colors")
            .short('t')
            .long("text-colors")
            .value_name("X")
            .multiple_values(true)
            .takes_value(true)
            .max_values(6)
            .value_parser(color_values)
            .help("Changes the text colors (X X X...).")
            .long_help(
                "Changes the text colors (X X X...). \
                Goes in order of title, ~, underline, subtitle, colon, and info. \n\
                For example:\n \
                '--text-colors 9 10 11 12 13 14'")
        )
        .arg(
            Arg::new("no-bold")
            .long("no-bold")
            .help("Turns off bold formatting."),
        )
        .arg(
            Arg::new("no-palette")
            .long("no-palette")
            .help("Hides the color palette."),
        )
        .arg(
            Arg::new("no-merges")
            .long("no-merges")
            .help("Ignores merge commits."),
        )
        .arg(
            Arg::new("no-bots")
            .long("no-bots")
            .min_values(0)
            .max_values(1)
            .value_name("REGEX")
            .help("Exclude [bot] commits. Use <REGEX> to override the default pattern.")
            .value_parser(CliRegexParser),
        )
        .arg(
            Arg::new("isotime")
            .short('z')
            .long("isotime")
            .help("Use ISO 8601 formatted timestamps.")
        )
        .arg(
            Arg::new("disable-fields")
            .long("disable-fields")
            .short('d')
            .value_name("FIELD")
            .multiple_values(true)
            .takes_value(true)
            .ignore_case(true)
            .help("Allows you to disable FIELD(s) from appearing in the output.")
            .value_parser(value_parser!(InfoField))
        )
        .arg(
            Arg::new("authors-number")
            .short('A')
            .long("authors-number")
            .value_name("NUM")
            .takes_value(true)
            .default_value("3")
            .help("NUM of authors to be shown.")
            .value_parser(value_parser!(usize))
        )
        .arg(
            Arg::new("email")
            .short('E')
            .long("email")
            .help("show the email address of each author.")
        )
        .arg(
            Arg::new("hidden")
            .long("hidden")
            .help("Count hidden files and directories.")
        )
        .arg(
            Arg::new("exclude")
            .short('e')
            .long("exclude")
            .value_name("EXCLUDE")
            .multiple_values(true)
            .takes_value(true)
            .help("Ignore all files & directories matching EXCLUDE.")
            .value_hint(ValueHint::AnyPath)
        )
        .arg(
            Arg::new("type")
            .short('T')
            .long("type")
            .value_name("TYPE")
            .multiple_values(true)
            .takes_value(true)
            .ignore_case(true)
            .default_values(&["programming", "markup"])
            .hide_default_value(true)
            .help("Filters output by language type (*programming*, *markup*, prose, data).")
            .value_parser(value_parser!(LanguageType))
        )
        .arg(
            Arg::new("completion")
            .long("completion")
            .value_parser(value_parser!(Shell))
            .value_name("SHELL")
            .help("Prints out SHELL completion script.")
        )
}

#[derive(Clone, Debug)]
struct RegexWrapper(Regex);

impl clap::builder::ValueParserFactory for RegexWrapper {
    type Parser = CliRegexParser;

    fn value_parser() -> Self::Parser {
        CliRegexParser
    }
}

#[derive(Clone, Debug)]
struct CliRegexParser;

impl clap::builder::TypedValueParser for CliRegexParser {
    type Value = Regex;

    fn parse_ref(
        &self,
        _: &clap::Command,
        _: Option<&Arg>,
        value: &ffi::OsStr,
    ) -> std::result::Result<Self::Value, clap_error::Error> {
        Regex::from_str(&value.to_string_lossy())
            .map_err(|e| clap_error::Error::raw(clap_error::ErrorKind::InvalidValue, e))
    }
}

impl From<RegexWrapper> for Regex {
    fn from(wrapper: RegexWrapper) -> Regex {
        wrapper.0
    }
}
