#![cfg_attr(feature = "fail-on-deprecated", deny(deprecated))]

use anyhow::{bail, Result};
use clap::{CommandFactory, Parser};
use onefetch::cli;
use onefetch::cli::Config;
use onefetch::info::{repo, Info};
use onefetch::ui::printer::Printer;
use std::io;

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

    if let Some(generator) = config.completion {
        let mut cmd = Config::command();
        cli::print_completions(generator, &mut cmd);
        return Ok(());
    }

    if !repo::is_valid(&config.input)? {
        bail!("please run onefetch inside of a non-bare git repository");
    }

    let info = Info::new(&config)?;

    let mut printer = Printer::new(io::BufWriter::new(io::stdout()), info, config)?;

    printer.print()?;

    Ok(())
}
