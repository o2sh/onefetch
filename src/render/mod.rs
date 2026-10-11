use self::ascii::AsciiArt;
use self::colors::{TextColors, get_ascii_colors};
use self::format::DisplayOptions;
use self::image::{ImageBackend, ImageProtocol};
use crate::info::Info;
use crate::language::Language;
use ::image::DynamicImage;
use anyhow::{Context, Result};
use owo_colors::{AnsiColors, DynColors};
use std::fmt::Write as _;
use std::path::PathBuf;

mod ansi;
pub mod ascii;
mod colors;
mod fields;
pub mod format;
pub mod image;
pub mod layout;
pub mod text;

const CENTER_PAD_LENGTH: usize = 3;

pub const DEFAULT_COLOR_RESOLUTION: usize = 64;

#[derive(Clone, clap::ValueEnum, PartialEq, Eq, Debug)]
pub enum SerializationFormat {
    Json,
    Yaml,
}

/// How to print an `Info`: output format, art, and text styling.
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
    pub display: DisplayOptions,
}

/// The CLI defaults, except that true colors are off (see `DisplayOptions`).
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
            display: DisplayOptions::default(),
        }
    }
}

pub struct Printer {
    info: Info,
    kind: PrinterKind,
    no_bold: bool,
    text_colors: TextColors,
    ascii_colors: Vec<DynColors>,
    display_options: DisplayOptions,
}

enum PrinterKind {
    Plain,
    Json,
    Yaml,
    Ascii {
        art: String,
    },
    Image {
        image: DynamicImage,
        backend: Box<dyn ImageBackend>,
        resolution: usize,
    },
}

impl Printer {
    pub fn new(info: Info, options: &RenderOptions) -> Result<Self> {
        let image =
            match &options.image {
                Some(p) => Some(::image::open(p).with_context(|| {
                    format!("Could not load the image file at '{}'", p.display())
                })?),
                None => None,
            };

        let image_backend = if image.is_some() {
            options
                .image_protocol
                .clone()
                .map_or_else(image::get_best_backend, |s| Ok(image::get_image_backend(s)))?
        } else {
            None
        };

        let ascii_colors = get_ascii_colors(
            info.dominant_language.as_ref(),
            options.ascii_language.as_ref(),
            &options.ascii_colors,
            options.display.true_color,
        );
        let primary_color = ascii_colors
            .first()
            .copied()
            .unwrap_or(DynColors::Ansi(AnsiColors::Default));
        let text_colors = TextColors::new(&options.text_colors, primary_color);

        let kind = PrinterKind::new(options, info.dominant_language, image, image_backend)?;

        Ok(Self {
            info,
            kind,
            no_bold: options.no_bold,
            text_colors,
            ascii_colors,
            display_options: options.display,
        })
    }

    pub fn print(&self, writer: &mut dyn std::io::Write) -> Result<()> {
        match &self.kind {
            PrinterKind::Json => {
                write!(writer, "{}", serde_json::to_string_pretty(&self.info)?)?;
                Ok(())
            }
            PrinterKind::Yaml => {
                write!(writer, "{}", serde_yaml::to_string(&self.info)?)?;
                Ok(())
            }
            PrinterKind::Plain => {
                write_with_line_wrapping(writer, &self.info_text())?;
                Ok(())
            }
            PrinterKind::Image {
                image,
                backend,
                resolution,
            } => {
                let center_pad = " ".repeat(CENTER_PAD_LENGTH);
                let info_text = self.info_text();
                let info_lines = info_text
                    .lines()
                    .map(|s| format!("{center_pad}{s}"))
                    .collect();

                let rendered = backend
                    .add_image(info_lines, image, *resolution)
                    .context("Failed to render image")?;

                write_with_line_wrapping(writer, &rendered)?;
                Ok(())
            }
            PrinterKind::Ascii { art } => {
                let mut buf = String::new();
                let center_pad = " ".repeat(CENTER_PAD_LENGTH);
                let info_text = self.info_text();
                let mut info_lines = info_text.lines();
                let mut logo_lines = AsciiArt::new(art, &self.ascii_colors, !self.no_bold);

                loop {
                    match (logo_lines.next(), info_lines.next()) {
                        (Some(logo), Some(info)) => writeln!(buf, "{logo}{center_pad}{info:^}")?,
                        (Some(logo), None) => writeln!(buf, "{logo}")?,
                        (None, Some(info)) => writeln!(
                            buf,
                            "{:<width$}{center_pad}{info:^}",
                            "",
                            width = logo_lines.width()
                        )?,
                        (None, None) => break,
                    }
                }

                write_with_line_wrapping(writer, &buf)?;
                Ok(())
            }
        }
    }

    fn info_text(&self) -> String {
        ansi::render(
            &layout::info_lines(&self.info, &self.display_options),
            &self.text_colors,
            self.no_bold,
        )
    }
}

impl PrinterKind {
    fn new(
        options: &RenderOptions,
        dominant_language: Option<Language>,
        image: Option<DynamicImage>,
        image_backend: Option<Box<dyn ImageBackend>>,
    ) -> Result<Self> {
        let kind = match options.output {
            Some(SerializationFormat::Json) => Self::Json,
            Some(SerializationFormat::Yaml) => Self::Yaml,
            None if options.no_art => Self::Plain,
            None => {
                if let Some(image) = image {
                    Self::Image {
                        image,
                        backend: image_backend.context("No supported image backend")?,
                        resolution: options.color_resolution,
                    }
                } else {
                    let ascii_art = options
                        .ascii_input
                        .clone()
                        .or_else(|| {
                            options
                                .ascii_language
                                .map(|language| language.ascii_art().to_string())
                        })
                        .or_else(|| {
                            dominant_language.map(|language| language.ascii_art().to_string())
                        });

                    match ascii_art {
                        Some(art) => Self::Ascii { art },
                        None => Self::Plain,
                    }
                }
            }
        };
        Ok(kind)
    }
}

fn write_with_line_wrapping(writer: &mut dyn std::io::Write, content: &str) -> Result<()> {
    // \x1B[?7l turns off line wrapping and \x1B[?7h turns it on
    write!(writer, "\x1B[?7l{content}\x1B[?7h")?;
    Ok(())
}

impl PartialEq for PrinterKind {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (PrinterKind::Plain, PrinterKind::Plain)
                | (PrinterKind::Json, PrinterKind::Json)
                | (PrinterKind::Yaml, PrinterKind::Yaml)
                | (PrinterKind::Ascii { .. }, PrinterKind::Ascii { .. })
                | (PrinterKind::Image { .. }, PrinterKind::Image { .. })
        )
    }
}

impl std::fmt::Debug for PrinterKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            PrinterKind::Plain => "Plain",
            PrinterKind::Json => "Json",
            PrinterKind::Yaml => "Yaml",
            PrinterKind::Ascii { .. } => "Ascii",
            PrinterKind::Image { .. } => "Image",
        };
        write!(f, "PrinterKind::{name}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_json_printer() {
        let options = RenderOptions {
            output: Some(SerializationFormat::Json),
            ..Default::default()
        };

        let printer = Printer::new(Info::default(), &options).unwrap();

        assert_eq!(printer.kind, PrinterKind::Json);
    }

    #[test]
    fn test_create_yaml_printer() {
        let options = RenderOptions {
            output: Some(SerializationFormat::Yaml),
            ..Default::default()
        };

        let printer = Printer::new(Info::default(), &options).unwrap();

        assert_eq!(printer.kind, PrinterKind::Yaml);
    }

    #[test]
    fn test_create_plain_printer_when_no_art() {
        let info = Info {
            dominant_language: Some(Language::Rust),
            ..Default::default()
        };
        let options = RenderOptions {
            no_art: true,
            ..Default::default()
        };

        let printer = Printer::new(info, &options).unwrap();

        assert_eq!(printer.kind, PrinterKind::Plain);
    }

    #[test]
    fn test_create_plain_printer_when_no_dominant_language_no_ascii_input() {
        let printer = Printer::new(Info::default(), &RenderOptions::default()).unwrap();

        assert_eq!(printer.kind, PrinterKind::Plain);
    }

    #[test]
    fn test_create_ascii_printer_when_dominant_language() {
        let info = Info {
            dominant_language: Some(Language::Rust),
            ..Default::default()
        };

        let printer = Printer::new(info, &RenderOptions::default()).unwrap();

        assert!(matches!(printer.kind, PrinterKind::Ascii { .. }));
    }

    #[test]
    fn test_create_ascii_printer_when_ascii_language_without_dominant_language() {
        let options = RenderOptions {
            ascii_language: Some(Language::Rust),
            ..Default::default()
        };

        let printer = Printer::new(Info::default(), &options).unwrap();

        assert!(matches!(printer.kind, PrinterKind::Ascii { .. }));
    }

    #[test]
    fn test_ascii_colors_follow_dominant_language() {
        let info = Info {
            dominant_language: Some(Language::Rust),
            ..Default::default()
        };
        // The default `RenderOptions` have true colors off.
        let printer = Printer::new(info, &RenderOptions::default()).unwrap();

        assert_eq!(
            printer.ascii_colors,
            Language::Rust.colors(false),
            "ascii colors should come from the dominant language"
        );
    }

    struct DummyBackend;

    impl ImageBackend for DummyBackend {
        fn add_image(
            &self,
            _lines: Vec<String>,
            _image: &DynamicImage,
            _colors: usize,
        ) -> anyhow::Result<String> {
            Ok("foo".to_string())
        }
    }

    #[test]
    fn test_create_image_printer() {
        let kind = PrinterKind::new(
            &RenderOptions::default(),
            None,
            Some(DynamicImage::default()),
            Some(Box::new(DummyBackend)),
        )
        .unwrap();

        assert!(matches!(kind, PrinterKind::Image { .. }));
    }
}
