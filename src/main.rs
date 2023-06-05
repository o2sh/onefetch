#![cfg_attr(feature = "fail-on-deprecated", deny(deprecated))]

use anyhow::Result;
use clap::{CommandFactory, Parser};
use human_panic::setup_panic;
use onefetch::cli::{self, CliOptions};
use onefetch::info::build_info;
use onefetch::ui::printer::Printer;
use std::io;

fn main() -> Result<()> {
    setup_panic!();

    #[cfg(windows)]
    enable_ansi_support::enable_ansi_support()?;

    let cli_options = cli::CliOptions::parse();

    if cli_options.other.languages {
        return cli::print_supported_languages();
    }

    if cli_options.other.package_managers {
        return cli::print_supported_package_managers();
    }

    if let Some(generator) = cli_options.developer.completion {
        let mut cmd = CliOptions::command();
        cli::print_completions(generator, &mut cmd);
        return Ok(());
    }

    let info = build_info(&cli_options)?;

    let mut printer = Printer::new(io::BufWriter::new(io::stdout()), info, cli_options)?;

    printer.print()?;

    Ok(())
}
