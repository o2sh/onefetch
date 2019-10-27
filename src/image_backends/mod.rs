use image::DynamicImage;

#[cfg(target_os = "linux")]
mod kitty;

pub trait ImageBackend {
    fn add_image(&self, lines: Vec<String>, image: &DynamicImage) -> String;
}

#[cfg(target_os = "linux")]
pub fn get_best_backend() -> Option<Box<dyn ImageBackend>> {
    if kitty::KittyBackend::supported() {
        Some(Box::new(kitty::KittyBackend::new()))
    } else {
        None
    }
}

#[cfg(not(target_os = "linux"))]
pub fn get_best_backend() -> Option<Box<dyn ImageBackend>> {
    None
}
