use {
    crate::onefetch::error::*,
    color_quant::NeuQuant,
    image::{
        imageops::{colorops, FilterType},
        DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgb,
    },
    libc::{
        c_void, ioctl, poll, pollfd, read, tcgetattr, tcsetattr, termios, winsize, ECHO, ICANON,
        POLLIN, STDIN_FILENO, STDOUT_FILENO, TCSANOW, TIOCGWINSZ,
    },
    std::io::{stdout, Write},
    std::time::Instant,
};

pub struct SixelBackend {}

impl SixelBackend {
    pub fn new() -> Self {
        Self {}
    }

    pub fn supported() -> bool {
        // save terminal attributes and disable canonical input processing mode
        let old_attributes = unsafe {
            let mut old_attributes: termios = std::mem::zeroed();
            tcgetattr(STDIN_FILENO, &mut old_attributes);

            let mut new_attributes = old_attributes;
            new_attributes.c_lflag &= !ICANON;
            new_attributes.c_lflag &= !ECHO;
            tcsetattr(STDIN_FILENO, TCSANOW, &new_attributes);
            old_attributes
        };

        // ask for the primary device attribute string
        print!("\x1B[c");
        stdout().flush().unwrap();

        let start_time = Instant::now();
        let mut stdin_pollfd = pollfd { fd: STDIN_FILENO, events: POLLIN, revents: 0 };
        let mut buf = Vec::<u8>::new();
        loop {
            // check for timeout while polling to avoid blocking the main thread
            while unsafe { poll(&mut stdin_pollfd, 1, 0) < 1 } {
                if start_time.elapsed().as_millis() > 50 {
                    unsafe {
                        tcsetattr(STDIN_FILENO, TCSANOW, &old_attributes);
                    }
                    return false;
                }
            }
            let mut byte = 0;
            unsafe {
                read(STDIN_FILENO, &mut byte as *mut _ as *mut c_void, 1);
            }
            buf.push(byte);
            if buf.starts_with(&[0x1B, b'[', b'?']) && buf.ends_with(&[b'c']) {
                for attribute in buf[3..(buf.len() - 1)].split(|x| *x == b';') {
                    if attribute == [b'4'] {
                        unsafe {
                            tcsetattr(STDIN_FILENO, TCSANOW, &old_attributes);
                        }
                        return true;
                    }
                }
            }
        }
    }
}

impl super::ImageBackend for SixelBackend {
    #[allow(clippy::map_entry)]
    fn add_image(&self, lines: Vec<String>, image: &DynamicImage, colors: usize) -> Result<String> {
        let tty_size = unsafe {
            let tty_size: winsize = std::mem::zeroed();
            ioctl(STDOUT_FILENO, TIOCGWINSZ, &tty_size);
            tty_size
        };
        let cw = tty_size.ws_xpixel / tty_size.ws_col;
        let lh = tty_size.ws_ypixel / tty_size.ws_row;
        let width_ratio = 1.0 / cw as f64;
        let height_ratio = 1.0 / lh as f64;

        // resize image to fit the text height with the Lanczos3 algorithm
        let image = image.resize(
            u32::max_value(),
            (lines.len() as f64 / height_ratio) as u32,
            FilterType::Lanczos3,
        );
        let image_columns = width_ratio * image.width() as f64;
        let image_rows = height_ratio * image.height() as f64;

        let rgba_image = image.to_rgba8(); // convert the image to rgba samples
        let flat_samples = rgba_image.as_flat_samples();
        let mut rgba_image = rgba_image.clone();
        // reduce the amount of colors using dithering
        colorops::dither(
            &mut rgba_image,
            &NeuQuant::new(
                10,
                colors,
                flat_samples.image_slice().ok_or("Error while slicing the image")?,
            ),
        );
        let rgb_image = ImageBuffer::from_fn(rgba_image.width(), rgba_image.height(), |x, y| {
            let rgba_pixel = rgba_image.get_pixel(x, y);
            let mut rgb_pixel = rgba_pixel.to_rgb();
            for subpixel in &mut rgb_pixel.0 {
                *subpixel = (*subpixel as f32 / 255.0 * rgba_pixel[3] as f32) as u8;
            }
            rgb_pixel
        });

        let mut image_data = Vec::<u8>::new();
        image_data.extend(b"\x1BPq"); // start sixel data
        image_data.extend(format!("\"1;1;{};{}", image.width(), image.height()).as_bytes());

        let mut colors = std::collections::HashMap::<Rgb<u8>, u8>::new();
        // subtract 1 -> divide -> add 1 to round up the integer division
        for i in 0..((rgb_image.height() - 1) / 6 + 1) {
            let sixel_row = rgb_image.view(
                0,
                i * 6,
                rgb_image.width(),
                std::cmp::min(6, rgb_image.height() - i * 6),
            );
            for (_, _, pixel) in sixel_row.pixels() {
                if !colors.contains_key(&pixel) {
                    // sixel uses percentages for rgb values
                    let color_multiplier = 100.0 / 255.0;
                    image_data.extend(
                        format!(
                            "#{};2;{};{};{}",
                            colors.len(),
                            (pixel[0] as f32 * color_multiplier) as u32,
                            (pixel[1] as f32 * color_multiplier) as u32,
                            (pixel[2] as f32 * color_multiplier) as u32
                        )
                        .as_bytes(),
                    );
                    colors.insert(pixel, colors.len() as u8);
                }
            }
            for (color, color_index) in &colors {
                let mut sixel_samples = vec![0; sixel_row.width() as usize];
                sixel_samples.resize(sixel_row.width() as usize, 0);
                for (x, y, pixel) in sixel_row.pixels() {
                    if color == &pixel {
                        sixel_samples[x as usize] |= 1 << y;
                    }
                }
                image_data.extend(format!("#{}", color_index).bytes());
                image_data.extend(sixel_samples.iter().map(|x| x + 0x3F));
                image_data.push(b'$');
            }
            image_data.push(b'-');
        }
        image_data.extend(b"\x1B\\");

        image_data.extend(format!("\x1B[{}A", image_rows as u32).as_bytes()); // move cursor to top-left corner
        image_data.extend(format!("\x1B[{}C", image_columns as u32 + 1).as_bytes()); // move cursor to top-right corner of image
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
