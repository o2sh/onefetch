use anyhow::{anyhow, Result};
use gengo::{Builder, Git};

use globset::{Glob, GlobSetBuilder};
use language::{Language, LanguageType};
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

pub mod language;

pub fn get_main_language(size_by_language: &[(Language, usize)]) -> Language {
    size_by_language[0].0
}

/// Returns a vector of tuples containing all the languages detected inside the repository.
/// Each tuple is composed of the language and its corresponding blob size.
/// The vector is sorted by size in descending order.
pub fn get_size_by_language_sorted(
    dir: &Path,
    globs_to_exclude: &[String],
    language_types: &[LanguageType],
    include_hidden: bool,
) -> Result<Vec<(Language, usize)>> {
    let globset = globs_to_exclude
        .iter()
        .filter_map(|glob| Glob::new(glob).ok())
        .fold(GlobSetBuilder::new(), |mut builder, glob| {
            builder.add(glob);
            builder
        })
        .build()?;
    let analysis = match get_statistics(dir) {
        Ok(stats) => stats,
        Err(e) => return Err(anyhow!("Could not analyze repository: {}", e)),
    };

    // NOTE If finer control is ever needed, summary_with can be used.
    let mut size_by_language: Vec<(Language, _)> = analysis
        .iter()
        .filter(|(path, _)| include_hidden || !is_hidden(path))
        .filter(|(path, _)| !globset.is_match(path))
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

fn get_statistics(dir: &Path) -> Result<gengo::Analysis, Box<dyn Error>> {
    // TODO Determine best way to ignore files (and if that should continue to be handled by onefetch)
    let file_source = Git::new(dir, "HEAD")?;
    let gengo = Builder::new(file_source).build()?;
    gengo.analyze()
}

/// Returns `true` if the file is or any of its containing directories are hidden.
fn is_hidden(path: impl AsRef<Path>) -> bool {
    path.as_ref()
        .components()
        .any(|c| c.as_os_str().to_string_lossy().starts_with('.'))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case("file", false)]
    #[case("path/to/file", false)]
    #[case(".file", true)]
    #[case(".path/to/file", true)]
    #[case("path/.to/file", true)]
    fn test_is_hidden(#[case] path: &str, #[case] expected: bool) {
        assert_eq!(super::is_hidden(path), expected);
    }
}
