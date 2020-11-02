use crate::onefetch::error::*;
use image::DynamicImage;

#[cfg(not(windows))]
pub mod iterm2;
#[cfg(not(windows))]
pub mod kitty;
#[cfg(not(windows))]
pub mod sixel;

pub trait ImageBackend {
    fn add_image(&self, lines: Vec<String>, image: &DynamicImage, colors: usize) -> String;
}

#[cfg(not(windows))]
pub fn get_best_backend() -> Option<Box<dyn ImageBackend>> {
    if kitty::KittyBackend::supported() {
        Some(Box::new(kitty::KittyBackend::new()))
    } else if iterm2::ITerm2Backend::supported() {
        Some(Box::new(iterm2::ITerm2Backend::new()))
    } else if sixel::SixelBackend::supported() {
        Some(Box::new(sixel::SixelBackend::new()))
    } else {
        None
    }
}

pub fn check_if_supported(backend_name: &str) -> Result<()> {
    #[cfg(windows)]
    return Err(format!("{} image backend is not supported", backend_name).into());

    #[cfg(not(windows))]
    match backend_name {
        "kitty" => {
            if !kitty::KittyBackend::supported() {
                return Err("Kitty image backend is not supported".into());
            }
        }
        "iterm2" => {
            if !iterm2::ITerm2Backend::supported() {
                return Err("Sixel image backend is not supported".into());
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
        "iterm2" => Box::new(iterm2::ITerm2Backend::new()) as Box<dyn ImageBackend>,
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
