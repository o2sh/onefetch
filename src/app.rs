use {
    crate::info::InfoFields,
    clap::{App, AppSettings, Arg},
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
        .arg(Arg::with_name("input").default_value(".").help(
            "Run as if onefetch was started in <input> instead of the current working directory.",
        ))
        .arg(
            Arg::with_name("ascii-language")
                .short("a")
                .long("ascii-language")
                .takes_value(true)
                .case_insensitive(true)
                .help("Which language's ascii art to print."),
        )
        .arg(
            Arg::with_name("disable-fields")
                .long("disable-fields")
                .short("d")
                .multiple(true)
                .takes_value(true)
                .case_insensitive(true)
                .help("Allows you to disable an info line from appearing in the output.")
                .possible_values(
                    &InfoFields::iter()
                        .take(InfoFields::COUNT - 1)
                        .map(|field| field.into())
                        .collect::<Vec<&str>>()
                        .as_slice(),
                ),
        )
        .arg(
            Arg::with_name("ascii-colors")
                .short("c")
                .long("ascii-colors")
                .multiple(true)
                .takes_value(true)
                .possible_values(&[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ])
                .help("Colors to print the ascii art."),
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
                .takes_value(true)
                .help("Which image to use. Possible values: [/path/to/img]"),
        )
        .arg(
            Arg::with_name("image-backend")
                .long("image-backend")
                .takes_value(true)
                .possible_values(&possible_backends)
                .help("Which image backend to use."),
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
                .takes_value(true)
                .default_value("3")
                .help("Number of authors to be shown."),
        )
        .arg(
            Arg::with_name("exclude")
                .short("e")
                .long("exclude")
                .multiple(true)
                .takes_value(true)
                .help("Ignore all files & directories matching the pattern."),
        )
}
