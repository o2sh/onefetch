use std::any::Any;
mod ascii;
pub mod factory;
mod image;
mod json;
mod plain;
mod yaml;

const CENTER_PAD_LENGTH: usize = 3;

#[derive(Clone, clap::ValueEnum, PartialEq, Eq, Debug)]
pub enum SerializationFormat {
    Json,
    Yaml,
}

pub trait Printer: Any {
    fn print(&self, writer: &mut dyn std::io::Write) -> anyhow::Result<()>;
}
