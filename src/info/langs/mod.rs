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

fn get_loc_by_language(languages: &tokei::Languages) -> Option<HashMap<Language, usize>> {
    let mut loc_by_language = HashMap::new();

    for (language_name, language) in languages.iter() {
        let loc = language::loc(language_name, language);

        if loc == 0 {
            continue;
        }

        loc_by_language.insert(Language::from(*language_name), loc);
    }

    let total_loc: usize = loc_by_language.values().sum();
    if total_loc == 0 {
        None
    } else {
        Some(loc_by_language)
    }
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
) -> tokei::Languages {
    let mut languages = tokei::Languages::new();
    let filtered_languages = filter_languages_on_type(language_types);

    let tokei_config = tokei::Config {
        types: Some(filtered_languages),
        hidden: Some(include_hidden),
        ..tokei::Config::default()
    };
    let ignored: Vec<&str> = globs_to_exclude.iter().map(AsRef::as_ref).collect();
    languages.get_statistics(&[&dir], &ignored, &tokei_config);
    languages
}

fn filter_languages_on_type(types: &[LanguageType]) -> Vec<tokei::LanguageType> {
    Language::iter()
        .filter(|language| types.contains(&language.get_type()))
        .map(std::convert::Into::into)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use tokei;

    #[test]
    fn get_loc_by_language_counts_md_comments() {
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

        let loc_by_language = get_loc_by_language(&languages).unwrap();

        // NOTE: JS  with 100 lines of code, MD with 300 lines of code + comments
        assert_eq!(loc_by_language[&Language::JavaScript], 100);
        assert_eq!(loc_by_language[&Language::Markdown], 300);
    }

    #[test]
    fn deeply_nested_total_loc() {
        let mut bash_code_stats = tokei::CodeStats::new();
        // NOTE: When inside Markdown, comments should be counted as code
        bash_code_stats.code = 5;
        bash_code_stats.blanks = 1;
        bash_code_stats.comments = 2;

        let mut md_code_stats = tokei::CodeStats::new();
        md_code_stats.code = 10;
        md_code_stats.blanks = 2;
        md_code_stats.comments = 4;
        md_code_stats
            .blobs
            .insert(tokei::LanguageType::Bash, bash_code_stats);
        // NOTE: This may break if tokei ever does more than just assign `name` to a field
        let mut md_report = tokei::Report::new("/tmp/file.ipynb".into());
        md_report.stats = md_code_stats;

        let mut jupyter_notebook = tokei::Language::default();
        jupyter_notebook
            .children
            .insert(tokei::LanguageType::Markdown, vec![md_report]);

        let mut languages = tokei::Languages::new();
        languages.insert(tokei::LanguageType::Jupyter, jupyter_notebook);

        let loc_by_language = get_loc_by_language(&languages).unwrap();

        assert_eq!(loc_by_language[&Language::Jupyter], 21);
    }

    // https://github.com/o2sh/onefetch/issues/966
    #[test]
    fn get_loc_by_language_should_not_panic_when_children_language_is_not_supported() {
        let mut stylus_code_stats = tokei::CodeStats::new();
        stylus_code_stats.code = 10;
        stylus_code_stats.blanks = 2;
        stylus_code_stats.comments = 4;

        let mut stylus_report = tokei::Report::new("/tmp/file.vue".into());
        stylus_report.stats = stylus_code_stats;

        let mut vue = tokei::Language {
            blanks: 50,
            comments: 200,
            code: 100,
            ..Default::default()
        };

        vue.children
            .insert(tokei::LanguageType::Stylus, vec![stylus_report]);

        let mut languages = tokei::Languages::new();
        languages.insert(tokei::LanguageType::Vue, vue);

        let loc_by_language = get_loc_by_language(&languages).unwrap();

        assert_eq!(loc_by_language[&Language::Vue], 110);
    }

    #[test]
    fn test_get_loc_by_language_sorted() {
        let mut map = HashMap::new();
        map.insert(Language::Ada, 300);
        map.insert(Language::Java, 40);
        map.insert(Language::Rust, 1200);
        map.insert(Language::Go, 8);

        let sorted_map = sort_by_loc(map);

        let expected_order = vec![
            (Language::Rust, 1200),
            (Language::Ada, 300),
            (Language::Java, 40),
            (Language::Go, 8),
        ];
        let actual_order: Vec<_> = sorted_map.into_iter().collect();

        assert_eq!(expected_order, actual_order);
    }

    #[test]
    fn test_get_total_loc() {
        let loc_by_language = [(Language::JavaScript, 100), (Language::Markdown, 300)];
        assert_eq!(get_total_loc(&loc_by_language), 400);
    }
}
