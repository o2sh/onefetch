pub mod ascii;
mod colors;
mod fields;
pub mod format;
pub mod image;
pub mod info;
mod printer;
pub mod style;

pub use printer::Printer;

#[derive(Clone, clap::ValueEnum, PartialEq, Eq, Debug)]
pub enum SerializationFormat {
    Json,
    Yaml,
}
