use self::format::NumberSeparator;
use self::image::ImageProtocol;
use crate::data::FieldKind;
use crate::language::Language;
use std::path::PathBuf;

pub mod ascii;
mod colors;
mod fields;
pub mod format;
pub mod image;
pub mod info;
mod printer;
pub mod style;

pub use printer::Printer;

pub const DEFAULT_COLOR_RESOLUTION: usize = 64;

/// A slot in `ascii_colors` or `text_colors` that keeps its default color.
/// The config file can set e.g. the key color alone; the command line cannot
/// produce this value since it only accepts colors 0 to 15.
pub const UNSET_COLOR: u8 = u8::MAX;

pub const DEFAULT_SEPARATOR: &str = ": ";

/// How the key of each info line is laid out. Only the config file sets it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyLayout {
    /// Printed between a key and its value.
    pub separator: String,
    /// Pads the left padding, key and separator to this many columns, so that
    /// the values line up. 0 adds no padding.
    pub width: usize,
    /// Spaces before each key.
    pub padding_left: usize,
    /// The key and colors set for single fields.
    pub fields: Vec<(FieldKind, FieldStyle)>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FieldStyle {
    /// Replaces the field's key.
    pub key: Option<String>,
    pub key_color: Option<u8>,
    pub output_color: Option<u8>,
}

impl KeyLayout {
    pub fn field_style(&self, kind: FieldKind) -> Option<&FieldStyle> {
        self.fields
            .iter()
            .find(|(field, _)| *field == kind)
            .map(|(_, style)| style)
    }
}

impl Default for KeyLayout {
    fn default() -> Self {
        Self {
            separator: DEFAULT_SEPARATOR.into(),
            width: 0,
            padding_left: 0,
            fields: Vec::new(),
        }
    }
}

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
    pub key_layout: KeyLayout,
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
            key_layout: KeyLayout::default(),
        }
    }
}
