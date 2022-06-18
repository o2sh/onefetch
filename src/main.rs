#![cfg_attr(feature = "fail-on-deprecated", deny(deprecated))]

use crate::cli::Config;
use anyhow::{bail, Result};
use clap::Parser;
use info::{repo, Info};
use std::io;
use ui::printer::Printer;

mod cli;
mod info;
mod ui;

fn main() -> Result<()> {
    #[cfg(windows)]
    let _ = ansi_term::enable_ansi_support();

    let config = Config::parse();

    if config.languages {
        return cli::print_supported_languages();
    }

    if config.package_managers {
        return cli::print_supported_package_managers();
    }

    /*     if let Some(generator) = config.completion {
        cli::print_completions(generator);
        return Ok(());
    } */

    if !repo::is_valid(&config.input)? {
        bail!("please run onefetch inside of a non-bare git repository");
    }

    let info = Info::new(config)?;

    let mut printer = Printer::new(io::BufWriter::new(io::stdout()), info);

    printer.print()?;

    Ok(())
}
