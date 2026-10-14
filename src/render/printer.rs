use super::lines::info_lines;
use super::logo::AsciiArt;
use super::terminal::ansi;
use super::terminal::image::{self, ImageBackend};
use super::text::{Line, Span};
use super::theme::Theme;
use super::{RenderOptions, SerializationFormat};
use crate::data::Info;
use crate::language::Language;
use ::image::DynamicImage;
use anyhow::{Context, Result};

const CENTER_PAD_LENGTH: usize = 3;

pub struct Printer {
    info: Info,
    kind: PrinterKind,
    options: RenderOptions,
    theme: Theme,
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

        let theme = Theme::new(options, info.dominant_language);

        let kind = PrinterKind::new(options, info.dominant_language, image, image_backend)?;

        Ok(Self {
            info,
            kind,
            options: options.clone(),
            theme,
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
                let info_text = ansi::paint(&self.info_lines(), &self.theme);
                ansi::write_unwrapped(writer, &info_text)
            }
            PrinterKind::Image {
                image,
                backend,
                resolution,
            } => {
                let center_pad = " ".repeat(CENTER_PAD_LENGTH);
                let info_text = ansi::paint(&self.info_lines(), &self.theme);
                let info_lines = info_text
                    .lines()
                    .map(|s| format!("{center_pad}{s}"))
                    .collect();

                let rendered = backend
                    .add_image(info_lines, image, *resolution)
                    .context("Failed to render image")?;

                ansi::write_unwrapped(writer, &rendered)
            }
            PrinterKind::Ascii { art } => {
                let logo = AsciiArt::new(art, &self.theme.logo);
                let lines = logo_beside_info(logo, self.info_lines());
                ansi::write_unwrapped(writer, &ansi::paint(&lines, &self.theme))
            }
        }
    }

    fn info_lines(&self) -> Vec<Line> {
        info_lines(&self.info, &self.options)
    }
}

/// Puts the logo on the left of the info lines.
fn logo_beside_info(mut logo: AsciiArt, info: Vec<Line>) -> Vec<Line> {
    let center_pad = || Span::plain(" ".repeat(CENTER_PAD_LENGTH));
    let mut info = info.into_iter();
    let mut lines = Vec::new();

    loop {
        let spans = match (logo.next(), info.next()) {
            (Some(logo), Some(info)) => [logo.0, vec![center_pad()], info.0].concat(),
            (Some(logo), None) => logo.0,
            (None, Some(info)) => {
                let blank = Span::plain(" ".repeat(logo.width()));
                [vec![blank, center_pad()], info.0].concat()
            }
            (None, None) => break,
        };
        lines.push(Line::from(spans));
    }

    lines
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
    fn test_logo_beside_info() {
        let colors = [];
        let logo = AsciiArt::new("{0}ab\n{0}cd\n{0}ef", &colors);
        let info = vec![Line::from("one"), Line::from("two")];
        let pad = Span::plain("   ");

        assert_eq!(
            logo_beside_info(logo, info.clone()),
            vec![
                Line::from(
                    [
                        AsciiArt::new("{0}ab", &colors).next().unwrap().0,
                        vec![pad.clone()],
                        info[0].0.clone()
                    ]
                    .concat()
                ),
                Line::from(
                    [
                        AsciiArt::new("{0}cd", &colors).next().unwrap().0,
                        vec![pad.clone()],
                        info[1].0.clone()
                    ]
                    .concat()
                ),
                AsciiArt::new("{0}ef", &colors).next().unwrap(),
            ]
        );
    }

    #[test]
    fn test_info_longer_than_logo() {
        let colors = [];
        let logo = AsciiArt::new("{0}ab", &colors);
        let info = vec![Line::from("one"), Line::from("two")];

        let lines = logo_beside_info(logo, info);

        assert_eq!(lines.len(), 2);
        assert_eq!(
            lines[1],
            Line::from(vec![
                Span::plain("  "),
                Span::plain("   "),
                Span::value("two")
            ])
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
