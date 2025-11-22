#![cfg_attr(feature = "fail-on-deprecated", deny(deprecated))]

use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_i18n_richformatter::{init_clap_rich_formatter_localizer, ClapI18nRichFormatter};
use human_panic::setup_panic;
use onefetch::cli::{self, CliOptions};
use onefetch::info::build_info;
use onefetch::ui::printer::factory::PrinterFactory;
use std::io;

fn main() -> Result<()> {
    setup_panic!();
    onefetch::i18n::init()?;
    init_clap_rich_formatter_localizer();

    #[cfg(windows)]
    enable_ansi_support::enable_ansi_support()?;

    let cli_options = cli::CliOptions::try_parse().map_err(|e| {
        let e = e.apply::<ClapI18nRichFormatter>();
        e.exit()
    }).unwrap();

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

    let printer = PrinterFactory::new(info, cli_options)?.create()?;

    let mut writer = io::BufWriter::new(io::stdout());

    printer.print(&mut writer)?;

    Ok(())
}
