use crate::info::Info;
use ::image::DynamicImage;
use anyhow::{Context, Result};
use onefetch_ascii::AsciiArt;
use onefetch_image::ImageBackend;
use std::fmt::Write as _;

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
}

enum PrinterType {
    Plain,
    Json,
    Yaml,
    Ascii {
        art: String,
        no_bold: bool,
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
                writeln!(writer, "{}", serde_json::to_string_pretty(&self.info)?)?;
                Ok(())
            }
            PrinterType::Yaml => {
                writeln!(writer, "{}", serde_yaml::to_string(&self.info)?)?;
                Ok(())
            }
            PrinterType::Plain => {
                write!(writer, "\x1B[?7l{}\x1B[?7h", self.info)?;
                Ok(())
            }
            PrinterType::Image {
                image,
                backend,
                resolution,
            } => {
                let center_pad = " ".repeat(CENTER_PAD_LENGTH);
                let info_str = self.info.to_string();
                let info_lines = info_str
                    .lines()
                    .map(|s| format!("{center_pad}{s}"))
                    .collect();

                let rendered = backend
                    .add_image(info_lines, image, *resolution)
                    .context("Failed to render image")?;

                write!(writer, "\x1B[?7l{rendered}\x1B[?7h")?;
                Ok(())
            }
            PrinterType::Ascii { art, no_bold } => {
                let mut buf = String::new();
                let center_pad = " ".repeat(CENTER_PAD_LENGTH);
                let info_str = self.info.to_string();
                let mut info_lines = info_str.lines();
                let mut logo_lines = AsciiArt::new(art, &self.info.ascii_colors, !no_bold);

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
                        (None, None) => {
                            buf.push('\n');
                            break;
                        }
                    }
                }

                // \x1B[?7l turns off line wrapping and \x1B[?7h turns it on
                write!(writer, "\x1B[?7l{buf}\x1B[?7h")?;
                Ok(())
            }
        }
    }
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
