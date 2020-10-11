use {
    crate::onefetch::{info::InfoFields, language::Language},
    clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg},
    strum::{EnumCount, IntoEnumIterator},
};

pub fn build_app() -> App<'static, 'static> {
    #[cfg(target_os = "linux")]
    let possible_backends = ["kitty", "sixel"];
    #[cfg(not(target_os = "linux"))]
    let possible_backends = [];

    App::new(crate_name!())
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
        )
}
