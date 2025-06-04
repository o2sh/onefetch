use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

use crate::{
    cli::{NumberSeparator, When},
    info::utils::info_field::InfoType,
};

#[derive(Serialize, Deserialize)]
pub struct ConfigOptions {
    // THIS IS JUST A RAW, REALLY WIP CONFIG STRUCTURE
    #[serde(default)]
    pub disabled_fields: Vec<InfoType>,
    // Lol is this really will turn into comment?
    /// Or maybe this?
    #[serde(default)]
    pub no_title: bool,
    #[serde(default)]
    pub number_of_authors: usize,
    #[serde(default)]
    pub number_of_languages: usize,
    #[serde(default)]
    pub number_of_file_churns: usize,
    #[serde(default)]
    pub no_merges: bool,
    #[serde(default)]
    pub include_hidden: bool,
    #[serde(default)]
    pub iso_time: bool,
    #[serde(default)]
    pub number_separator: NumberSeparator,
    #[serde(default)]
    pub no_bold: bool,
    #[serde(default)]
    pub true_color: When,
    #[serde(default)]
    pub nerd_fonts: bool,
}

impl Default for ConfigOptions {
    fn default() -> Self {
        Self {
            disabled_fields: vec![],
            no_title: false,
            number_of_authors: 3usize,
            number_of_languages: 6usize,
            number_of_file_churns: 3usize,
            no_merges: false,
            include_hidden: false,
            iso_time: false,
            number_separator: NumberSeparator::default(),
            no_bold: false,
            true_color: When::Auto,
            nerd_fonts: false,
        }
    }
}

impl ConfigOptions {
    pub fn read<P>(path: P) -> Self
    // Is it even should panic?
    where
        P: AsRef<Path>,
    {
        let contents = fs::read_to_string(path);
        match contents {
            Err(_) => Self::default(),
            Ok(contents) => {
                // I wish to print exact error here, like syntax errors
                toml::from_str(&contents).unwrap()
            }
        }
    }

    pub fn write<P>(self, path: P) -> Result<()>
    // I believe user would like to generate config with CLI flags
    // I mean to write disabled_fields with --disabled-fields flag
    where
        P: AsRef<Path>,
    {
        // I dont think this can panic so i simply unwrapped it
        let contents = toml::to_string(&self).unwrap();
        let dir = path
            .as_ref()
            .parent()
            .with_context(|| format!("'{}' is not a path!", path.as_ref().display()));
        fs::create_dir_all(dir?)?;
        match fs::write(&path, contents) {
            Ok(_) => println!("Created default config at {}", path.as_ref().display()),
            Err(e) => eprintln!(
                "Failed to write default config at {}: {}",
                path.as_ref().display(),
                e
            ),
        }
        Ok(())
    }
}
