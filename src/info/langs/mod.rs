use anyhow::{Context, Result};
use language::{Language, LanguageType};
use regex::Regex;
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub mod language;

pub fn get_dominant_language(languages_stat_vec: &[(Language, f64)]) -> Language {
    languages_stat_vec[0].0.clone()
}

pub fn get_language_statistics(
    dir: &str,
    ignored_directories: &[String],
    language_types: &[LanguageType],
    include_hidden: bool,
) -> Result<(Vec<(Language, f64)>, usize)> {
    let stats = get_statistics(dir, ignored_directories, language_types, include_hidden);
    let language_distribution = get_language_distribution(&stats)
        .with_context(|| "Could not find any source code in this repository")?;
    let mut language_distribution_vec: Vec<(_, _)> = language_distribution.into_iter().collect();
    language_distribution_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());
    let loc = get_total_loc(&stats);
    Ok((language_distribution_vec, loc))
}

fn get_language_distribution(languages: &tokei::Languages) -> Option<HashMap<Language, f64>> {
    let mut language_distribution = HashMap::new();

    for (language_name, language) in languages.iter() {
        let mut code = language.code;

        let has_children = !language.children.is_empty();

        if has_children {
            for reports in language.children.values() {
                for stats in reports.iter().map(|r| r.stats.summarise()) {
                    code += stats.code;
                }
            }
        }

        if code == 0 {
            continue;
        }

        language_distribution.insert(Language::from(*language_name), code as f64);
    }

    let total: f64 = language_distribution.iter().map(|(_, v)| v).sum();

    if total.abs() < f64::EPSILON {
        None
    } else {
        for (_, val) in language_distribution.iter_mut() {
            *val /= total;
            *val *= 100_f64;
        }

        Some(language_distribution)
    }
}

fn get_total_loc(languages: &tokei::Languages) -> usize {
    languages
        .values()
        .collect::<Vec<&tokei::Language>>()
        .iter()
        .fold(0, |sum, val| sum + val.code)
}

fn get_statistics(
    dir: &str,
    ignored_directories: &[String],
    language_types: &[LanguageType],
    include_hidden: bool,
) -> tokei::Languages {
    let mut languages = tokei::Languages::new();
    let supported_languages = get_supported_languages(language_types);

    let tokei_config = tokei::Config {
        types: Some(supported_languages),
        hidden: Some(include_hidden),
        ..tokei::Config::default()
    };
    let user_ignored = get_ignored_directories(ignored_directories);
    let ignored: Vec<&str> = user_ignored.iter().map(AsRef::as_ref).collect();
    languages.get_statistics(&[&dir], &ignored, &tokei_config);
    languages
}

fn get_supported_languages(types: &[LanguageType]) -> Vec<tokei::LanguageType> {
    Language::iter()
        .filter(|language| types.contains(&language.get_type()))
        .map(|language| language.into())
        .collect()
}

fn get_ignored_directories(user_ignored_directories: &[String]) -> Vec<String> {
    let mut ignored_directories = Vec::new();
    if !user_ignored_directories.is_empty() {
        let re = Regex::new(r"((.*)+/)+(.*)").unwrap();
        for user_ignored_directory in user_ignored_directories {
            if re.is_match(user_ignored_directory) {
                let prefix = if user_ignored_directory.starts_with('/') {
                    "**"
                } else {
                    "**/"
                };
                ignored_directories.push(format!("{}{}", prefix, user_ignored_directory));
            } else {
                ignored_directories.push(String::from(user_ignored_directory));
            }
        }
    }
    ignored_directories
}
