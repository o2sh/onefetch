use anyhow::Result;
use image::DynamicImage;


#[derive(clap::ValueEnum, Clone, PartialEq, Eq, Debug)]
pub enum ImageProtocol {
    Kitty,
    Sixel,
    Iterm,
}

#[cfg(not(windows))]
pub mod iterm;
#[cfg(not(windows))]
pub mod kitty;
#[cfg(not(windows))]
pub mod sixel;

pub trait ImageBackend {
    fn add_image(&self, lines: Vec<String>, image: &DynamicImage, colors: usize) -> Result<String>;
}

pub fn get_best_backend() -> Result<Option<Box<dyn ImageBackend>>> {
    #[cfg(not(windows))]
    if sixel::SixelBackend::supported()? {
        Ok(Some(Box::new(sixel::SixelBackend)))
    } else if kitty::KittyBackend::supported()? {
        Ok(Some(Box::new(kitty::KittyBackend)))
    } else if iterm::ITermBackend::supported() {
        Ok(Some(Box::new(iterm::ITermBackend)))
    } else {
        Ok(None)
    }

    #[cfg(windows)]
    Ok(None)
}

#[allow(unused_variables)]
pub fn get_image_backend(image_protocol: ImageProtocol) -> Option<Box<dyn ImageBackend>> {
    #[cfg(not(windows))]
    let backend = Some(match image_protocol {
        ImageProtocol::Kitty => Box::new(kitty::KittyBackend) as Box<dyn ImageBackend>,
        ImageProtocol::Iterm => Box::new(iterm::ITermBackend) as Box<dyn ImageBackend>,
        ImageProtocol::Sixel => Box::new(sixel::SixelBackend) as Box<dyn ImageBackend>,
    });

    #[cfg(windows)]
    let backend = None;
    backend
}
