//! Picks the output, puts the ascii art or image beside the info, and writes
//! it. The only code that writes escape codes, apart from the image protocols.

use super::ascii::ascii_lines;
use super::colors::Colors;
use super::image::{self, ImageBackend};
use super::info::info_lines;
use super::style::{Line, Segment};
use super::{RenderOptions, SerializationFormat};
use crate::data::Info;
use crate::language::Language;
use ::image::DynamicImage;
use anyhow::{Context, Result};
use owo_colors::OwoColorize;

const CENTER_PAD_LENGTH: usize = 3;

pub struct Printer {
    info: Info,
    kind: PrinterKind,
    options: RenderOptions,
    colors: Colors,
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

        let colors = Colors::new(options, info.main_language);

        let kind = PrinterKind::new(options, info.main_language, image, image_backend)?;

        Ok(Self {
            info,
            kind,
            options: options.clone(),
            colors,
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
                let info = info_lines(&self.info, &self.options);
                write_unwrapped(writer, &paint(&info, &self.colors))
            }
            PrinterKind::Image {
                image,
                backend,
                resolution,
            } => {
                let info: Vec<Line> = info_lines(&self.info, &self.options)
                    .into_iter()
                    .map(|line| {
                        let mut segments = vec![center_pad()];
                        segments.extend(line.0);
                        Line::from(segments)
                    })
                    .collect();
                let info = paint(&info, &self.colors)
                    .lines()
                    .map(String::from)
                    .collect();

                let rendered = backend
                    .add_image(info, image, *resolution)
                    .context("Failed to render image")?;

                write_unwrapped(writer, &rendered)
            }
            PrinterKind::Ascii { art } => {
                let info = info_lines(&self.info, &self.options);
                let lines = ascii_beside_info(ascii_lines(art), info);
                write_unwrapped(writer, &paint(&lines, &self.colors))
            }
        }
    }
}

/// The space between the ascii art or image and the info.
fn center_pad() -> Segment {
    Segment::plain(" ".repeat(CENTER_PAD_LENGTH))
}

/// Puts the ascii art on the left of the info lines.
fn ascii_beside_info(ascii: Vec<Line>, info: Vec<Line>) -> Vec<Line> {
    let width = ascii.first().map_or(0, Line::width);
    let mut ascii = ascii.into_iter();
    let mut info = info.into_iter();
    let mut lines = Vec::new();

    loop {
        let segments = match (ascii.next(), info.next()) {
            (Some(art), Some(info)) => {
                let mut segments = art.0;
                segments.push(center_pad());
                segments.extend(info.0);
                segments
            }
            (Some(art), None) => art.0,
            (None, Some(info)) => {
                let mut segments = vec![Segment::plain(" ".repeat(width)), center_pad()];
                segments.extend(info.0);
                segments
            }
            (None, None) => break,
        };
        lines.push(Line::from(segments));
    }

    lines
}

/// Writes the lines with the escape codes of their colors.
fn paint(lines: &[Line], colors: &Colors) -> String {
    let mut output = String::new();
    for line in lines {
        for segment in &line.0 {
            let style = colors.style(segment.style);
            output.push_str(&sanitize(&segment.text).style(style).to_string());
        }
        output.push('\n');
    }
    output
}

/// Replaces control characters, so that repository data can't inject escape
/// sequences. Everything onefetch prints goes through it.
fn sanitize(text: &str) -> String {
    text.replace(char::is_control, "\u{FFFD}")
}

fn write_unwrapped(writer: &mut dyn std::io::Write, content: &str) -> Result<()> {
    // \x1B[?7l turns off line wrapping and \x1B[?7h turns it on
    write!(writer, "\x1B[?7l{content}\x1B[?7h")?;
    Ok(())
}

impl PrinterKind {
    fn new(
        options: &RenderOptions,
        main_language: Option<Language>,
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
                        .or_else(|| main_language.map(|language| language.ascii_art().to_string()));

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
    use crate::render::style::{LanguageEntry, Style};

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
            main_language: Some(Language::Rust),
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
    fn test_create_plain_printer_when_no_main_language_no_ascii_input() {
        let printer = Printer::new(Info::default(), &RenderOptions::default()).unwrap();

        assert_eq!(printer.kind, PrinterKind::Plain);
    }

    #[test]
    fn test_create_ascii_printer_when_main_language() {
        let info = Info {
            main_language: Some(Language::Rust),
            ..Default::default()
        };

        let printer = Printer::new(info, &RenderOptions::default()).unwrap();

        assert!(matches!(printer.kind, PrinterKind::Ascii { .. }));
    }

    #[test]
    fn test_create_ascii_printer_when_ascii_language_without_main_language() {
        let options = RenderOptions {
            ascii_language: Some(Language::Rust),
            ..Default::default()
        };

        let printer = Printer::new(Info::default(), &options).unwrap();

        assert!(matches!(printer.kind, PrinterKind::Ascii { .. }));
    }

    #[test]
    fn test_ascii_beside_info() {
        let ascii = ascii_lines("{0}ab\n{0}cd\n{0}ef");
        let info = vec![Line::from("one"), Line::from("two")];
        let art = |text| Segment::new(text, Style::AsciiArt(Some(0)));
        let pad = || Segment::plain("   ");

        assert_eq!(
            ascii_beside_info(ascii, info),
            vec![
                Line::from(vec![art("ab"), pad(), Segment::value("one")]),
                Line::from(vec![art("cd"), pad(), Segment::value("two")]),
                Line::from(vec![art("ef")]),
            ]
        );
    }

    #[test]
    fn test_info_longer_than_ascii() {
        let ascii = ascii_lines("{0}ab");
        let info = vec![Line::from("one"), Line::from("two")];

        let lines = ascii_beside_info(ascii, info);

        assert_eq!(lines.len(), 2);
        assert_eq!(
            lines[1],
            Line::from(vec![
                Segment::plain("  "),
                Segment::plain("   "),
                Segment::value("two")
            ])
        );
    }

    fn paint_with_white(lines: &[Line], no_bold: bool) -> String {
        let options = RenderOptions {
            no_bold,
            ..Default::default()
        };
        // Without a language, the ascii art and the keys are white.
        paint(lines, &Colors::new(&options, None))
    }

    #[test]
    fn test_paint() {
        let line = Line::from(vec![
            Segment::new("title", Style::Key),
            Segment::new(":", Style::Separator),
            Segment::plain(" "),
            Segment::value("test"),
        ]);
        assert_eq!(
            paint_with_white(&[line], false),
            "\u{1b}[37;1mtitle\u{1b}[0m\u{1b}[39;1m:\u{1b}[0m \u{1b}[39mtest\u{1b}[0m\n"
        );
    }

    #[test]
    fn test_paint_no_bold() {
        let line = Line::from(vec![Segment::new("title", Style::Key)]);
        assert_eq!(
            paint_with_white(&[line], true),
            "\u{1b}[37mtitle\u{1b}[0m\n"
        );
    }

    #[test]
    fn test_paint_chips_and_palette() {
        // Without true colors, the first chip is red, like palette color 1.
        let go = LanguageEntry {
            language: Some(Language::Go),
            rank: 0,
        };
        let line = Line::from(vec![
            Segment::new("  ", Style::LanguageBar(go)),
            Segment::new("●", Style::LanguageChip(go)),
            Segment::new("   ", Style::Palette(1)),
        ]);
        assert_eq!(
            paint_with_white(&[line], false),
            "\u{1b}[41m  \u{1b}[0m\u{1b}[31m●\u{1b}[0m\u{1b}[41m   \u{1b}[0m\n"
        );
    }

    #[test]
    fn test_paint_ascii() {
        let line = Line::from(vec![Segment::new("██", Style::AsciiArt(Some(0)))]);
        assert_eq!(
            paint_with_white(&[line], false),
            "\u{1b}[37;1m██\u{1b}[0m\n"
        );
    }

    #[test]
    fn test_paint_empty_line() {
        assert_eq!(paint_with_white(&[Line::default()], false), "\n");
    }

    #[test]
    fn test_paint_sanitizes_text() {
        // OSC set-title sequence
        let painted = paint_with_white(&[Line::from("1.0.0\u{1b}]0;PWNED\u{07}")], false);
        assert!(painted.contains("1.0.0\u{FFFD}]0;PWNED\u{FFFD}"));
        assert!(!painted.contains('\u{07}'));
    }

    #[test]
    fn test_sanitize_keeps_printable_text() {
        assert_eq!(sanitize("Rust (100.0 %) ●"), "Rust (100.0 %) ●");
    }

    #[test]
    fn test_write_unwrapped() {
        let mut output = Vec::new();
        write_unwrapped(&mut output, "text").unwrap();
        assert_eq!(output, b"\x1B[?7ltext\x1B[?7h");
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
