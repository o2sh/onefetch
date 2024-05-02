use anyhow::{anyhow, Context, Result};
use gengo::{analysis, FileSource, Git, Builder};
use gix::filter::plumbing::eol::Stats;
use language::{Language, LanguageType};
use std::collections::HashMap;
use std::error::Error;
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
    let analysis = match get_statistics(dir, globs_to_exclude, language_types, include_hidden) {
        Ok(stats) => stats,
        Err(e) => return Err(anyhow!("Could not analyze repository: {}", e))
    };

    // NOTE If finer control is ever needed, summary_with can be used.
    let mut size_by_language: Vec<(Language, _)> = analysis.iter()
        .filter(|(_, entry)| {
            let lang_type = LanguageType(entry.language().category());
            language_types.contains(&lang_type)
        })
        .filter_map(|(_, entry)| {
            let language = *entry.language();
            let language: Option<Language> = language.try_into().ok();
            language.map(|language: Language| (language, entry.size()))
        })
        .fold(HashMap::new(), |mut acc, (language, size)| {
            *acc.entry(language).or_insert(0) += size;
            acc
        })
        .into_iter()
        .collect();
    // NOTE Sort by size (descending) first, then by language name (ascending) in case the size is equal
    size_by_language.sort_by_key(|(language, size)| (usize::MAX - *size, language.to_string()));

    Ok(size_by_language)
}

pub fn get_total_size(loc_by_language: &[(Language, usize)]) -> usize {
    let total_loc: usize = loc_by_language.iter().map(|(_, v)| v).sum();
    total_loc
}

fn get_statistics(
    dir: &Path,
    globs_to_exclude: &[String],
    language_types: &[LanguageType],
    include_hidden: bool,
) -> Result<gengo::Analysis, Box<dyn Error>> {
    // TODO Determine best way to ignore files (and if that should continue to be handled by onefetch)
    let file_source = Git::new(dir, "HEAD")?;
    let gengo = Builder::new(file_source).build()?;
    gengo.analyze()
}
