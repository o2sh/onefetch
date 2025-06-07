use super::Printer;
use crate::info::Info;
use anyhow::Result;
use std::io::Write;

pub struct YamlPrinter {
    pub info: Info,
}

impl Printer for YamlPrinter {
    fn print(&self, writer: &mut dyn Write) -> Result<()> {
        writeln!(writer, "{}", serde_yaml::to_string(&self.info)?)?;
        Ok(())
    }
}
