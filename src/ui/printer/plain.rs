use crate::{info::Info, ui::printer::Printer};
use anyhow::Result;
use std::io::Write;

pub struct PlainPrinter {
    pub info: Info,
}

impl Printer for PlainPrinter {
    fn print(&self, writer: &mut dyn Write) -> Result<()> {
        write!(writer, "{}", self.info.to_string())?;
        Ok(())
    }
}
