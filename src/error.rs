use colored::Colorize;
use error_chain::error_chain;
use std::io::Write;

error_chain! {
    foreign_links {
        Clap(::clap::Error) #[cfg(feature = "application")];
        Io(::std::io::Error);
        ParseIntError(::std::num::ParseIntError);
    }
}

pub fn default_error_handler(error: &Error, output: &mut dyn Write) {
    writeln!(output, "{}: {}", "[onefetch error]".red(), error).ok();
}
