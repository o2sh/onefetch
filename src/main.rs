#[macro_use]
extern crate clap;

#[cfg(target_os = "linux")]
use {
    ascii_art::AsciiArt,
    commit_info::CommitInfo,
    error::Error,
    exit_codes::ExitCode,
    info::Info,
    language::Language,
    process::{Command, Stdio},
    std::{convert::From, env, process, result, str::FromStr},
    strum::IntoEnumIterator,
};

mod clap_app;
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

/// Returns `Err(..)` upon fatal errors. Otherwise, returns `Ok(true)` on full success and
/// `Ok(false)` if any intermediate errors occurred (were printed).
fn run() -> Result<bool> {
    #[cfg(windows)]
    let _ = ansi_term::enable_ansi_support();

    if !is_git_installed() {
        return Err(Error::GitNotInstalled);
    }

    let matches = clap_app::build_app().get_matches_from(env::args_os());

    if matches.is_present("languages") {
        return list_languages();
    }

    let fields_to_hide: Vec<String> = if let Some(values) = matches.values_of("disable-fields") {
        values.map(String::from).collect()
    } else {
        Vec::new()
    };

    let image = if let Some(image_path) = matches.value_of("image") {
        Some(image::open(image_path).map_err(|_| Error::ImageLoadError)?)
    } else {
        None
    };

    let image_backend = if let Some(backend_name) = matches.value_of("image-backend") {
        image_backends::get_image_backend(&image, backend_name)
    } else {
        image_backends::get_best_backend()
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
        disabled_fields: info::get_disabled_fields(fields_to_hide)?,
        no_bold: !matches.is_present("no-bold"),
        image,
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
    Ok(true)
}

pub fn list_languages() -> Result<bool> {
    let iterator = Language::iter().filter(|x| *x != Language::Unknown);

    for l in iterator {
        println!("{}", l);
    }

    Ok(true)
}

fn main() {
    let result = run();
    match result {
        Ok(true) => {
            process::exit(ExitCode::Success.into());
        }
        Ok(false) => {
            process::exit(ExitCode::GeneralError.into());
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
