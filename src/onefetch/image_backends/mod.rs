use crate::onefetch::error::*;
use image::DynamicImage;

#[cfg(not(windows))]
pub mod iterm;
#[cfg(not(windows))]
pub mod kitty;
#[cfg(not(windows))]
pub mod sixel;

pub trait ImageBackend {
    fn add_image(&self, lines: Vec<String>, image: &DynamicImage, colors: usize) -> Result<String>;
}

pub fn get_best_backend() -> Option<Box<dyn ImageBackend>> {
    #[cfg(not(windows))]
    if kitty::KittyBackend::supported() {
        Some(Box::new(kitty::KittyBackend::new()))
    } else if iterm::ITermBackend::supported() {
        Some(Box::new(iterm::ITermBackend::new()))
    } else if sixel::SixelBackend::supported() {
        Some(Box::new(sixel::SixelBackend::new()))
    } else {
        None
    }

    #[cfg(windows)]
    None
}

pub fn get_image_backend(backend_name: &str) -> Option<Box<dyn ImageBackend>> {
    #[cfg(not(windows))]
    let backend = Some(match backend_name {
        "kitty" => Box::new(kitty::KittyBackend::new()) as Box<dyn ImageBackend>,
        "iterm" => Box::new(iterm::ITermBackend::new()) as Box<dyn ImageBackend>,
        "sixel" => Box::new(sixel::SixelBackend::new()) as Box<dyn ImageBackend>,
        _ => unreachable!(),
    });

    #[cfg(windows)]
    let backend = None;
    backend
}
