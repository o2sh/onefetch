use super::Printer;
use crate::{info::Info, ui::printer::CENTER_PAD_LENGTH};
use anyhow::{Context, Result};
use image::DynamicImage;
use onefetch_image::ImageBackend;
use std::io::Write;

pub struct ImagePrinter {
    pub info: Info,
    pub image: DynamicImage,
    pub image_backend: Box<dyn ImageBackend>,
    pub color_resolution: usize,
}

impl Printer for ImagePrinter {
    fn print(&self, writer: &mut dyn Write) -> Result<()> {
        let center_pad = " ".repeat(CENTER_PAD_LENGTH);
        let info_str = self.info.to_string();
        let info_lines = info_str
            .lines()
            .map(|s| format!("{center_pad}{s}"))
            .collect();

        let rendered = self
            .image_backend
            .add_image(info_lines, &self.image, self.color_resolution)
            .context("Failed to render image")?;

        write!(writer, "\x1B[?7l{rendered}\x1B[?7h")?;
        Ok(())
    }
}
