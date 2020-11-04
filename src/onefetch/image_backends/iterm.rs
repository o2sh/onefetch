use {
    crate::onefetch::error::*,
    image::{imageops::FilterType, DynamicImage, GenericImageView},
    libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ},
    std::env,
};

pub struct ITermBackend {}

impl ITermBackend {
    pub fn new() -> Self {
        ITermBackend {}
    }

    pub fn supported() -> bool {
        let term_program = env::var("TERM_PROGRAM").unwrap_or_else(|_| "".to_string());
        term_program == "iTerm.app"
    }
}

impl super::ImageBackend for ITermBackend {
    fn add_image(
        &self,
        lines: Vec<String>,
        image: &DynamicImage,
        _colors: usize,
    ) -> Result<String> {
        let tty_size = unsafe {
            let tty_size: winsize = std::mem::zeroed();
            ioctl(STDOUT_FILENO, TIOCGWINSZ, &tty_size);
            tty_size
        };
        let width_ratio = f64::from(tty_size.ws_col) / f64::from(tty_size.ws_xpixel);
        let height_ratio = f64::from(tty_size.ws_row) / f64::from(tty_size.ws_ypixel);

        // resize image to fit the text height with the Lanczos3 algorithm
        let image = image.resize(
            u32::max_value(),
            (lines.len() as f64 / height_ratio) as u32,
            FilterType::Lanczos3,
        );
        let _image_columns = width_ratio * f64::from(image.width());
        let image_rows = height_ratio * f64::from(image.height());

        let mut bytes: Vec<u8> = Vec::new();
        image.write_to(&mut bytes, image::ImageOutputFormat::Png)?;
        let encoded_image = base64::encode(bytes);
        let mut image_data = Vec::<u8>::new();

        image_data.extend(b"\x1B]1337;File=inline=1:");
        image_data.extend(encoded_image.bytes());
        image_data.extend(b"\x07");

        image_data.extend(format!("\x1B[{}A", image_rows as u32 - 1).as_bytes()); // move cursor to start of image
        let mut i = 0;
        for line in &lines {
            image_data.extend(format!("\x1B[s{}\x1B[u\x1B[1B", line).as_bytes());
            i += 1;
        }
        image_data
            .extend(format!("\n\x1B[{}B", lines.len().max(image_rows as usize) - i).as_bytes()); // move cursor to end of image

        Ok(String::from_utf8(image_data)?)
    }
}
