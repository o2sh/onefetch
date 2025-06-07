use super::Printer;
use crate::info::Info;
use anyhow::Result;
use serde_json;
use std::io::Write;

pub struct JsonPrinter {
    pub info: Info,
}

impl Printer for JsonPrinter {
    fn print(&self, writer: &mut dyn Write) -> Result<()> {
        writeln!(writer, "{}", serde_json::to_string_pretty(&self.info)?)?;
        Ok(())
    }
}
