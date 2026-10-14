//! The colors of the ascii logo and of the info lines, and whether they are
//! bold.

use super::RenderOptions;
use super::text::Style;
use crate::language::Language;
use owo_colors::{AnsiColors, DynColors, Style as AnsiStyle};

const DEFAULT_COLOR: DynColors = DynColors::Ansi(AnsiColors::Default);

/// The 8 standard terminal colors.
pub const COLOR_PALETTE: [DynColors; 8] = [
    DynColors::Ansi(AnsiColors::Black),
    DynColors::Ansi(AnsiColors::Red),
    DynColors::Ansi(AnsiColors::Green),
    DynColors::Ansi(AnsiColors::Yellow),
    DynColors::Ansi(AnsiColors::Blue),
    DynColors::Ansi(AnsiColors::Magenta),
    DynColors::Ansi(AnsiColors::Cyan),
    DynColors::Ansi(AnsiColors::White),
];

pub struct Theme {
    /// The colors of the ascii logo, `{0}` to `{n}`.
    pub logo: Vec<DynColors>,
    title: DynColors,
    tilde: DynColors,
    underline: DynColors,
    key: DynColors,
    separator: DynColors,
    value: DynColors,
    bold: bool,
}

impl Theme {
    /// The logo takes the colors of `--ascii-language`, or else of the dominant
    /// language, overridden by `--ascii-colors`. The title and the keys of the
    /// info lines take the logo's first color, unless `--text-colors` says
    /// otherwise.
    pub fn new(options: &RenderOptions, dominant_language: Option<Language>) -> Self {
        let logo = logo_colors(
            dominant_language.as_ref(),
            options.ascii_language.as_ref(),
            &options.ascii_colors,
            options.true_color,
        );
        Self::with_text_colors(logo, &options.text_colors, !options.no_bold)
    }

    pub(crate) fn with_text_colors(logo: Vec<DynColors>, text_colors: &[u8], bold: bool) -> Self {
        let primary_color = logo.first().copied().unwrap_or(DEFAULT_COLOR);
        let text_color = |i: usize| text_colors.get(i).copied().map(ansi_color);
        Self {
            title: text_color(0).unwrap_or(primary_color),
            tilde: text_color(1).unwrap_or(DEFAULT_COLOR),
            underline: text_color(2).unwrap_or(DEFAULT_COLOR),
            key: text_color(3).unwrap_or(primary_color),
            separator: text_color(4).unwrap_or(DEFAULT_COLOR),
            value: text_color(5).unwrap_or(DEFAULT_COLOR),
            logo,
            bold,
        }
    }

    pub(crate) fn style(&self, style: Style) -> AnsiStyle {
        match style {
            Style::Plain => AnsiStyle::new(),
            Style::Value => colored(self.value, false),
            Style::Key => colored(self.key, self.bold),
            Style::Separator => colored(self.separator, self.bold),
            Style::Title => colored(self.title, self.bold),
            Style::Tilde => colored(self.tilde, self.bold),
            Style::Underline => colored(self.underline, false),
            Style::Color(color) => AnsiStyle::new().color(color),
            Style::Background(color) => AnsiStyle::new().on_color(color),
            Style::Logo(color) => colored(color, self.bold),
        }
    }
}

fn colored(color: DynColors, bold: bool) -> AnsiStyle {
    let style = AnsiStyle::new().color(color);
    if bold { style.bold() } else { style }
}

fn logo_colors(
    language_opt: Option<&Language>,
    override_language_opt: Option<&Language>,
    ascii_colors: &[u8],
    true_color: bool,
) -> Vec<DynColors> {
    let language_colors = match override_language_opt.or(language_opt) {
        Some(lang) => lang.colors(true_color),
        None => vec![DynColors::Ansi(AnsiColors::White)],
    };
    if ascii_colors.is_empty() {
        return language_colors;
    }

    let mut colors: Vec<DynColors> = ascii_colors.iter().copied().map(ansi_color).collect();

    if language_colors.len() > colors.len() {
        colors.extend(language_colors.into_iter().skip(colors.len()));
    }

    colors
}

/// The color of `--ascii-colors` and `--text-colors` values: 0 to 15.
fn ansi_color(num: u8) -> DynColors {
    let ansi_color = match num {
        0 => AnsiColors::Black,
        1 => AnsiColors::Red,
        2 => AnsiColors::Green,
        3 => AnsiColors::Yellow,
        4 => AnsiColors::Blue,
        5 => AnsiColors::Magenta,
        6 => AnsiColors::Cyan,
        7 => AnsiColors::White,
        8 => AnsiColors::BrightBlack,
        9 => AnsiColors::BrightRed,
        10 => AnsiColors::BrightGreen,
        11 => AnsiColors::BrightYellow,
        12 => AnsiColors::BrightBlue,
        13 => AnsiColors::BrightMagenta,
        14 => AnsiColors::BrightCyan,
        15 => AnsiColors::BrightWhite,
        _ => AnsiColors::Default,
    };
    DynColors::Ansi(ansi_color)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ansi_color() {
        assert_eq!(ansi_color(2), DynColors::Ansi(AnsiColors::Green));
        assert_eq!(ansi_color(u8::MAX), DynColors::Ansi(AnsiColors::Default));
    }

    #[test]
    fn logo_colors_no_language_no_custom_language_custom_colors() {
        let colors = logo_colors(None, None, &[3, 5, 8], false);
        assert_eq!(colors.len(), 3);
        assert_eq!(colors, vec![ansi_color(3), ansi_color(5), ansi_color(8)]);
    }

    #[test]
    fn logo_colors_no_language_no_custom_language() {
        let colors = logo_colors(None, None, &[], false);
        assert_eq!(colors.len(), 1);
        assert_eq!(colors, vec![DynColors::Ansi(AnsiColors::White)]);
    }

    #[test]
    fn logo_colors_no_language_with_custom_language() {
        let colors = logo_colors(None, Some(&Language::Python), &[], false);
        assert_eq!(colors.len(), 2);
        assert_eq!(
            colors,
            vec![
                DynColors::Ansi(AnsiColors::Blue),
                DynColors::Ansi(AnsiColors::Yellow)
            ]
        );
    }

    #[test]
    fn logo_colors_no_custom_language_no_custom_colors_no_true_color() {
        let colors = logo_colors(Some(&Language::Rust), None, &[], false);
        assert_eq!(colors.len(), 2);
        assert_eq!(
            colors,
            vec![
                DynColors::Ansi(AnsiColors::Red),
                DynColors::Ansi(AnsiColors::Default)
            ]
        );
    }

    #[test]
    fn logo_colors_no_custom_language_no_custom_colors_true_color() {
        let colors = logo_colors(Some(&Language::Rust), None, &[], true);
        assert_eq!(colors.len(), 2);
        assert_eq!(
            colors,
            vec![DynColors::Rgb(228, 55, 23), DynColors::Rgb(255, 255, 255)]
        );
    }

    #[test]
    fn logo_colors_custom_language_no_custom_colors_no_true_color() {
        let colors = logo_colors(Some(&Language::Rust), Some(&Language::Sh), &[], false);
        assert_eq!(colors.len(), 1);
        assert_eq!(colors, vec![DynColors::Ansi(AnsiColors::Green)]);
    }

    #[test]
    fn logo_colors_no_custom_language_custom_colors_no_true_color() {
        let colors = logo_colors(Some(&Language::Rust), None, &[2, 3], false);
        assert_eq!(colors.len(), 2);
        assert_eq!(colors, vec![ansi_color(2), ansi_color(3)]);
    }

    #[test]
    fn logo_colors_fill_custom_colors_with_language_colors() {
        // When custom ascii colors are not enough for the given language,
        // language colors should be used as default
        let colors = logo_colors(Some(&Language::Go), None, &[0], false);
        assert_eq!(colors.len(), 3);
        assert_eq!(
            colors,
            vec![
                ansi_color(0),
                DynColors::Ansi(AnsiColors::Default),
                DynColors::Ansi(AnsiColors::Yellow)
            ]
        );
    }

    #[test]
    fn logo_colors_follow_dominant_language() {
        // The default `RenderOptions` have true colors off.
        let theme = Theme::new(&RenderOptions::default(), Some(Language::Rust));
        assert_eq!(theme.logo, Language::Rust.colors(false));
    }

    #[test]
    fn no_custom_text_colors() {
        let primary_color = DynColors::Ansi(AnsiColors::Blue);
        let theme = Theme::with_text_colors(vec![primary_color], &[], true);
        assert_eq!(theme.title, primary_color);
        assert_eq!(theme.tilde, DEFAULT_COLOR);
        assert_eq!(theme.underline, DEFAULT_COLOR);
        assert_eq!(theme.key, primary_color);
        assert_eq!(theme.separator, DEFAULT_COLOR);
        assert_eq!(theme.value, DEFAULT_COLOR);
    }

    #[test]
    fn with_custom_text_colors() {
        let custom_colors = vec![0, 1, 2, 3, 4, 5];
        let theme = Theme::with_text_colors(
            vec![DynColors::Ansi(AnsiColors::Blue)],
            &custom_colors,
            true,
        );
        assert_eq!(theme.title, ansi_color(custom_colors[0]));
        assert_eq!(theme.tilde, ansi_color(custom_colors[1]));
        assert_eq!(theme.underline, ansi_color(custom_colors[2]));
        assert_eq!(theme.key, ansi_color(custom_colors[3]));
        assert_eq!(theme.separator, ansi_color(custom_colors[4]));
        assert_eq!(theme.value, ansi_color(custom_colors[5]));
    }

    #[test]
    fn with_some_custom_text_colors() {
        let custom_colors = vec![0, 1, 2];
        let primary_color = DynColors::Ansi(AnsiColors::Blue);
        let theme = Theme::with_text_colors(vec![primary_color], &custom_colors, true);
        assert_eq!(theme.title, ansi_color(custom_colors[0]));
        assert_eq!(theme.tilde, ansi_color(custom_colors[1]));
        assert_eq!(theme.underline, ansi_color(custom_colors[2]));
        assert_eq!(theme.key, primary_color);
        assert_eq!(theme.separator, DEFAULT_COLOR);
        assert_eq!(theme.value, DEFAULT_COLOR);
    }
}
