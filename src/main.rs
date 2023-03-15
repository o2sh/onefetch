#![cfg_attr(feature = "fail-on-deprecated", deny(deprecated))]

use anyhow::Result;
use clap::{CommandFactory, Parser};
use human_panic::setup_panic;
use onefetch::cli;
use onefetch::info::Info;
use onefetch::ui::printer::Printer;
use std::io::{self, Write};

fn main() -> Result<()> {
    setup_panic!();
    let mut writer = io::BufWriter::new(io::stdout());

    // Disable line wrapping
    write!(writer, "\x1B[?7l")?;

    // Set up panic handler to re-enable line wrapping
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        print!("\x1B[?7h");
        io::stdout().flush().unwrap();
        orig_hook(panic_info);
    }));

    #[cfg(windows)]
    let _ = enable_ansi_support::enable_ansi_support();

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
    let mut printer = Printer::new(info, config)?;

    printer.print(&mut writer)?;

    // Re-enable line wrapping
    write!(writer, "\x1B[?7h")?;

    Ok(())
}
