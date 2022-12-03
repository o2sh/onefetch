#![cfg_attr(feature = "fail-on-deprecated", deny(deprecated))]

use anyhow::Result;
use clap::{CommandFactory, Parser};
use human_panic::setup_panic;
use onefetch::cli;
use onefetch::info::Info;
use onefetch::ui::printer::Printer;
use std::io;

fn main() -> Result<()> {
    #[cfg(windows)]
    let _ = enable_ansi_support::enable_ansi_support();

    setup_panic!();

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

    Ok(())
}
