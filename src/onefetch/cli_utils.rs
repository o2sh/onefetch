use crate::onefetch::{deps::package_manager::PackageManager, error::*, language::Language};
use std::env;
use std::process::{Command, Stdio};
use strum::IntoEnumIterator;

pub fn print_supported_languages() -> Result<()> {
    for l in Language::iter() {
        println!("{}", l);
    }

    Ok(())
}

pub fn print_supported_package_managers() -> Result<()> {
    for p in PackageManager::iter() {
        println!("{}", p);
    }

    Ok(())
}

pub fn is_truecolor_terminal() -> bool {
    env::var("COLORTERM")
        .map(|colorterm| colorterm == "truecolor" || colorterm == "24bit")
        .unwrap_or(false)
}

pub fn get_git_version() -> Result<String> {
    let version = Command::new("git").arg("--version").output()?;
    Ok(String::from_utf8_lossy(&version.stdout).replace('\n', ""))
}

pub fn is_git_installed() -> bool {
    Command::new("git").arg("--version").stdout(Stdio::null()).status().is_ok()
}
