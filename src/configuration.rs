use anyhow::{anyhow, Result};
use crate::cli::Config;
use dirs::config_dir;
use merge::Merge;
use std::fs;
use std::{fs::File, path::Path};
use std::io::{BufReader, Read};

fn read_config_file<P: AsRef<Path>>(path: &P) -> Result<Config> {
    let f = File::open(&path)?;
    let mut buf_reader = BufReader::new(f);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(toml::from_str(contents.as_str()).unwrap())
}

fn load_config<P: AsRef<Path>>(path: Option<&P>) -> Result<Config> {
    match path {
        Some(config_path) => read_config_file(config_path),
        None => {
            let mut default_path = config_dir().unwrap();
            default_path.push("onefetch.toml");
            println!("Default config file path: {}", &default_path.display());
            if default_path.exists() {
                read_config_file(&default_path)
            } else {
                write_default_config(&default_path);
                Err(anyhow!("Default config file did not exist: {:?}", default_path))
            }
        } 
    }
}

fn write_default_config<P: AsRef<Path>>(default_path: &P) {
    let toml = toml::to_string(&Config::default()).expect("Config should be serializable");
    fs::write(default_path, toml).expect("Should be able to write to config dir");
}

pub fn populate_config(cli_config: Config) -> Config {
    match load_config(cli_config.config_path.as_ref()) {
        Ok(mut disk_config) => {
            disk_config.merge(cli_config);
            disk_config
        },
        Err(_) => cli_config
    }
}
