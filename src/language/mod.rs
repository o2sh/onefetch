use serde::Serialize;
use tokei;

pub(crate) mod stats;

include!(concat!(env!("OUT_DIR"), "/language.rs"));

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
