use {
    crate::onefetch::{
        cli_utils,
        error::*,
        image_backends,
        info_field::{InfoField, InfoFieldOff},
        language::Language,
        printer::SerializationFormat,
    },
    clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg},
    image::DynamicImage,
    std::{convert::From, env, str::FromStr},
    strum::IntoEnumIterator,
    term_size,
};

const MAX_TERM_WIDTH: usize = 95;

pub struct Cli {
    pub repo_path: String,
    pub ascii_input: Option<String>,
    pub ascii_language: Option<Language>,
    pub ascii_colors: Vec<String>,
    pub disabled_fields: InfoFieldOff,
    pub no_bold: bool,
    pub image: Option<DynamicImage>,
    pub image_backend: Option<Box<dyn image_backends::ImageBackend>>,
    pub image_color_resolution: usize,
    pub no_merges: bool,
    pub no_color_palette: bool,
    pub number_of_authors: usize,
    pub excluded: Vec<String>,
    pub print_languages: bool,
    pub print_package_managers: bool,
    pub output: Option<SerializationFormat>,
    pub true_color: bool,
    pub art_off: bool,
    pub text_colors: Vec<String>,
    pub iso_time: bool,
}

impl Cli {
    pub fn new() -> Result<Self> {
        #[cfg(not(windows))]
        let possible_backends = ["kitty", "iterm", "sixel"];
        #[cfg(windows)]
        let possible_backends = [];
        let color_values =
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15"];
        let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::UnifiedHelpMessage)
        .setting(AppSettings::HidePossibleValuesInHelp)
        .arg(
            Arg::with_name("input")
            .default_value(".")
            .hide_default_value(true)
            .help("Run as if onefetch was started in <input> instead of the current working directory.")
        )
        .arg(
            Arg::with_name("output")
            .short("o")
            .long("output")
            .help("Outputs Onefetch in a specific format (json, yaml).")
            .takes_value(true)
            .possible_values(&SerializationFormat::iter()
            .map(|format| format.into())
            .collect::<Vec<&str>>())
        )
        .arg(
            Arg::with_name("languages")
            .short("l")
            .long("languages")
            .help("Prints out supported languages."),
        )
        .arg(
            Arg::with_name("package-managers")
            .short("p")
            .long("package-managers")
            .help("Prints out supported package managers."),
        )
        .arg(
            Arg::with_name("show-logo")
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
            Arg::with_name("image")
            .short("i")
            .long("image")
            .value_name("IMAGE")
            .takes_value(true)
            .help("Path to the IMAGE file."),
        )
        .arg(
            Arg::with_name("image-backend")
            .long("image-backend")
            .value_name("BACKEND")
            .takes_value(true)
            .requires("image")
            .possible_values(&possible_backends)
            .help("Which image BACKEND to use."),
        )
        .arg(
            Arg::with_name("color-resolution")
            .long("color-resolution")
            .value_name("VALUE")
            .requires("image")
            .takes_value(true)
            .possible_values(&["16", "32", "64", "128", "256"])
            .help("VALUE of color resolution to use with SIXEL backend."),
        )
        .arg(
            Arg::with_name("ascii-language")
           .short("a")
           .value_name("LANGUAGE")
           .long("ascii-language")
           .takes_value(true)
           .case_insensitive(true)
           .help("Which LANGUAGE's ascii art to print.")
           .possible_values(
               &Language::iter()
               .map(|language| language.into())
               .collect::<Vec<&str>>())
        )
        .arg(
            Arg::with_name("ascii-input")
            .long("ascii-input")
            .value_name("STRING")
            .takes_value(true)
            .help("Takes a non-empty STRING as input to replace the ASCII logo.")
            .long_help(
                "Takes a non-empty STRING as input to replace the ASCII logo. \
                It is possible to pass a generated STRING by command substitution. \n\
                For example:\n \
                '--ascii-input \"$(fortune | cowsay -W 25)\"'")
            .validator(
                |t| {
                    if t.is_empty() {
                        return Err(String::from("must not be empty"));
                    }
                    Ok(())
                },
            ),
        )
        .arg(
            Arg::with_name("true-color")
            .long("true-color")
            .value_name("WHEN")
            .takes_value(true)
            .possible_values(&["auto", "never", "always"])
            .default_value("auto")
            .hide_default_value(true)
            .help("Specify when to use true color (*auto*, never, always).")
            .long_help(
                "Specify when to use true color (*auto*, never, always). \n\
                If set to auto: true color will be enabled if supported by the terminal.")
        )
        .arg(
            Arg::with_name("ascii-colors")
            .short("c")
            .long("ascii-colors")
            .value_name("X")
            .multiple(true)
            .takes_value(true)
            .possible_values(color_values)
            .help("Colors (X X X...) to print the ascii art."),
        )
        .arg(
            Arg::with_name("text-colors")
            .short("t")
            .long("text-colors")
            .value_name("X")
            .multiple(true)
            .takes_value(true)
            .max_values(6)
            .possible_values(color_values)
            .help("Changes the text colors (X X X...).")
            .long_help(
                "Changes the text colors (X X X...). \
                Goes in order of title, ~, underline, subtitle, colon, and info. \n\
                For example:\n \
                '--text-colors 9 10 11 12 13 14'")
        )
        .arg(
            Arg::with_name("no-bold")
            .long("no-bold")
            .help("Turns off bold formatting."),
        )
        .arg(
            Arg::with_name("no-color-palette")
            .long("no-color-palette")
            .help("Hides the color palette."),
        )
        .arg(
            Arg::with_name("no-merge-commits")
            .long("no-merge-commits")
            .help("Ignores merge commits."),
        )
        .arg(
            Arg::with_name("isotime")
            .short("z")
            .long("isotime")
            .help("Use ISO 8601 formatted timestamps.")
        )
        .arg(
            Arg::with_name("disable-fields")
            .long("disable-fields")
            .short("d")
            .value_name("FIELD")
            .multiple(true)
            .takes_value(true)
            .case_insensitive(true)
            .help("Allows you to disable FIELD(s) from appearing in the output.")
            .possible_values(
                &InfoField::iter()
                    .map(|field| field.into())
                    .collect::<Vec<&str>>())
        )
        .arg(
            Arg::with_name("authors-number")
            .short("A")
            .long("authors-number")
            .value_name("NUM")
            .takes_value(true)
            .default_value("3")
            .help("NUM of authors to be shown.")
            .validator(
                |t| {
                    t.parse::<u32>()
                        .map_err(|_t| "must be a number")
                        .map(|_t|())
                        .map_err(|e| e.to_string())
                })
        )
        .arg(
            Arg::with_name("exclude")
            .short("e")
            .long("exclude")
            .value_name("EXCLUDE")
            .multiple(true)
            .takes_value(true)
            .help("Ignore all files & directories matching EXCLUDE."),
        )
        .get_matches();

        let true_color = match matches.value_of("true-color") {
            Some("always") => true,
            Some("never") => false,
            Some("auto") => cli_utils::is_truecolor_terminal(),
            _ => unreachable!(),
        };
        let no_bold = matches.is_present("no-bold");
        let no_merges = matches.is_present("no-merge-commits");
        let no_color_palette = matches.is_present("no-color-palette");
        let print_languages = matches.is_present("languages");
        let print_package_managers = matches.is_present("package-managers");
        let iso_time = matches.is_present("isotime");

        let output =
            matches.value_of("output").map(SerializationFormat::from_str).transpose().unwrap();

        let fields_to_hide: Vec<String> = if let Some(values) = matches.values_of("disable-fields")
        {
            values.map(String::from).collect()
        } else {
            Vec::new()
        };

        let disabled_fields = InfoFieldOff::new(fields_to_hide)?;

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
            _ => unreachable!("other values for --hide-logo are not allowed"),
        };

        let image = if let Some(image_path) = matches.value_of("image") {
            Some(image::open(image_path).chain_err(|| "Could not load the specified image")?)
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

        if image.is_some() && image_backend.is_none() {
            return Err("Could not detect a supported image backend".into());
        }

        let image_color_resolution = if let Some(value) = matches.value_of("color-resolution") {
            usize::from_str(value).unwrap()
        } else {
            16
        };

        let repo_path = String::from(matches.value_of("input").unwrap());

        let ascii_input = matches.value_of("ascii-input").map(String::from);

        let ascii_language = if let Some(ascii_language) = matches.value_of("ascii-language") {
            Some(Language::from_str(&ascii_language.to_lowercase()).unwrap())
        } else {
            None
        };

        let ascii_colors = if let Some(values) = matches.values_of("ascii-colors") {
            values.map(String::from).collect()
        } else {
            Vec::new()
        };

        let text_colors = if let Some(values) = matches.values_of("text-colors") {
            values.map(String::from).collect()
        } else {
            Vec::new()
        };

        let number_of_authors: usize = matches.value_of("authors-number").unwrap().parse().unwrap();

        let excluded = if let Some(user_ignored) = matches.values_of("exclude") {
            user_ignored.map(String::from).collect()
        } else {
            Vec::new()
        };

        Ok(Cli {
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
            excluded,
            print_languages,
            print_package_managers,
            output,
            true_color,
            text_colors,
            art_off,
            iso_time,
        })
    }
}
