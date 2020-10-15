use {
    crate::onefetch::{
        error::*, image_backends, info_fields, info_fields::InfoFields, language::Language,
    },
    clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg},
    image::DynamicImage,
    std::{convert::From, env, str::FromStr},
    strum::{EnumCount, IntoEnumIterator},
};

pub struct Cli {
    pub path: String,
    pub ascii_language: Language,
    pub ascii_colors: Vec<String>,
    pub disabled_fields: info_fields::InfoFieldOn,
    pub no_bold: bool,
    pub image: Option<DynamicImage>,
    pub image_backend: Option<Box<dyn image_backends::ImageBackend>>,
    pub no_merges: bool,
    pub no_color_blocks: bool,
    pub number_of_authors: usize,
    pub excluded: Vec<String>,
    pub print_languages: bool,
}

impl Cli {
    /// Build `Options` from command line arguments.
    pub fn new() -> Result<Self> {
        #[cfg(not(target_os = "windows"))]
        let possible_backends = ["kitty", "sixel"];
        #[cfg(target_os = "windows")]
        let possible_backends = [];

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
            Arg::with_name("ascii-colors")
                .short("c")
                .long("ascii-colors")
                .value_name("X")
                .multiple(true)
                .takes_value(true)
                .possible_values(&[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ])
                .help("Colors (X X X...) to print the ascii art."),
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
                .help("Prints out supported languages"),
        )
        .arg(
            Arg::with_name("image")
                .short("i")
                .long("image")
                .value_name("IMAGE")
                .takes_value(true)
                .max_values(1)
                .help("Path to the IMAGE file"),
        )
        .arg(
            Arg::with_name("image-backend")
                .long("image-backend")
                .value_name("BACKEND")
                .takes_value(true)
                .max_values(1)
                .possible_values(&possible_backends)
                .help("Which image BACKEND to use."),
        )
        .arg(
            Arg::with_name("no-merge-commits")
                .long("no-merge-commits")
                .help("Ignores merge commits"),
        )
        .arg(
            Arg::with_name("no-color-blocks")
                .long("no-color-blocks")
                .help("Hides the color blocks"),
        )
        .arg(
            Arg::with_name("authors-number")
                .short("A")
                .long("authors-number")
                .value_name("NUM")
                .takes_value(true)
                .max_values(1)
                .default_value("3")
                .help("NUM of authors to be shown."),
        )
        .arg(
            Arg::with_name("exclude")
                .short("e")
                .long("exclude")
                .value_name("EXCLUDE")
                .multiple(true)
                .takes_value(true)
                .help("Ignore all files & directories matching EXCLUDE."),
            ).get_matches();

        let no_bold = matches.is_present("no-bold");
        let no_merges = matches.is_present("no-merge-commits");
        let no_color_blocks = matches.is_present("no-color-blocks");
        let print_languages = matches.is_present("languages");

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
                image_backends::get_image_backend(backend_name)
            } else {
                image_backends::get_best_backend()
            }
        } else {
            None
        };

        let path = String::from(matches.value_of("input").unwrap());
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

        let disabled_fields = info_fields::get_disabled_fields(fields_to_hide)?;

        let number_of_authors = if let Some(value) = matches.value_of("authors-number") {
            usize::from_str(value).unwrap()
        } else {
            3
        };

        let excluded = if let Some(user_ignored) = matches.values_of("exclude") {
            user_ignored.map(String::from).collect()
        } else {
            Vec::new()
        };

        Ok(Cli {
            path,
            ascii_language,
            ascii_colors,
            disabled_fields,
            no_bold,
            image,
            image_backend,
            no_merges,
            no_color_blocks,
            number_of_authors,
            excluded,
            print_languages,
        })
    }

    pub fn print_supported_languages() -> Result<()> {
        let iterator = Language::iter().filter(|x| *x != Language::Unknown);

        for l in iterator {
            println!("{}", l);
        }

        Ok(())
    }
}
