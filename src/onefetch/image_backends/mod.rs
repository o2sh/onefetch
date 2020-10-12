use image::DynamicImage;

#[cfg(target_os = "linux")]
pub mod kitty;
#[cfg(target_os = "linux")]
pub mod sixel;

pub trait ImageBackend {
    fn add_image(&self, lines: Vec<String>, image: &DynamicImage) -> String;
}

#[cfg(target_os = "linux")]
pub fn get_best_backend() -> Option<Box<dyn ImageBackend>> {
    if kitty::KittyBackend::supported() {
        Some(Box::new(kitty::KittyBackend::new()))
    } else if sixel::SixelBackend::supported() {
        Some(Box::new(sixel::SixelBackend::new()))
    } else {
        None
    }
}

pub fn get_image_backend(backend_name: &str) -> Option<Box<dyn ImageBackend>> {
    #[cfg(target_os = "linux")]
    let backend = Some(match backend_name {
        "kitty" => Box::new(kitty::KittyBackend::new()) as Box<dyn ImageBackend>,
        "sixel" => Box::new(sixel::SixelBackend::new()) as Box<dyn ImageBackend>,
        _ => unreachable!(),
    });
    #[cfg(not(target_os = "linux"))]
    let backend = None;
    backend
}

#[cfg(not(target_os = "linux"))]
pub fn get_best_backend() -> Option<Box<dyn ImageBackend>> {
    None
}
