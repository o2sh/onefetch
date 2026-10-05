use crate::info::Info;
use crate::info::display_options::DisplayOptions;
use crate::ui::text_colors::TextColors;
use ::image::DynamicImage;
use anyhow::{Context, Result};
use onefetch_ascii::AsciiArt;
use onefetch_image::ImageBackend;
use std::fmt::Write as _;

mod ansi;
pub mod factory;

const CENTER_PAD_LENGTH: usize = 3;

#[derive(Clone, clap::ValueEnum, PartialEq, Eq, Debug)]
pub enum SerializationFormat {
    Json,
    Yaml,
}

pub struct Printer {
    info: Info,
    r#type: PrinterType,
    no_bold: bool,
    text_colors: TextColors,
    display_options: DisplayOptions,
}

enum PrinterType {
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
    pub fn print(&self, writer: &mut dyn std::io::Write) -> Result<()> {
        match &self.r#type {
            PrinterType::Json => {
                write!(writer, "{}", serde_json::to_string_pretty(&self.info)?)?;
                Ok(())
            }
            PrinterType::Yaml => {
                write!(writer, "{}", serde_yaml::to_string(&self.info)?)?;
                Ok(())
            }
            PrinterType::Plain => {
                write_with_line_wrapping(writer, &self.info_text())?;
                Ok(())
            }
            PrinterType::Image {
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
            PrinterType::Ascii { art } => {
                let mut buf = String::new();
                let center_pad = " ".repeat(CENTER_PAD_LENGTH);
                let info_text = self.info_text();
                let mut info_lines = info_text.lines();
                let mut logo_lines = AsciiArt::new(art, &self.info.ascii_colors, !self.no_bold);

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
            &self.info.lines(&self.display_options),
            &self.text_colors,
            self.no_bold,
        )
    }
}

fn write_with_line_wrapping(writer: &mut dyn std::io::Write, content: &str) -> Result<()> {
    // \x1B[?7l turns off line wrapping and \x1B[?7h turns it on
    write!(writer, "\x1B[?7l{content}\x1B[?7h")?;
    Ok(())
}

impl PartialEq for PrinterType {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (PrinterType::Plain, PrinterType::Plain)
                | (PrinterType::Json, PrinterType::Json)
                | (PrinterType::Yaml, PrinterType::Yaml)
                | (PrinterType::Ascii { .. }, PrinterType::Ascii { .. })
                | (PrinterType::Image { .. }, PrinterType::Image { .. })
        )
    }
}

impl std::fmt::Debug for PrinterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            PrinterType::Plain => "Plain",
            PrinterType::Json => "Json",
            PrinterType::Yaml => "Yaml",
            PrinterType::Ascii { .. } => "Ascii",
            PrinterType::Image { .. } => "Image",
        };
        write!(f, "PrinterType::{name}")
    }
}
