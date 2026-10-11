//! Turns the parsed command line into the options of each stage: what to
//! collect from the repository (`InfoOptions`) and how to print it
//! (`RenderOptions`). Neither `info` nor `render` depends on `cli`.

use crate::cli::Cli;
use crate::info::InfoOptions;
use crate::render::RenderOptions;

impl From<&Cli> for InfoOptions {
    fn from(cli: &Cli) -> Self {
        Self {
            input: cli.input.clone(),
            disabled_fields: cli.info.disabled_fields.clone(),
            no_title: cli.info.no_title,
            number_of_authors: cli.info.number_of_authors,
            number_of_languages: cli.info.number_of_languages,
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

impl From<&Cli> for RenderOptions {
    fn from(cli: &Cli) -> Self {
        Self {
            output: cli.developer.output.clone(),
            no_art: cli.visuals.no_art,
            image: cli.image.image.clone(),
            image_protocol: cli.image.image_protocol.clone(),
            color_resolution: cli.image.color_resolution,
            ascii_input: cli.ascii.ascii_input.clone(),
            ascii_language: cli.ascii.ascii_language,
            ascii_colors: cli.ascii.ascii_colors.clone(),
            text_colors: cli.text_formatting.text_colors.clone(),
            no_bold: cli.text_formatting.no_bold,
            number_separator: cli.text_formatting.number_separator,
            iso_time: cli.text_formatting.iso_time,
            true_color: cli.true_color(),
            nerd_fonts: cli.visuals.nerd_fonts,
            no_color_palette: cli.visuals.no_color_palette,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::When;

    #[test]
    fn default_render_options_match_cli_defaults() {
        let mut cli = Cli::default();
        cli.ascii.true_color = When::Never;
        assert_eq!(RenderOptions::from(&cli), RenderOptions::default());
    }
}
