use anyhow::{Context as _, Result};
use base64::{engine, Engine};
use image::{imageops::FilterType, DynamicImage};

use rustix::event::{poll, PollFd, PollFlags, Timespec};
use rustix::io::read;
use rustix::termios::{tcgetattr, tcgetwinsize, tcsetattr, LocalModes, OptionalActions};

use std::io::{stdout, Write};
use std::os::fd::AsFd as _;
use std::time::Instant;

pub struct KittyBackend;

impl KittyBackend {
    pub fn supported() -> Result<bool> {
        let stdin = std::io::stdin();
        // save terminal attributes and disable canonical input processing mode
        let old_attributes = {
            let old = tcgetattr(&stdin).context("Failed to recieve terminal attibutes")?;

            let mut new = old.clone();
            new.local_modes &= !LocalModes::ICANON;
            new.local_modes &= !LocalModes::ECHO;
            tcsetattr(&stdin, OptionalActions::Now, &new)
                .context("Failed to update terminal attributes")?;
            old
        };

        // generate red rgba test image
        let mut test_image = Vec::<u8>::with_capacity(32 * 32 * 4);
        test_image.extend(std::iter::repeat_n([255, 0, 0, 255].iter(), 32 * 32).flatten());

        // print the test image with the action set to query
        print!(
            "\x1B_Gi=1,f=32,s=32,v=32,a=q;{}\x1B\\",
            engine::general_purpose::STANDARD.encode(&test_image)
        );
        stdout().flush()?;

        let start_time = Instant::now();
        let stdin_fd = stdin.as_fd();
        let mut stdin_pollfd = [PollFd::new(&stdin_fd, PollFlags::IN)];
        let allowed_bytes = [0x1B, b'_', b'G', b'\\'];
        let mut buf = Vec::<u8>::new();
        loop {
            // check for timeout while polling to avoid blocking the main thread
            while poll(&mut stdin_pollfd, Some(&Timespec::default()))? < 1 {
                if start_time.elapsed().as_millis() > 50 {
                    tcsetattr(&stdin, OptionalActions::Now, &old_attributes)
                        .context("Failed to update terminal attributes")?;
                    return Ok(false);
                }
            }
            let mut byte = [0];
            read(&stdin, &mut byte)?;
            if allowed_bytes.contains(&byte[0]) {
                buf.push(byte[0]);
            }
            if buf.starts_with(&[0x1B, b'_', b'G']) && buf.ends_with(&[0x1B, b'\\']) {
                tcsetattr(&stdin, OptionalActions::Now, &old_attributes)
                    .context("Failed to update terminal attributes")?;
                return Ok(true);
            }
        }
    }
}

impl super::ImageBackend for KittyBackend {
    fn add_image(
        &self,
        lines: Vec<String>,
        image: &DynamicImage,
        _colors: usize,
    ) -> Result<String> {
        let tty_size = tcgetwinsize(std::io::stdin())?;
        let width_ratio = f64::from(tty_size.ws_col) / f64::from(tty_size.ws_xpixel);
        let height_ratio = f64::from(tty_size.ws_row) / f64::from(tty_size.ws_ypixel);

        // resize image to fit the text height with the Lanczos3 algorithm
        let image = image.resize(
            u32::MAX,
            (lines.len() as f64 / height_ratio) as u32,
            FilterType::Lanczos3,
        );
        let _image_columns = width_ratio * f64::from(image.width());
        let image_rows = height_ratio * f64::from(image.height());

        // convert the image to rgba samples
        let rgba_image = image.to_rgba8();
        let flat_samples = rgba_image.as_flat_samples();
        let raw_image = flat_samples
            .image_slice()
            .expect("Conversion from image to rgba samples failed");
        assert_eq!(
            image.width() as usize * image.height() as usize * 4,
            raw_image.len()
        );

        let encoded_image = engine::general_purpose::STANDARD.encode(raw_image); // image data is base64 encoded
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
            image_data.extend(b"\x1B\\");
        }
        image_data.extend(b"\x1B_Gm=0;\x1B\\"); // write empty last chunk
        image_data.extend(format!("\x1B[{}A", image_rows as u32 - 1).as_bytes()); // move cursor to start of image
        let mut i = 0;
        for line in &lines {
            image_data.extend(format!("\x1B[s{line}\x1B[u\x1B[1B").as_bytes());
            i += 1;
        }
        image_data
            .extend(format!("\n\x1B[{}B", lines.len().max(image_rows as usize) - i).as_bytes()); // move cursor to end of image

        Ok(String::from_utf8(image_data)?)
    }
}
