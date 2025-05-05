use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

use crate::info::utils::info_field::InfoType;

#[derive(Serialize, Deserialize)]
pub struct ConfigOptions {
    #[serde(default)]
    pub disabled_fields: Vec<InfoType>,
}

impl Default for ConfigOptions {
    fn default() -> Self {
        Self {
            disabled_fields: vec![],
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
        match fs::create_dir_all(&path.as_ref().parent().unwrap_or(Path::new("/"))) {
            Ok(_) => match fs::write(&path, &contents) {
                Ok(_) => {
                    let path = path.as_ref().display();
                    println!("Config config file created at: {path}")
                }
                Err(e) => {
                    let path = path.as_ref().display();
                    eprintln!("Failed to write config file at {path}: {e}")
                }
            },
            Err(e) => {
                let path = path.as_ref().display();
                eprintln!("Failed to create config directory {path}: {e}");
            }
        }
        // Im not sure it should return simple Ok(())?
        Ok(())
    }
}
