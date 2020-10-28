use {
    crate::onefetch::{
        cli_utils,
        error::*,
        image_backends,
        info_fields::{self, InfoFields},
        language::Language,
    },
    clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg},
    image::DynamicImage,
    std::{convert::From, env, str::FromStr},
    strum::{EnumCount, IntoEnumIterator},
};

pub struct Cli {
    pub path: String,
    pub ascii_input: Option<String>,
    pub ascii_language: Language,
    pub ascii_colors: Vec<String>,
    pub disabled_fields: info_fields::InfoFieldOn,
    pub no_bold: bool,
    pub image: Option<DynamicImage>,
    pub image_backend: Option<Box<dyn image_backends::ImageBackend>>,
    pub image_colors: usize,
    pub no_merges: bool,
    pub no_color_palette: bool,
    pub number_of_authors: usize,
    pub excluded: Vec<String>,
    pub print_languages: bool,
    pub true_color: bool,
    pub art_off: bool,
    pub text_colors: Vec<String>,
}

impl Cli {
    /// Build `Options` from command line arguments.
    pub fn new() -> Result<Self> {
        #[cfg(not(windows))]
        let possible_backends = ["kitty", "sixel"];
        #[cfg(windows)]
        let possible_backends = [];
        let color_values = &[
            "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15",
        ];
        let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::UnifiedHelpMessage)
        .setting(AppSettings::HidePossibleValuesInHelp)
        .arg(Arg::with_name("input").default_value(".").hide_default_value(true).help(
            "Run as if onefetch was started in <input> instead of the current working directory.",
        ))
        .arg(
            Arg::with_name("ascii-language")
                .short("a")
                .value_name("LANGUAGE")
                .long("ascii-language")
                .max_values(1)
                .takes_value(true)
                .case_insensitive(true)
                .help("Which LANGUAGE's ascii art to print.")
                .possible_values(
                    &Language::iter()
                        .filter(|language| *language != Language::Unknown)
                        .map(|language| language.into())
                        .collect::<Vec<&str>>()
                    ),
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
                    &InfoFields::iter()
                        .take(InfoFields::COUNT - 1)
                        .map(|field| field.into())
                        .collect::<Vec<&str>>()
                ),
        )
        .arg(
            Arg::with_name("ascii-input")
                .long("ascii-input")
                .value_name("STRING")
                .takes_value(true)
                .max_values(1)
                .help("Takes a non-empty STRING as input to replace the ASCII logo.")
                .long_help(
                    "Takes a non-empty STRING as input to replace the ASCII logo. \
                     It is possible to pass a generated STRING by command substitution. \
                     For example:\n \
                     '--ascii-input \"$(fortune | cowsay -W 25)\"'"
            )
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
                     Goes in order of title, ~, underline, subtitle, colon, and info. \
                     For example:\n \
                     '--text-color 9 10 11 12 13 14'"
            )
        )
        .arg(
            Arg::with_name("no-bold")
                .long("no-bold")
                .help("Turns off bold formatting."),
        )
        .arg(
            Arg::with_name("languages")
                .short("l")
                .long("languages")
                .help("Prints out supported languages."),
        )
        .arg(
            Arg::with_name("image")
                .short("i")
                .long("image")
                .value_name("IMAGE")
                .takes_value(true)
                .max_values(1)
                .help("Path to the IMAGE file."),
        )
        .arg(
            Arg::with_name("image-backend")
                .long("image-backend")
                .value_name("BACKEND")
                .takes_value(true)
                .requires("image")
                .max_values(1)
                .possible_values(&possible_backends)
                .help("Which image BACKEND to use."),
        )
        .arg(
            Arg::with_name("color-resolution")
                .long("color-resolution")
                .value_name("VALUE")
                .requires("image")
                .takes_value(true)
                .max_values(1)
                .possible_values(&["16", "32", "64", "128", "256"])
                .help("VALUE of color resolution to use with SIXEL backend."),
        )
        .arg(
            Arg::with_name("no-merge-commits")
                .long("no-merge-commits")
                .help("Ignores merge commits."),
        )
        .arg(
            Arg::with_name("no-color-palette")
                .long("no-color-palette")
                .help("Hides the color palette."),
        )
        .arg(
            Arg::with_name("authors-number")
                .short("A")
                .long("authors-number")
                .value_name("NUM")
                .takes_value(true)
                .max_values(1)
                .default_value("3")
                .help("NUM of authors to be shown.")
                .validator(
                    |t| {
                        t.parse::<u32>()
                            .map_err(|_t| "must be a number")
                            .map(|_t|())
                            .map_err(|e| e.to_string())
                    },
                )
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
        .arg(
            Arg::with_name("off")
                .long("off")
                .help("Only shows the info lines.")
                .conflicts_with_all(&["image", "ascii-language", "ascii-input"]), 
            ).get_matches();

        let no_bold = matches.is_present("no-bold");
        let no_merges = matches.is_present("no-merge-commits");
        let no_color_palette = matches.is_present("no-color-palette");
        let print_languages = matches.is_present("languages");
        let art_off = matches.is_present("off");
        let true_color = cli_utils::is_truecolor_terminal();

        let fields_to_hide: Vec<String> = if let Some(values) = matches.values_of("disable-fields")
        {
            values.map(String::from).collect()
        } else {
            Vec::new()
        };

        let image = if let Some(image_path) = matches.value_of("image") {
            Some(image::open(image_path).chain_err(|| "Could not load the specified image")?)
        } else {
            None
        };

        let image_backend = if image.is_some() {
            if let Some(backend_name) = matches.value_of("image-backend") {
                image_backends::check_if_supported(backend_name)?;
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

        let image_colors = if let Some(value) = matches.value_of("color-resolution") {
            usize::from_str(value).unwrap()
        } else {
            16
        };

        let path = String::from(matches.value_of("input").unwrap());

        let ascii_input = matches.value_of("ascii-input").map(String::from);

        let ascii_language = if let Some(ascii_language) = matches.value_of("ascii-language") {
            Language::from_str(&ascii_language.to_lowercase()).unwrap()
        } else {
            Language::Unknown
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

        let disabled_fields = info_fields::get_disabled_fields(fields_to_hide)?;

        let number_of_authors: usize = matches.value_of("authors-number").unwrap().parse().unwrap();

        let excluded = if let Some(user_ignored) = matches.values_of("exclude") {
            user_ignored.map(String::from).collect()
        } else {
            Vec::new()
        };

        Ok(Cli {
            path,
            ascii_input,
            ascii_language,
            ascii_colors,
            disabled_fields,
            no_bold,
            image,
            image_backend,
            image_colors,
            no_merges,
            no_color_palette,
            number_of_authors,
            excluded,
            print_languages,
            true_color,
            text_colors,
            art_off,
        })
    }
}
