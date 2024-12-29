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

#[allow(unused_variables)]
pub fn get_image_backend(image_protocol: ImageProtocol) -> Option<Box<dyn ImageBackend>> {
    #[cfg(not(windows))]
    let backend = Some(match image_protocol {
        ImageProtocol::Kitty => Box::new(kitty::KittyBackend::new()) as Box<dyn ImageBackend>,
        ImageProtocol::Iterm => Box::new(iterm::ITermBackend::new()) as Box<dyn ImageBackend>,
        ImageProtocol::Sixel => Box::new(sixel::SixelBackend::new()) as Box<dyn ImageBackend>,
    });

    #[cfg(windows)]
    let backend = None;
    backend
}

#[cfg(not(windows))]
unsafe fn get_dimensions() -> libc::winsize {
    use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
    use std::mem::zeroed;

    let mut window: winsize = zeroed();
    let result = ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut window);

    if result == -1 {
        zeroed()
    } else {
        window
    }
}
