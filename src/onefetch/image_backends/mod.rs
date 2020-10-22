use crate::onefetch::error::*;
use image::DynamicImage;

#[cfg(not(windows))]
pub mod kitty;
#[cfg(not(windows))]
pub mod sixel;

pub trait ImageBackend {
    fn add_image(&self, lines: Vec<String>, image: &DynamicImage) -> String;
}

#[cfg(not(windows))]
pub fn get_best_backend() -> Option<Box<dyn ImageBackend>> {
    if kitty::KittyBackend::supported() {
        Some(Box::new(kitty::KittyBackend::new()))
    } else if sixel::SixelBackend::supported() {
        Some(Box::new(sixel::SixelBackend::new()))
    } else {
        None
    }
}

pub fn check_if_supported(backend_name: &str) -> Result<()> {

    #[cfg(windows)]
    return Err(format!("{} image backend is not supported", backend_name));

    match backend_name {
        "kitty" => {
            if !kitty::KittyBackend::supported() {
                return Err("Kitty image backend is not supported".into());
            }
        }
        "sixel" => {
            if !sixel::SixelBackend::supported() {
                return Err("Sixel image backend is not supported".into());
            }
        }
        _ => unreachable!(),
    };

    Ok(())
}

pub fn get_image_backend(backend_name: &str) -> Option<Box<dyn ImageBackend>> {
    #[cfg(not(windows))]
    let backend = Some(match backend_name {
        "kitty" => Box::new(kitty::KittyBackend::new()) as Box<dyn ImageBackend>,
        "sixel" => Box::new(sixel::SixelBackend::new()) as Box<dyn ImageBackend>,
        _ => unreachable!(),
    });
    #[cfg(windows)]
    let backend = None;
    backend
}

#[cfg(windows)]
pub fn get_best_backend() -> Option<Box<dyn ImageBackend>> {
    None
}
