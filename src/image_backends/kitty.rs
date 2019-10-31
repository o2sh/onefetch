use image::{imageops::FilterType, DynamicImage, GenericImageView};
use libc::{
    ioctl, tcgetattr, tcsetattr, termios, winsize, ECHO, ICANON, STDIN_FILENO, STDOUT_FILENO,
    TCSANOW, TIOCGWINSZ,
};
use std::io::Read;
use std::sync::mpsc::{self, TryRecvError};
use std::time::Duration;

pub struct KittyBackend {}

impl KittyBackend {
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

        // generate red rgba test image
        let mut test_image = Vec::<u8>::with_capacity(32 * 32 * 4);
        test_image.extend(
            std::iter::repeat([255, 0, 0, 255].iter())
                .take(32 * 32)
                .flatten(),
        );

        // print the test image with the action set to query
        println!(
            "\x1B_Gi=1,f=32,s=32,v=32,a=q;{}\x1B\\",
            base64::encode(&test_image)
        );

        // a new thread is required to avoid blocking the main thread if the terminal doesn't respond
        let (sender, receiver) = mpsc::channel::<()>();
        let (stop_sender, stop_receiver) = mpsc::channel::<()>();
        std::thread::spawn(move || {
            let mut buf = Vec::<u8>::new();
            let allowed_bytes = [0x1B, b'_', b'G', b'\\'];
            for byte in std::io::stdin().lock().bytes() {
                let byte = byte.unwrap();
                if allowed_bytes.contains(&byte) {
                    buf.push(byte);
                }
                if buf.starts_with(&[0x1B, b'_', b'G']) && buf.ends_with(&[0x1B, b'\\']) {
                    sender.send(()).unwrap();
                    return;
                }
                match stop_receiver.try_recv() {
                    Err(TryRecvError::Empty) => {}
                    _ => return,
                }
            }
        });
        if receiver.recv_timeout(Duration::from_millis(50)).is_ok() {
            unsafe {
                tcsetattr(STDIN_FILENO, TCSANOW, &old_attributes);
            }
            true
        } else {
            unsafe {
                tcsetattr(STDIN_FILENO, TCSANOW, &old_attributes);
            }
            stop_sender.send(()).ok();
            false
        }
    }
}

impl super::ImageBackend for KittyBackend {
    fn add_image(&self, lines: Vec<String>, image: &DynamicImage) -> String {
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

        // convert the image to rgba samples
        let rgba_image = image.to_rgba();
        let flat_samples = rgba_image.as_flat_samples();
        let raw_image = flat_samples
            .image_slice()
            .expect("Conversion from image to rgba samples failed");
        assert_eq!(
            image.width() as usize * image.height() as usize * 4,
            raw_image.len()
        );

        let encoded_image = base64::encode(&raw_image); // image data is base64 encoded
        let mut image_data = Vec::<u8>::new();
        for chunk in encoded_image.as_bytes().chunks(4096) {
            // send a 4096 byte chunk of base64 encoded rgba image data
            image_data.extend(
                format!(
                    "\x1B_Gf=32,s={},v={},m=1,a=T;",
                    image.width(),
                    image.height()
                )
                .as_bytes(),
            );
            image_data.extend(chunk);
            image_data.extend("\x1B\\".as_bytes());
        }
        image_data.extend("\x1B_Gm=0;\x1B\\".as_bytes()); // write empty last chunk
        image_data.extend(format!("\x1B[{}A", image_rows as u32 - 1).as_bytes()); // move cursor to start of image
        let mut i = 0;
        for line in &lines {
            image_data.extend(format!("\x1B[s{}\x1B[u\x1B[1B", line).as_bytes());
            i += 1;
        }
        image_data
            .extend(format!("\n\x1B[{}B", lines.len().max(image_rows as usize) - i).as_bytes()); // move cursor to end of image

        String::from_utf8(image_data).unwrap()
    }
}
