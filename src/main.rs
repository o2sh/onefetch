extern crate bytecount;
extern crate colored;
extern crate git2;
extern crate license;
extern crate tokei;
#[macro_use]
extern crate clap;

use clap::{App, Arg};
use colored::Color;
use license::License;
use std::{collections::HashMap, convert::From, fs, result, str::FromStr};

mod configuration;
mod error;
mod info;
mod language;
mod lib;

use configuration::get_configuration;
use error::{Error, Result};
use info::Info;
use language::Language;

fn main() -> Result<()> {
    if !is_git_installed() {
        return Err(Error::GitNotInstalled);
    }

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author("o2sh <ossama-hjaji@live.fr>")
        .about(crate_description!())
        .arg(
            Arg::with_name("directory")
                .short("d")
                .long("dir")
                .takes_value(true)
                .default_value("."),
        )
        .arg(
            Arg::with_name("ascii_language")
                .short("a")
                .long("ascii_language")
                .takes_value(true)
                .default_value("")
                .help("Overrides showing the dominant language ascii logo"),
        )
        .get_matches();
    let dir = String::from(matches.value_of("directory").unwrap());
    let custom_logo: Language =
        Language::from_str(&matches.value_of("ascii_language").unwrap().to_lowercase())
            .unwrap_or(Language::Unknown);

    let tokei_langs = project_languages(&dir);
    let languages_stat = get_languages_stat(&tokei_langs).ok_or(Error::SourceCodeNotFound)?;
    let mut languages_stat_vec: Vec<(_, _)> = languages_stat.into_iter().collect();
    languages_stat_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());
    let dominant_language = languages_stat_vec[0].0.clone();

    let authors = get_authors(&dir, 3);
    let current_commit_info = get_current_commit_info(&dir)?;
    let config = get_configuration(&dir)?;
    let version = get_version(&dir)?;
    let commits = get_commits(&dir)?;
    let repo_size = get_packed_size(&dir)?;
    let last_change = get_last_change(&dir)?;
    let creation_date = get_creation_time().unwrap();

    let info = Info {
        project_name: config.repository_name,
        current_commit: current_commit_info,
        version,
        creation_date: creation_date,
        dominant_language,
        languages: languages_stat_vec,
        authors,
        last_change,
        repo: config.repository_url,
        commits,
        repo_size,
        number_of_lines: get_total_loc(&tokei_langs),
        license: project_license(&dir)?,
        custom_logo,
    };

    println!("{}", info);
    Ok(())
}

fn project_languages(dir: &str) -> tokei::Languages {
    use tokei::Config;

    let mut languages = tokei::Languages::new();
    let required_languages = get_all_language_types();
    let tokei_config = Config {
        types: Some(required_languages),
        ..Config::default()
    };
    languages.get_statistics(&[&dir], &[".git", "target"], &tokei_config);
    languages
}
