use anyhow::{anyhow, Result};
use dirs::config_dir;
use merge::Merge;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{BufReader, Read},
    path::Path,
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Merge)]
pub struct Configuration {
    #[merge(skip)]
    pub separator: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            separator: ":".to_string(),
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
