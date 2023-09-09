use crate::cli::CliOptions;
use crate::info::Info;
use crate::ui::Language;
use anyhow::{Context, Result};
use image::DynamicImage;
use onefetch_ascii::AsciiArt;
use onefetch_image::ImageBackend;
use std::fmt::Write as _;
use std::io::Write;

const CENTER_PAD_LENGTH: usize = 3;

#[derive(Clone, clap::ValueEnum, PartialEq, Eq, Debug)]
pub enum SerializationFormat {
    Json,
    Yaml,
}

pub struct Printer<W> {
    writer: W,
    info: Info,
    output: Option<SerializationFormat>,
    art_off: bool,
    image: Option<DynamicImage>,
    image_backend: Option<Box<dyn ImageBackend>>,
    color_resolution: usize,
    no_bold: bool,
    ascii_input: Option<String>,
    ascii_language: Option<Language>,
}

impl<W: Write> Printer<W> {
    pub fn new(writer: W, info: Info, cli_options: CliOptions) -> Result<Self> {
        let image = match cli_options.image.image {
            Some(p) => Some(image::open(p).context("Could not load the specified image")?),
            None => None,
        };

        let image_backend = if image.is_some() {
            cli_options
                .image
                .image_protocol
                .map_or_else(onefetch_image::get_best_backend, |s| {
                    onefetch_image::get_image_backend(s)
                })
        } else {
            None
        };

        Ok(Self {
            writer,
            info,
            output: cli_options.developer.output,
            art_off: cli_options.visuals.no_art,
            image,
            image_backend,
            color_resolution: cli_options.image.color_resolution,
            no_bold: cli_options.text_formatting.no_bold,
            ascii_input: cli_options.ascii.ascii_input,
            ascii_language: cli_options.ascii.ascii_language,
        })
    }

    pub fn print(&mut self) -> Result<()> {
        match &self.output {
            Some(format) => match format {
                SerializationFormat::Json => {
                    writeln!(self.writer, "{}", serde_json::to_string_pretty(&self.info)?)?;
                }
                SerializationFormat::Yaml => {
                    writeln!(self.writer, "{}", serde_yaml::to_string(&self.info)?)?;
                }
            },
            None => {
                let center_pad = " ".repeat(CENTER_PAD_LENGTH);
                let info_str = self.info.to_string();
                let mut info_lines = info_str.lines();
                let mut buf = String::new();

                if self.art_off {
                    buf.push_str(&info_str);
                } else if let Some(custom_image) = &self.image {
                    let image_backend = self
                        .image_backend
                        .as_ref()
                        .context("Could not detect a supported image backend")?;

                    buf.push_str(
                        &image_backend
                            .add_image(
                                info_lines.map(|s| format!("{center_pad}{s}")).collect(),
                                custom_image,
                                self.color_resolution,
                            )
                            .context("Error while drawing image")?,
                    );
                } else {
                    let mut logo_lines = if let Some(custom_ascii) = &self.ascii_input {
                        AsciiArt::new(custom_ascii, &self.info.ascii_colors, !self.no_bold)
                    } else {
                        AsciiArt::new(self.get_ascii(), &self.info.ascii_colors, !self.no_bold)
                    };

                    loop {
                        match (logo_lines.next(), info_lines.next()) {
                            (Some(logo_line), Some(info_line)) => {
                                writeln!(buf, "{logo_line}{center_pad}{info_line:^}")?;
                            }
                            (Some(logo_line), None) => writeln!(buf, "{logo_line}")?,
                            (None, Some(info_line)) => writeln!(
                                buf,
                                "{:<width$}{}{:^}",
                                "",
                                center_pad,
                                info_line,
                                width = logo_lines.width()
                            )?,
                            (None, None) => {
                                buf.push('\n');
                                break;
                            }
                        }
                    }
                }

                // \x1B[?7l turns off line wrapping and \x1B[?7h turns it on
                write!(self.writer, "\x1B[?7l{buf}\x1B[?7h")?;
            }
        }
        Ok(())
    }

    fn get_ascii(&self) -> &str {
        let language = if let Some(ascii_language) = &self.ascii_language {
            ascii_language
        } else {
            &self.info.dominant_language
        };

        language.get_ascii_art()
    }
}
