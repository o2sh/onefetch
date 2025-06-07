use super::Printer;
use crate::{info::Info, ui::printer::CENTER_PAD_LENGTH};
use anyhow::Result;
use onefetch_ascii::AsciiArt;
use std::fmt::Write as _;
use std::io::Write;

pub struct AsciiPrinter {
    pub info: Info,
    pub ascii_art: String,
    pub no_bold: bool,
}

impl Printer for AsciiPrinter {
    fn print(&self, writer: &mut dyn Write) -> Result<()> {
        let mut buf = String::new();
        let center_pad = " ".repeat(CENTER_PAD_LENGTH);
        let info_str = self.info.to_string();
        let mut info_lines = info_str.lines();
        let mut logo_lines = AsciiArt::new(&self.ascii_art, &self.info.ascii_colors, !self.no_bold);

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

        write!(writer, "\x1B[?7l{buf}\x1B[?7h")?;
        Ok(())
    }
}
