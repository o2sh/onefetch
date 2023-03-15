#![cfg_attr(feature = "fail-on-deprecated", deny(deprecated))]

use anyhow::Result;
use clap::{CommandFactory, Parser};
use onefetch::cli;
use onefetch::info::Info;
use onefetch::ui::printer::Printer;
use std::{
    io::{self, stdout, Write},
    panic,
};

fn main() -> Result<()> {
    // Disable line wrapping
    print!("\x1B[?7l");
    stdout().flush().unwrap();
    #[cfg(windows)]
    let _ = enable_ansi_support::enable_ansi_support();

    // Set up panic handler to re-enable line wrapping
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        print!("\x1B[?7h");
        stdout().flush().unwrap();
        orig_hook(panic_info);
    }));

    let config = cli::Config::parse();

    if config.languages {
        return cli::print_supported_languages();
    }

    if config.package_managers {
        return cli::print_supported_package_managers();
    }

    if let Some(generator) = config.completion {
        let mut cmd = cli::Config::command();
        cli::print_completions(generator, &mut cmd);
        return Ok(());
    }

    let info = Info::new(&config)?;
    let mut printer = Printer::new(io::BufWriter::new(io::stdout()), info, config)?;

    printer.print()?;
    // Re-enable line wrapping
    print!("\x1B[?7h");
    stdout().flush().unwrap();

    Ok(())
}
