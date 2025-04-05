use anyhow::{anyhow, Result};
use dirs::config_dir;
use num_format::CustomFormat;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{BufReader, Read},
    path::Path,
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Configuration {
    pub separator: Option<String>,
    pub number_separator: Option<NumberSeparator>,
    pub nerd_fonts: Option<bool>,
    pub percent_verbosity: Option<usize>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            separator: Some(":".to_string()),
            number_separator: Some(NumberSeparator::Plain),
            nerd_fonts: Some(false),
            percent_verbosity: Some(1),
        }
    }
}

pub fn read_cfg<P: AsRef<Path>>(path: &P) -> Result<Configuration> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    Ok(toml::from_str(content.as_str()).unwrap())
}

pub fn load_cfg<P: AsRef<Path>>(path: Option<&P>) -> Result<Configuration> {
    match path {
        Some(config_path) => read_cfg(config_path),
        None => {
            let mut default_path = config_dir().unwrap();
            default_path.push("onefetch");
            if default_path.exists() {
                default_path.push("config.toml");
                if default_path.exists() {
                    read_cfg(&default_path)
                } else {
                    write_default_cfg(&default_path);
                    Err(anyhow!("Wrote config at {:?}", default_path))
                }
            } else {
                fs::create_dir_all(&default_path).expect("An error occured");
                default_path.push("config.toml");
                write_default_cfg(&default_path);
                Err(anyhow!("Wrote config at {:?}", default_path))
            }
        }
    }
}

pub fn write_default_cfg<P: AsRef<Path>>(default_path: &P) {
    let config = toml::to_string(&Configuration::default()).expect("Config should be serializable");
    fs::write(default_path, config).expect("Could not write config!");
}

#[derive(clap::ValueEnum, Clone, PartialEq, Eq, Debug, Deserialize, Default, Copy, Serialize)]
pub enum NumberSeparator {
    #[default]
    Plain,
    Comma,
    Space,
    Underscore,
    Dot,
}

impl NumberSeparator {
    fn separator(&self) -> &'static str {
        match self {
            Self::Plain => "",
            Self::Comma => ",",
            Self::Space => "\u{202f}",
            Self::Underscore => "_",
            Self::Dot => ".",
        }
    }

    pub fn get_format(&self) -> CustomFormat {
        num_format::CustomFormat::builder()
            .grouping(num_format::Grouping::Standard)
            .separator(self.separator())
            .build()
            .unwrap()
    }
}
