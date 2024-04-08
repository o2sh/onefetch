use anyhow::{Context, Result};
use language::{Language, LanguageType};
use std::collections::HashMap;
use std::path::Path;
use strum::IntoEnumIterator;

pub mod language;

pub fn get_main_language(loc_by_language: &[(Language, usize)]) -> Language {
    loc_by_language[0].0
}

/// Returns a vector of tuples containing all the languages detected inside the repository.
/// Each tuple is composed of the language and its corresponding loc (lines of code).
/// The vector is sorted by loc in descending order.
pub fn get_loc_by_language_sorted(
    dir: &Path,
    globs_to_exclude: &[String],
    language_types: &[LanguageType],
    include_hidden: bool,
) -> Result<Vec<(Language, usize)>> {
    let stats = get_statistics(dir, globs_to_exclude, language_types, include_hidden);

    let loc_by_language =
        get_loc_by_language(&stats).context("Could not find any source code in this repository")?;

    let loc_by_language_sorted = sort_by_loc(loc_by_language);

    Ok(loc_by_language_sorted)
}

fn sort_by_loc(map: HashMap<Language, usize>) -> Vec<(Language, usize)> {
    let mut vec: Vec<(Language, usize)> = map.into_iter().collect();
    vec.sort_by(|a, b| b.1.cmp(&a.1));
    vec
}

fn get_loc_by_language<Any>(languages: Any) -> Option<HashMap<Language, usize>> {
    todo!("Determine if this is necessary")
    // let mut loc_by_language = HashMap::new();

    // for (language_name, language) in languages.iter() {
    //     let loc = language::loc(language_name, language);

    //     if loc == 0 {
    //         continue;
    //     }

    //     loc_by_language.insert(Language::from(*language_name), loc);
    // }

    // let total_loc: usize = loc_by_language.values().sum();
    // if total_loc == 0 {
    //     None
    // } else {
    //     Some(loc_by_language)
    // }
}

pub fn get_total_loc(loc_by_language: &[(Language, usize)]) -> usize {
    let total_loc: usize = loc_by_language.iter().map(|(_, v)| v).sum();
    total_loc
}

fn get_statistics(
    dir: &Path,
    globs_to_exclude: &[String],
    language_types: &[LanguageType],
    include_hidden: bool,
) -> ! {
    // TODO Determine best way to ignore files (and if that should continue to be handled by onefetch)
    todo!("Get statistics")
}

fn filter_languages_on_type(types: &[LanguageType]) -> ! {
    todo!("Determine if this is even necessary")
}
