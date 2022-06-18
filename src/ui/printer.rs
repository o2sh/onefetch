use crate::info::Info;
use crate::ui::ascii_art::AsciiArt;
use anyhow::{Context, Result};
use std::io::Write;
use strum::{EnumIter, EnumString, IntoStaticStr};

const CENTER_PAD_LENGTH: usize = 3;

#[derive(Clone, Copy, EnumString, EnumIter, IntoStaticStr, clap::ValueEnum)]
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
                    writeln!(self.writer, "{}", serde_json::to_string_pretty(&self.info)?)?
                }
                SerializationFormat::Yaml => {
                    writeln!(self.writer, "{}", serde_yaml::to_string(&self.info)?)?
                }
            },
            None => {
                let center_pad = " ".repeat(CENTER_PAD_LENGTH);
                let info_str = format!("{}", &self.info);
                let mut info_lines = info_str.lines();
                let mut buf = String::new();

                if true {
                    buf.push_str(&info_str);
                } else {
                    let mut logo_lines = if let Some(custom_ascii) = &self.info.config.ascii_input {
                        AsciiArt::new(
                            custom_ascii,
                            &self.info.ascii_colors,
                            !self.info.config.no_bold,
                        )
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
