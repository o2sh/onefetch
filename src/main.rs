#[macro_use]
extern crate clap;

#[cfg(target_os = "linux")]
use image_backends::ImageBackend;
use {
    ascii_art::AsciiArt,
    commit_info::CommitInfo,
    error::Error,
    exit_codes::ExitCode,
    info::{Info, InfoFieldOn, InfoFields},
    language::Language,
    process::{Command, Stdio},
    std::{convert::From, env, process, result, str::FromStr},
    strum::IntoEnumIterator,
};

mod app;
mod ascii_art;
mod commit_info;
mod error;
mod exit_codes;
mod image_backends;
mod info;
mod language;
mod license;
mod options;

type Result<T> = result::Result<T, Error>;

fn run() -> Result<()> {
    #[cfg(target_os = "windows")]
    let enabled = ansi_term::enable_ansi_support().is_ok();

    #[cfg(not(target_os = "windows"))]
    let enabled = true;

    if enabled {
        colored::control::set_override(true);
    }

    if !is_git_installed() {
        return Err(Error::GitNotInstalled);
    }

    let matches = app::build_app().get_matches_from(env::args_os());

    if matches.is_present("languages") {
        let iterator = Language::iter().filter(|x| *x != Language::Unknown);

        for l in iterator {
            println!("{}", l);
        }
        std::process::exit(0);
    }

    let mut disabled_fields = InfoFieldOn {
        ..Default::default()
    };

    let fields_to_hide: Vec<String> = if let Some(values) = matches.values_of("disable-fields") {
        values.map(String::from).collect()
    } else {
        Vec::new()
    };

    for field in fields_to_hide.iter() {
        let item = InfoFields::from_str(field.to_lowercase().as_str())
            .unwrap_or(InfoFields::UnrecognizedField);

        match item {
            InfoFields::GitInfo => disabled_fields.git_info = true,
            InfoFields::Project => disabled_fields.project = true,
            InfoFields::HEAD => disabled_fields.head = true,
            InfoFields::Version => disabled_fields.version = true,
            InfoFields::Created => disabled_fields.created = true,
            InfoFields::Languages => disabled_fields.languages = true,
            InfoFields::Authors => disabled_fields.authors = true,
            InfoFields::LastChange => disabled_fields.last_change = true,
            InfoFields::Repo => disabled_fields.repo = true,
            InfoFields::Pending => disabled_fields.pending = true,
            InfoFields::Commits => disabled_fields.commits = true,
            InfoFields::LinesOfCode => disabled_fields.lines_of_code = true,
            InfoFields::Size => disabled_fields.size = true,
            InfoFields::License => disabled_fields.license = true,
            _ => (),
        }
    }

    let image = if let Some(image_path) = matches.value_of("image") {
        Some(image::open(image_path).map_err(|_| Error::ImageLoadError)?)
    } else {
        None
    };

    let image_backend = if image.is_some() {
        if let Some(backend_name) = matches.value_of("image-backend") {
            #[cfg(target_os = "linux")]
            let backend =
                Some(match backend_name {
                    "kitty" => Box::new(image_backends::kitty::KittyBackend::new())
                        as Box<dyn ImageBackend>,
                    "sixel" => Box::new(image_backends::sixel::SixelBackend::new())
                        as Box<dyn ImageBackend>,
                    _ => unreachable!(),
                });
            #[cfg(not(target_os = "linux"))]
            let backend = None;
            backend
        } else {
            crate::image_backends::get_best_backend()
        }
    } else {
        None
    };

    let config = options::Options {
        path: String::from(matches.value_of("input").unwrap()),
        ascii_language: if let Some(ascii_language) = matches.value_of("ascii-language") {
            Language::from_str(&ascii_language.to_lowercase()).unwrap()
        } else {
            Language::Unknown
        },
        ascii_colors: if let Some(values) = matches.values_of("ascii-colors") {
            values.map(String::from).collect()
        } else {
            Vec::new()
        },
        disabled_fields,
        no_bold: !matches.is_present("no-bold"),
        image: image,
        image_backend,
        no_merges: matches.is_present("no-merge-commits"),
        no_color_blocks: matches.is_present("no-color-blocks"),
        number_of_authors: if let Some(value) = matches.value_of("authors-number") {
            usize::from_str(value).unwrap()
        } else {
            3
        },
        excluded: if let Some(user_ignored) = matches.values_of("exclude") {
            user_ignored.map(String::from).collect()
        } else {
            Vec::new()
        },
    };

    let info = Info::new(config)?;

    print!("{}", info);
    Ok(())
}

fn main() {
    let result = run();
    match result {
        Ok(_) => {
            process::exit(ExitCode::Success.into());
        }
        Err(_) => {
            process::exit(ExitCode::GeneralError.into());
        }
    }
}

fn is_git_installed() -> bool {
    Command::new("git")
        .arg("--version")
        .stdout(Stdio::null())
        .status()
        .is_ok()
}
