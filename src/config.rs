use crate::cli::CliOptions;
use anyhow::{anyhow, Result};
use dirs::config_dir;
use merge::Merge;
use std::{
    fs::{self, create_dir, File},
    io::{BufReader, Read},
    path::Path,
};

pub fn read_cfg<P: AsRef<Path>>(path: &P) -> Result<CliOptions> {
    let file = File::open(&path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(toml::from_str(contents.as_str()).unwrap())
}

pub fn load_cfg<P: AsRef<Path>>(path: Option<&P>) -> Result<CliOptions> {
    match path {
        Some(config_path) => read_cfg(config_path),
        None => {
            let mut default_path = config_dir().unwrap();
            default_path.push("/onefetch/config.toml");
            if default_path.exists() {
                read_cfg(&default_path)
            } else {
                create_dir(&default_path)?;
                write_default(&default_path);
                Err(anyhow!(
                    "Configuration file at {:?} does not exist!",
                    default_path
                ))
            }
        }
    }
}

pub fn write_default<P: AsRef<Path>>(default: &P) {
    let toml = toml::to_string(&CliOptions::default()).expect("Config is not serializable!");
    fs::write(default, toml).expect("Could not write config!")
}

pub fn populate_cfg(cfg: CliOptions) -> CliOptions {
    match load_cfg(cfg.config_path.as_ref()) {
        Ok(mut config) => {
            config.merge(cfg);
            config
        },
        Err(_) => cfg
    }
}
