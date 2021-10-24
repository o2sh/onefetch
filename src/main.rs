#![cfg_attr(feature = "fail-on-deprecated", deny(deprecated))]

use anyhow::{bail, Result};
use cli::Config;
use info::{repo, Info};
use std::io;
use ui::printer::Printer;

mod cli;
mod info;
mod ui;

fn main() -> Result<()> {
	#[cfg(windows)]
	let _ = ansi_term::enable_ansi_support();

	let config = Config::new()?;

	if config.print_languages {
		return cli::print_supported_languages();
	}

	if config.print_package_managers {
		return cli::print_supported_package_managers();
	}

	if !repo::is_valid(&config.repo_path)? {
		bail!("please run onefetch inside of a non-bare git repository");
	}

	let info = Info::new(config)?;

	let mut printer = Printer::new(io::BufWriter::new(io::stdout()), info);

	printer.print()?;

	Ok(())
}
