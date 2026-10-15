//! The options of each stage: what to collect from the repository
//! (`InfoOptions`) and how to print it (`RenderOptions`), and how the parsed
//! command line fills them in. Neither `data` nor `render` depends on `cli`.

use crate::cli::{Cli, When};
use crate::data::FieldKind;
use crate::data::git::bots::BotRegex;
use crate::language::{Language, LanguageType};
use crate::render::SerializationFormat;
use crate::render::format::NumberSeparator;
use crate::render::image::ImageProtocol;
use std::env;
use std::path::PathBuf;

pub const DEFAULT_NUMBER_OF_AUTHORS: usize = 3;
pub const DEFAULT_NUMBER_OF_LANGUAGES: usize = 6;
pub const DEFAULT_NUMBER_OF_FILE_CHURNS: usize = 3;
pub const DEFAULT_COLOR_RESOLUTION: usize = 64;

/// What to collect from the repository and which fields to show.
#[derive(Clone, Debug)]
pub struct InfoOptions {
    pub input: PathBuf,
    pub disabled_fields: Vec<FieldKind>,
    pub no_title: bool,
    pub number_of_authors: usize,
    pub number_of_languages: usize,
    pub number_of_file_churns: usize,
    pub churn_pool_size: Option<usize>,
    pub exclude: Vec<String>,
    pub no_bots: Option<BotRegex>,
    pub no_merges: bool,
    pub email: bool,
    pub http_url: bool,
    pub hide_token: bool,
    pub include_hidden: bool,
    pub language_types: Vec<LanguageType>,
}

/// How to print an `Info`: output format, art, colors, and how values look in
/// the info lines.
#[derive(Clone, Debug, PartialEq)]
pub struct RenderOptions {
    pub output: Option<SerializationFormat>,
    pub no_art: bool,
    pub image: Option<PathBuf>,
    pub image_protocol: Option<ImageProtocol>,
    pub color_resolution: usize,
    pub ascii_input: Option<String>,
    pub ascii_language: Option<Language>,
    pub ascii_colors: Vec<u8>,
    pub text_colors: Vec<u8>,
    pub no_bold: bool,
    pub number_separator: NumberSeparator,
    pub iso_time: bool,
    pub true_color: bool,
    pub nerd_fonts: bool,
    pub no_color_palette: bool,
}

/// The CLI defaults, except that true colors are off since `--true-color auto`
/// depends on the terminal.
impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            output: None,
            no_art: false,
            image: None,
            image_protocol: None,
            color_resolution: DEFAULT_COLOR_RESOLUTION,
            ascii_input: None,
            ascii_language: None,
            ascii_colors: Vec::new(),
            text_colors: Vec::new(),
            no_bold: false,
            number_separator: NumberSeparator::default(),
            iso_time: false,
            true_color: false,
            nerd_fonts: false,
            no_color_palette: false,
        }
    }
}

impl From<&Cli> for InfoOptions {
    fn from(cli: &Cli) -> Self {
        Self {
            input: cli.input.clone(),
            disabled_fields: cli.info.disabled_fields.clone(),
            no_title: cli.info.no_title,
            number_of_authors: cli.info.number_of_authors,
            number_of_languages: cli.info.number_of_languages,
            number_of_file_churns: cli.info.number_of_file_churns,
            churn_pool_size: cli.info.churn_pool_size,
            exclude: cli.info.exclude.clone(),
            no_bots: cli.info.no_bots.clone(),
            no_merges: cli.info.no_merges,
            email: cli.info.email,
            http_url: cli.info.http_url,
            hide_token: cli.info.hide_token,
            include_hidden: cli.info.include_hidden,
            language_types: cli.info.language_types.clone(),
        }
    }
}

impl From<&Cli> for RenderOptions {
    fn from(cli: &Cli) -> Self {
        Self {
            output: cli.developer.output.clone(),
            no_art: cli.visuals.no_art,
            image: cli.image.image.clone(),
            image_protocol: cli.image.image_protocol.clone(),
            color_resolution: cli.image.color_resolution,
            ascii_input: cli.ascii.ascii_input.clone(),
            ascii_language: cli.ascii.ascii_language,
            ascii_colors: cli.ascii.ascii_colors.clone(),
            text_colors: cli.text_formatting.text_colors.clone(),
            no_bold: cli.text_formatting.no_bold,
            number_separator: cli.text_formatting.number_separator,
            iso_time: cli.text_formatting.iso_time,
            true_color: true_color(&cli.ascii.true_color),
            nerd_fonts: cli.visuals.nerd_fonts,
            no_color_palette: cli.visuals.no_color_palette,
        }
    }
}

fn true_color(when: &When) -> bool {
    match when {
        When::Always => true,
        When::Never => false,
        When::Auto => is_truecolor_terminal(),
    }
}

fn is_truecolor_terminal() -> bool {
    env::var("COLORTERM")
        .map(|colorterm| colorterm == "truecolor" || colorterm == "24bit")
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_render_options_match_cli_defaults() {
        let mut cli = Cli::default();
        cli.ascii.true_color = When::Never;
        assert_eq!(RenderOptions::from(&cli), RenderOptions::default());
    }
}
