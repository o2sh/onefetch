use crate::onefetch::{ascii_art::AsciiArt, error::*, info::Info};
use colored::Color;
use std::io::Write;
use strum::{EnumIter, EnumString, IntoStaticStr};

const CENTER_PAD_LENGTH: usize = 3;

#[derive(EnumString, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
pub enum SerializationFormat {
    Json,
    Yaml,
}

pub struct Printer<W> {
    writer: W,
    info: Info,
}

impl<W: Write> Printer<W> {
    pub fn new(writer: W, info: Info) -> Self {
        Self { writer, info }
    }

    pub fn print(&mut self) -> Result<()> {
        match &self.info.config.output {
            Some(format) => match format {
                SerializationFormat::Json => {
                    writeln!(self.writer, "{}", serde_json::to_string_pretty(&self.info).unwrap())?
                }
                SerializationFormat::Yaml => {
                    writeln!(self.writer, "{}", serde_yaml::to_string(&self.info).unwrap())?
                }
            },
            None => {
                let center_pad = " ".repeat(CENTER_PAD_LENGTH);
                let info_str = format!("{}", &self.info);
                let mut info_lines = info_str.lines();
                let colors: Vec<Color> = Vec::new();
                let mut buf = String::new();

                if self.info.config.art_off {
                    buf.push_str(&info_str);
                } else if let Some(custom_image) = &self.info.config.image {
                    buf.push_str(
                        &self
                            .info
                            .config
                            .image_backend
                            .as_ref()
                            .unwrap()
                            .add_image(
                                info_lines.map(|s| format!("{}{}", center_pad, s)).collect(),
                                custom_image,
                                self.info.config.image_color_resolution,
                            )
                            .chain_err(|| "Error while drawing image")?,
                    );
                } else {
                    let mut logo_lines = if let Some(custom_ascii) = &self.info.config.ascii_input {
                        AsciiArt::new(custom_ascii, &colors, !self.info.config.no_bold)
                    } else {
                        AsciiArt::new(
                            self.get_ascii(),
                            &self.info.ascii_colors,
                            !self.info.config.no_bold,
                        )
                    };

                    loop {
                        match (logo_lines.next(), info_lines.next()) {
                            (Some(logo_line), Some(info_line)) => buf
                                .push_str(&format!("{}{}{:^}\n", logo_line, center_pad, info_line)),
                            (Some(logo_line), None) => buf.push_str(&format!("{}\n", logo_line)),
                            (None, Some(info_line)) => buf.push_str(&format!(
                                "{:<width$}{}{:^}\n",
                                "",
                                center_pad,
                                info_line,
                                width = logo_lines.width()
                            )),
                            (None, None) => {
                                buf.push('\n');
                                break;
                            }
                        }
                    }
                }

                write!(self.writer, "{}", buf)?;
            }
        }
        Ok(())
    }

    fn get_ascii(&self) -> &str {
        let language = if let Some(ascii_language) = &self.info.config.ascii_language {
            ascii_language
        } else {
            &self.info.dominant_language
        };

        language.get_ascii_art()
    }
}
