use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::{Generator, generate};
use human_panic::setup_panic;
use onefetch::cli::Cli;
use onefetch::config::Config;
use onefetch::data::build_info;
use onefetch::data::manifest::ManifestType;
use onefetch::language::Language;
use onefetch::options;
use onefetch::render::Printer;
use std::io;
use strum::IntoEnumIterator;

fn main() -> Result<()> {
    setup_panic!();

    #[cfg(windows)]
    enable_ansi_support::enable_ansi_support()?;

    let cli = Cli::parse();

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

    let config = Config::load(cli.other.config.as_deref())?;

    let (info_options, render_options) = options::new(cli, config)?;

    let info = build_info(&info_options)?;

    let printer = Printer::new(info, &render_options)?;

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
