use anyhow::Result;
use clap::{CommandFactory, FromArgMatches};
use clap_complete::{Generator, generate};
use human_panic::setup_panic;
use onefetch::cli::Cli;
use onefetch::config;
use onefetch::data::manifest::ManifestType;
use onefetch::data::{InfoOptions, build_info};
use onefetch::language::Language;
use onefetch::render::{Printer, RenderOptions};
use std::io;
use strum::IntoEnumIterator;

fn main() -> Result<()> {
    setup_panic!();

    #[cfg(windows)]
    enable_ansi_support::enable_ansi_support()?;

    let matches = Cli::command().get_matches();
    let mut cli = Cli::from_arg_matches(&matches).unwrap_or_else(|e| e.exit());

    if cli.other.languages {
        print_supported_languages();
        return Ok(());
    }

    if cli.other.package_managers {
        print_supported_package_managers();
        return Ok(());
    }

    if let Some(generator) = cli.developer.completion {
        print_completions(generator);
        return Ok(());
    }

    if cli.config.list_config_paths {
        for dir in config::search_dirs() {
            println!("{}", dir.display());
        }
        return Ok(());
    }

    if let Some(config) = config::load(cli.config.config.as_deref())? {
        config.apply(&mut cli, &matches)?;
    }

    if config::generate_if_requested(&cli)? {
        return Ok(());
    }

    let info = build_info(&InfoOptions::from(&cli))?;

    let printer = Printer::new(info, &RenderOptions::from(&cli))?;

    let mut writer = io::BufWriter::new(io::stdout());

    printer.print(&mut writer)?;

    Ok(())
}

fn print_supported_languages() {
    for l in Language::iter() {
        println!("{l}");
    }
}

fn print_supported_package_managers() {
    for p in ManifestType::iter() {
        println!("{p}");
    }
}

fn print_completions<G: Generator>(generator: G) {
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();
    generate(generator, &mut cmd, name, &mut io::stdout());
}
