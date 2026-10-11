use self::format::NumberSeparator;
use self::image::ImageProtocol;
use crate::language::Language;
use std::path::PathBuf;

mod colors;
mod fields;
pub mod format;
pub mod image;
pub mod lines;
pub mod logo;
mod printer;
pub mod style;

pub use printer::Printer;

pub const DEFAULT_COLOR_RESOLUTION: usize = 64;

#[derive(Clone, clap::ValueEnum, PartialEq, Eq, Debug)]
pub enum SerializationFormat {
    Json,
    Yaml,
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
