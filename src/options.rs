//! Turns the parsed command line into the options of each stage: what to
//! collect from the repository and how to display it. This is the only place
//! where `cli` meets `info`.

use crate::cli::Cli;
use crate::info::InfoOptions;
use crate::render::format::DisplayOptions;

impl From<&Cli> for InfoOptions {
    fn from(cli: &Cli) -> Self {
        Self {
            input: cli.input.clone(),
            disabled_fields: cli.info.disabled_fields.clone(),
            no_title: cli.info.no_title,
            number_of_authors: cli.info.number_of_authors,
            number_of_file_churns: cli.info.number_of_file_churns,
            churn_pool_size: cli.info.churn_pool_size,
            exclude: cli.info.exclude.clone(),
            no_bots: cli.info.no_bots.clone(),
            no_merges: cli.info.no_merges,
            email: cli.info.email,
            http_url: cli.info.http_url,
            hide_token: cli.info.hide_token,
            include_hidden: cli.info.include_hidden,
            language_types: cli.info.language_types.clone(),
        }
    }
}

impl From<&Cli> for DisplayOptions {
    fn from(cli: &Cli) -> Self {
        Self {
            number_separator: cli.text_formatting.number_separator,
            iso_time: cli.text_formatting.iso_time,
            true_color: cli.true_color(),
            nerd_fonts: cli.visuals.nerd_fonts,
            no_color_palette: cli.visuals.no_color_palette,
            number_of_languages: cli.info.number_of_languages,
            number_of_authors: cli.info.number_of_authors,
        }
    }
}
