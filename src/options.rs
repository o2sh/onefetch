use crate::cli::Cli;
use crate::config::Config;
use crate::data::InfoOptions;
use crate::language::LanguageType;
use crate::render::format::NumberSeparator;
use crate::render::{RenderOptions, When};
use anyhow::{Context, Result};
use regex::Regex;
use std::env;

const NUMBER_OF_AUTHORS: usize = 3;
const NUMBER_OF_LANGUAGES: usize = 6;
const NUMBER_OF_FILE_CHURNS: usize = 3;
const ACTIVITY_MONTHS: usize = 12;
const LANGUAGE_TYPES: &[LanguageType] = &[LanguageType::Programming, LanguageType::Markup];
const COLOR_RESOLUTION: usize = 64;
const NUMBER_SEPARATOR: NumberSeparator = NumberSeparator::Plain;
const TRUE_COLOR: When = When::Auto;

pub fn new(cli: Cli, config: Config) -> Result<(InfoOptions, RenderOptions)> {
    let Cli {
        input,
        info,
        text_formatting: text,
        ascii,
        image,
        visuals,
        developer,
        ..
    } = cli;
    let ignore_author = match info.ignore_author {
        Some(regex) => Some(regex),
        None => config
            .ignore_author
            .as_deref()
            .map(Regex::new)
            .transpose()
            .context("Invalid `ignore-author` in the config file")?,
    };
    let info_options = InfoOptions {
        input,
        disabled_fields: info.disabled_fields.unwrap_or(config.disabled_fields),
        no_title: info.no_title || config.no_title,
        number_of_authors: info
            .number_of_authors
            .or(config.number_of_authors)
            .unwrap_or(NUMBER_OF_AUTHORS),
        number_of_languages: info
            .number_of_languages
            .or(config.number_of_languages)
            .unwrap_or(NUMBER_OF_LANGUAGES),
        number_of_file_churns: info
            .number_of_file_churns
            .or(config.number_of_file_churns)
            .unwrap_or(NUMBER_OF_FILE_CHURNS),
        churn_pool_size: info.churn_pool_size.or(config.churn_pool_size),
        activity_months: info
            .activity_months
            .or(config.activity_months)
            .unwrap_or(ACTIVITY_MONTHS),
        exclude: info.exclude.unwrap_or(config.exclude),
        ignore_author,
        no_merges: info.no_merges || config.no_merges,
        email: info.email || config.email,
        http_url: info.http_url || config.http_url,
        hide_token: info.hide_token || config.hide_token,
        include_hidden: info.include_hidden || config.include_hidden,
        language_types: info
            .language_types
            .or(config.language_types)
            .unwrap_or_else(|| LANGUAGE_TYPES.to_vec()),
    };
    let render_options = RenderOptions {
        output: developer.output.or(config.output),
        no_art: visuals.no_art || config.no_art,
        image: image.image.or(config.image),
        image_protocol: image.image_protocol.or(config.image_protocol),
        color_resolution: image
            .color_resolution
            .or(config.color_resolution)
            .unwrap_or(COLOR_RESOLUTION),
        ascii_input: ascii.ascii_input.or(config.ascii_input),
        ascii_language: ascii.ascii_language.or(config.ascii_language),
        ascii_colors: ascii.ascii_colors.unwrap_or(config.ascii_colors),
        text_colors: text.text_colors.unwrap_or(config.text_colors),
        no_bold: text.no_bold || config.no_bold,
        number_separator: text
            .number_separator
            .or(config.number_separator)
            .unwrap_or(NUMBER_SEPARATOR),
        iso_time: text.iso_time || config.iso_time,
        true_color: match ascii.true_color.or(config.true_color).unwrap_or(TRUE_COLOR) {
            When::Always => true,
            When::Never => false,
            When::Auto => is_truecolor_terminal(),
        },
        nerd_fonts: visuals.nerd_fonts || config.nerd_fonts,
        no_color_palette: visuals.no_color_palette || config.no_color_palette,
    };
    Ok((info_options, render_options))
}

fn is_truecolor_terminal() -> bool {
    env::var("COLORTERM")
        .map(|colorterm| colorterm == "truecolor" || colorterm == "24bit")
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language::Language;
    use clap::Parser;

    fn options(config: &str, args: &[&str]) -> Result<(InfoOptions, RenderOptions)> {
        let cli = Cli::try_parse_from(["onefetch"].iter().chain(args))?;
        new(cli, serde_json::from_str(config)?)
    }

    #[test]
    fn default_render_options_match_config_defaults() -> Result<()> {
        let (_, render) = options("{}", &["--true-color", "never"])?;
        assert_eq!(render, RenderOptions::default());
        Ok(())
    }

    #[test]
    fn config_replaces_the_defaults() -> Result<()> {
        let (info, render) = options(
            r#"{"number-of-authors": 5, "language-types": ["prose"], "ascii-language": "c++", "no-merges": true, "exclude": ["a"], "text-colors": [1, 2]}"#,
            &["--email"],
        )?;
        assert_eq!(info.number_of_authors, 5);
        assert_eq!(info.language_types, [LanguageType::Prose]);
        assert!(info.no_merges);
        assert!(info.email);
        assert_eq!(info.exclude, ["a"]);
        assert_eq!(render.text_colors, [1, 2]);
        assert_eq!(render.ascii_language, Some(Language::Cpp));
        Ok(())
    }

    #[test]
    fn command_line_wins() -> Result<()> {
        let (info, render) = options(
            r#"{"number-of-authors": 5, "exclude": ["a"], "text-colors": [1, 2], "true-color": "always"}"#,
            &[
                "--number-of-authors",
                "3",
                "-e",
                "b",
                "-t",
                "3",
                "--true-color",
                "never",
            ],
        )?;
        assert_eq!(info.number_of_authors, 3);
        assert_eq!(info.exclude, ["b"]);
        assert_eq!(render.text_colors, [3]);
        assert!(!render.true_color);
        Ok(())
    }

    #[test]
    fn rejects_invalid_values_in_the_config() {
        for config in [
            r#"{"ignore-author": "("}"#,
            r#"{"number-separator": "dot"}"#,
            r#"{"language-types": ["prose", "nope"]}"#,
        ] {
            assert!(options(config, &[]).is_err(), "accepted {config}");
        }
    }
}
