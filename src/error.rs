use colored::Colorize;
use error_chain::error_chain;
use std::io::Write;

error_chain! {
    foreign_links {
        Clap(::clap::Error) #[cfg(feature = "application")];
        Io(::std::io::Error);
        ParseIntError(::std::num::ParseIntError);
        Image(image::error::ImageError);
        Utf8(std::string::FromUtf8Error);
        Json(json::Error);
        Regex(regex::Error);
        Toml(toml::de::Error);
        Git2(git2::Error);
        Strum(strum::ParseError);
    }
}

pub fn default_error_handler(e: &Error, output: &mut dyn Write) {
    writeln!(output, "{}: {}", "[onefetch error]".red(), e).ok();

    for e in e.iter().skip(1) {
        writeln!(output, "caused by: {}", e).ok();
    }
}
