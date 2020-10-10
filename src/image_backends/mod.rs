use image::DynamicImage;

#[cfg(unix)]
pub mod kitty;
#[cfg(unix)]
pub mod sixel;

pub trait ImageBackend {
    fn add_image(&self, lines: Vec<String>, image: &DynamicImage) -> String;
}

#[cfg(unix)]
pub fn get_best_backend() -> Option<Box<dyn ImageBackend>> {
    if kitty::KittyBackend::supported() {
        Some(Box::new(kitty::KittyBackend::new()))
    } else if sixel::SixelBackend::supported() {
        Some(Box::new(sixel::SixelBackend::new()))
    } else {
        None
    }
}

pub fn get_image_backend(
    image: &Option<DynamicImage>,
    backend_name: &str,
) -> Option<Box<dyn ImageBackend>> {
    if image.is_some() {
        #[cfg(unix)]
        let backend = Some(match backend_name {
            "kitty" => {
                Box::new(kitty::KittyBackend::new()) as Box<dyn ImageBackend>
            }
            "sixel" => {
                Box::new(sixel::SixelBackend::new()) as Box<dyn ImageBackend>
            }
            _ => unreachable!(),
        });
        #[cfg(not(unix))]
        let backend = None;
        backend
    } else {
        None
    }
}

#[cfg(not(unix))]
pub fn get_best_backend() -> Option<Box<dyn ImageBackend>> {
    None
}
