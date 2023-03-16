#![cfg_attr(feature = "fail-on-deprecated", deny(deprecated))]

use anyhow::Result;
use clap::{CommandFactory, Parser};
use onefetch::cli;
use onefetch::info::Info;
use onefetch::ui::printer::Printer;
use std::{
    env,
    io::{self, Write},
};

fn main() -> Result<()> {
    setup_panic_hook();

    let mut writer = io::BufWriter::new(io::stdout());

    // Disable line wrapping
    write!(writer, "\x1B[?7l")?;

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

fn setup_panic_hook() {
    let orig_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        eprintln!("\n====================================================================");
        eprintln!("Onefetch has panicked. This is a bug in Onefetch. Please report this");
        eprintln!("at https://github.com/o2sh/onefetch/issues/new.");
        eprintln!();
        eprintln!("Platform: {} {}", env::consts::OS, env::consts::ARCH);
        eprintln!("Args: {:?}", env::args().collect::<Vec<_>>());
        eprintln!();
        print!("\x1B[?7h");
        orig_hook(panic_info);
        std::process::exit(1);
    }));
}
