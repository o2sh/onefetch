use serde::Serialize;
use tokei;

include!(concat!(env!("OUT_DIR"), "/language.rs"));

pub fn loc(language_type: &tokei::LanguageType, language: &tokei::Language) -> usize {
    __loc(language_type, language.code, language.comments)
        + language
            .children
            .iter()
            .fold(0, |sum, (lang_type, reports)| {
                sum + reports.iter().fold(0, |sum, report| {
                    let stats = report.stats.summarise();
                    sum + __loc(lang_type, stats.code, stats.comments)
                })
            })
}

fn __loc(language_type: &tokei::LanguageType, code: usize, comments: usize) -> usize {
    match language_type {
        tokei::LanguageType::Markdown => code + comments,
        _ => code,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Language::Go, true, '\u{e627}')]
    #[case(Language::Abap, true, DEFAULT_CHIP_ICON)] // No Nerd Font icon for this language
    #[case(Language::Rust, false, DEFAULT_CHIP_ICON)]
    fn test_language_get_chip_icon(
        #[case] language: Language,
        #[case] use_nerd_fonts: bool,
        #[case] expected_chip_icon: char,
    ) {
        let result = language.get_chip_icon(use_nerd_fonts);
        assert_eq!(result, expected_chip_icon);
    }
}
