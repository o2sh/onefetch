#![cfg_attr(feature = "fail-on-deprecated", deny(deprecated))]

use anyhow::Result;
use clap::{CommandFactory, Parser};
use cli::Config;
use info::Info;
use std::io;
use ui::printer::Printer;

fn main() -> Result<()> {
    #[cfg(windows)]
    let _ = enable_ansi_support::enable_ansi_support();

    let config = Config::parse();

    if config.languages {
        return cli::print_supported_languages();
    }

    if config.package_managers {
        return cli::print_supported_package_managers();
    }

    if let Some(generator) = config.completion {
        let mut cmd = Config::command();
        cli::print_completions(generator, &mut cmd);
        return Ok(());
    }

    let info = Info::new(&config)?;
    let mut printer = Printer::new(io::BufWriter::new(io::stdout()), info, config)?;

    printer.print()?;

    Ok(())
}

mod cli;
mod info;
mod ui;
