//! The config file: a JSONC object whose keys are long command-line options,
//! such as `{"number-of-authors": 5, "no-merges": true}`.
//!
//! The config is read on its own, apart from the command line; `options`
//! merges the two.

use crate::data::{ACTIVITY_MONTHS, FieldKind};
use crate::language::{Language, LanguageType};
use crate::render::format::NumberSeparator;
use crate::render::image::ImageProtocol;
use crate::render::{COLOR_RESOLUTIONS, SerializationFormat, When};
use anyhow::{Context, Result, ensure};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::{env, fs};

/// The options of the config file. A missing key takes its value from
/// `Config::default()`, which holds the defaults of the command line.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case", default, deny_unknown_fields)]
pub struct Config {
    // INFO
    pub disabled_fields: Vec<FieldKind>,
    pub no_title: bool,
    pub number_of_authors: usize,
    pub number_of_languages: usize,
    pub number_of_file_churns: usize,
    pub churn_pool_size: Option<usize>,
    pub activity_months: usize,
    pub exclude: Vec<String>,
    pub ignore_author: Option<String>,
    pub no_merges: bool,
    pub email: bool,
    pub http_url: bool,
    pub hide_token: bool,
    pub include_hidden: bool,
    pub language_types: Vec<LanguageType>,
    // TEXT FORMATTING
    pub text_colors: Vec<u8>,
    pub iso_time: bool,
    pub number_separator: NumberSeparator,
    pub no_bold: bool,
    // ASCII
    pub ascii_input: Option<String>,
    pub ascii_colors: Vec<u8>,
    pub ascii_language: Option<Language>,
    pub true_color: When,
    // IMAGE
    pub image: Option<PathBuf>,
    pub image_protocol: Option<ImageProtocol>,
    pub color_resolution: usize,
    // VISUALS
    pub no_color_palette: bool,
    pub no_art: bool,
    pub nerd_fonts: bool,
    // DEVELOPER
    pub output: Option<SerializationFormat>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            disabled_fields: Vec::new(),
            no_title: false,
            number_of_authors: 3,
            number_of_languages: 6,
            number_of_file_churns: 3,
            churn_pool_size: None,
            activity_months: 12,
            exclude: Vec::new(),
            ignore_author: None,
            no_merges: false,
            email: false,
            http_url: false,
            hide_token: false,
            include_hidden: false,
            language_types: vec![LanguageType::Programming, LanguageType::Markup],
            text_colors: Vec::new(),
            iso_time: false,
            number_separator: NumberSeparator::Plain,
            no_bold: false,
            ascii_input: None,
            ascii_colors: Vec::new(),
            ascii_language: None,
            true_color: When::Auto,
            image: None,
            image_protocol: None,
            color_resolution: 64,
            no_color_palette: false,
            no_art: false,
            nerd_fonts: false,
            output: None,
        }
    }
}

impl Config {
    /// Reads `path`, or the default config file if there is one.
    pub fn load(path: Option<&Path>) -> Result<Self> {
        let Some(path) = path
            .map(Path::to_path_buf)
            .or_else(|| default_path().filter(|path| path.is_file()))
        else {
            return Ok(Self::default());
        };
        let text = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read the config file `{}`", path.display()))?;
        Self::parse(&text).with_context(|| format!("Invalid config file `{}`", path.display()))
    }

    fn parse(text: &str) -> Result<Self> {
        let mut text = text.to_owned();
        json_strip_comments::strip(&mut text)?;
        if text.trim().is_empty() {
            return Ok(Self::default());
        }
        ensure!(
            text.trim_start().starts_with('{'),
            "The config file must be a JSON object"
        );
        let config: Self = serde_json::from_str(&text)?;
        config.validate()?;
        Ok(config)
    }

    /// The checks clap runs on the same values on the command line.
    fn validate(&self) -> Result<()> {
        ensure!(
            ACTIVITY_MONTHS
                .iter()
                .any(|months| months.parse() == Ok(self.activity_months)),
            "`activity-months` must be one of {}",
            ACTIVITY_MONTHS.join(", ")
        );
        ensure!(
            COLOR_RESOLUTIONS
                .iter()
                .any(|resolution| resolution.parse() == Ok(self.color_resolution)),
            "`color-resolution` must be one of {}",
            COLOR_RESOLUTIONS.join(", ")
        );
        ensure!(
            self.ascii_colors.iter().all(|&color| color < 16),
            "`ascii-colors` must be below 16"
        );
        ensure!(
            self.text_colors.iter().all(|&color| color < 16),
            "`text-colors` must be below 16"
        );
        ensure!(
            self.text_colors.len() <= 6,
            "`text-colors` takes at most 6 colors"
        );
        ensure!(
            !self.language_types.is_empty(),
            "`language-types` takes at least one type"
        );
        Ok(())
    }
}

/// `$XDG_CONFIG_HOME/onefetch/config.jsonc`, or `~/.config/onefetch/config.jsonc`.
fn default_path() -> Option<PathBuf> {
    // The XDG spec says to ignore an empty or relative path.
    let dir = match env::var_os("XDG_CONFIG_HOME").map(PathBuf::from) {
        Some(dir) if dir.is_absolute() => dir,
        _ => env::home_dir()?.join(".config"),
    };
    Some(dir.join("onefetch/config.jsonc"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_options() -> Result<()> {
        let config = Config::parse(
            r#"{
                // a comment
                "number-of-authors": 5,
                "email": false,
                "image": null,
                "ignore-author": "^bors$",
                "disabled-fields": ["churn", "last-change"],
                "ascii-language": "c++",
            }"#,
        )?;
        assert_eq!(config.number_of_authors, 5);
        assert!(!config.email);
        assert_eq!(config.image, None);
        assert_eq!(config.ignore_author.as_deref(), Some("^bors$"));
        assert_eq!(
            config.disabled_fields,
            [FieldKind::Churn, FieldKind::LastChange]
        );
        assert_eq!(config.ascii_language, Some(Language::Cpp));
        assert_eq!(Config::parse("// nothing yet\n")?.number_of_authors, 3);
        Ok(())
    }

    #[test]
    fn rejects_bad_options() {
        for text in [
            r#"{"nope": 1}"#,
            r#"{"languages": true}"#,
            r#"{"number-of-authors": "many"}"#,
            r#"{"no-merges": 1}"#,
            r#"{"activity-months": 5}"#,
            r#"{"color-resolution": 10}"#,
            r#"{"ascii-colors": [16]}"#,
            r#"{"text-colors": [16]}"#,
            r#"{"text-colors": [1, 2, 3, 4, 5, 6, 7]}"#,
            r#"{"language-types": []}"#,
            "[]",
        ] {
            assert!(Config::parse(text).is_err(), "accepted {text}");
        }
    }

    #[test]
    fn presets_are_valid() -> Result<()> {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("presets");
        for entry in fs::read_dir(dir)? {
            Config::load(Some(&entry?.path()))?;
        }
        Ok(())
    }
}
