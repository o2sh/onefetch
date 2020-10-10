use {
    ascii_art::AsciiArt,
    colored::Colorize,
    commit_info::CommitInfo,
    error::Error,
    exit_codes::ExitCode,
    info::Info,
    language::Language,
    process::{Command, Stdio},
    std::{convert::From, env, io::Write, process, result, str::FromStr},
    strum::IntoEnumIterator,
};

mod ascii_art;
mod clap_app;
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

    let image_backend = if image.is_some() {
        if let Some(backend_name) = matches.value_of("image-backend") {
            image_backends::get_image_backend(backend_name)
        } else {
            image_backends::get_best_backend()
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
    Ok(())
}

pub fn list_languages() -> Result<()> {
    let iterator = Language::iter().filter(|x| *x != Language::Unknown);

    for l in iterator {
        println!("{}", l);
    }

    Ok(())
}

fn main() {
    let result = run();
    match result {
        Ok(_) => {
            process::exit(ExitCode::Success.into());
        }
        Err(error) => {
            let stderr = std::io::stderr();
            default_error_handler(&error, &mut stderr.lock());
            process::exit(ExitCode::GeneralError.into());
        }
    }
}

pub fn default_error_handler(error: &Error, output: &mut dyn Write) {
    writeln!(output, "{}: {}", "[onefetch error]".red(), error).ok();
}

fn is_git_installed() -> bool {
    Command::new("git")
        .arg("--version")
        .stdout(Stdio::null())
        .status()
        .is_ok()
}
