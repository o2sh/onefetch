use anyhow::{Context, Result};
use language::{Language, LanguageType};
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use strum::IntoEnumIterator;

pub mod language;

pub fn get_dominant_language(languages_stat_vec: &[(Language, f64)]) -> Language {
    languages_stat_vec[0].0
}

pub fn get_language_statistics(
    dir: &Path,
    ignored_directories: &[PathBuf],
    language_types: &[LanguageType],
    include_hidden: bool,
) -> Result<(Vec<(Language, f64)>, usize)> {
    let stats = get_statistics(dir, ignored_directories, language_types, include_hidden);
    let language_distribution = get_language_distribution(&stats)
        .context("Could not find any source code in this repository")?;
    let mut language_distribution_vec: Vec<(_, _)> = language_distribution.into_iter().collect();
    language_distribution_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());
    let loc = get_total_loc(&stats);
    Ok((language_distribution_vec, loc))
}

fn get_language_distribution(languages: &tokei::Languages) -> Option<HashMap<Language, f64>> {
    let mut language_distribution = HashMap::new();

    for (language_name, language) in languages.iter() {
        let loc = language::loc(language_name, language);

        if loc == 0 {
            continue;
        }

        language_distribution.insert(Language::from(*language_name), loc as f64);
    }

    let total: f64 = language_distribution.values().sum();

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
    languages.iter().fold(0, |sum, (lang_type, lang)| {
        sum + language::loc(lang_type, lang)
    })
}

fn get_statistics(
    dir: &Path,
    ignored_directories: &[PathBuf],
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

fn get_ignored_directories(user_ignored_directories: &[PathBuf]) -> Vec<String> {
    let mut ignored_directories = Vec::new();
    if !user_ignored_directories.is_empty() {
        let re = Regex::new(r"((.*)+/)+(.*)").unwrap();
        for user_ignored_directory in user_ignored_directories {
            let dir = user_ignored_directory.display().to_string();
            if re.is_match(&dir) {
                let prefix = if dir.starts_with('/') { "**" } else { "**/" };
                ignored_directories.push(format!("{}{}", prefix, dir));
            } else {
                ignored_directories.push(dir);
            }
        }
    }
    ignored_directories
}

#[cfg(test)]
mod test {
    use super::*;
    use tokei;

    #[test]
    fn get_language_distribution_counts_md_comments() {
        let js = tokei::Language {
            blanks: 25,
            comments: 50,
            code: 100,
            ..Default::default()
        };
        let js_type = tokei::LanguageType::JavaScript;

        let md = tokei::Language {
            blanks: 50,
            comments: 200,
            code: 100,
            ..Default::default()
        };
        let md_type = tokei::LanguageType::Markdown;

        let mut languages = tokei::Languages::new();
        languages.insert(js_type, js);
        languages.insert(md_type, md);

        let language_distribution = get_language_distribution(&languages).unwrap();

        // NOTE: JS is 25% with 100 lines of code, MD is 75% with 300 lines of code + comments
        assert_eq!(language_distribution[&Language::JavaScript], 25_f64);
        assert_eq!(language_distribution[&Language::Markdown], 75_f64);
    }

    #[test]
    fn get_total_loc_counts_md_comments() {
        let js = tokei::Language {
            blanks: 25,
            comments: 50,
            code: 100,
            ..Default::default()
        };
        let js_type = tokei::LanguageType::JavaScript;

        let md = tokei::Language {
            blanks: 50,
            comments: 200,
            code: 100,
            ..Default::default()
        };
        let md_type = tokei::LanguageType::Markdown;

        let mut languages = tokei::Languages::new();
        languages.insert(js_type, js);
        languages.insert(md_type, md);

        assert_eq!(get_total_loc(&languages), 400);
    }
}
