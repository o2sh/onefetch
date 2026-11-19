//! The config file: a JSONC object whose keys are long command-line options,
//! such as `{"number-of-authors": 5, "no-merges": true}`.

use crate::cli::Cli;
use anyhow::{Context, Result, bail};
use clap::{CommandFactory, Parser};
use serde_json::{Map, Value};
use std::ffi::OsString;
use std::path::PathBuf;
use std::{env, fs};

/// Parses the command line with the options of the config file in front, so
/// that the same option on the command line comes later and wins.
pub fn parse_args(mut args: Vec<OsString>) -> Result<Cli> {
    let explicit = Cli::parse_from(&args).other.config;
    if let Some(path) = explicit.or_else(|| default_path().filter(|path| path.is_file())) {
        let text = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read the config file `{}`", path.display()))?;
        args.splice(1..1, config_args(&text)?);
    }
    Ok(Cli::parse_from(args))
}

/// `$XDG_CONFIG_HOME/onefetch/config.jsonc`, or `~/.config/onefetch/config.jsonc`.
fn default_path() -> Option<PathBuf> {
    let dir = match env::var_os("XDG_CONFIG_HOME") {
        Some(dir) => PathBuf::from(dir),
        None => env::home_dir()?.join(".config"),
    };
    Some(dir.join("onefetch/config.jsonc"))
}

fn config_args(text: &str) -> Result<Vec<OsString>> {
    let mut text = text.to_owned();
    json_strip_comments::strip(&mut text)?;
    let config: Map<String, Value> = serde_json::from_str(&text)?;
    // Checked here because `false` and `[]` give no argument for clap to check.
    let command = Cli::command();
    if let Some(key) = config.keys().find(|key| {
        !command
            .get_arguments()
            .any(|arg| arg.get_long() == Some(key))
    }) {
        bail!("Unknown option `{key}`");
    }
    Ok(config
        .iter()
        .flat_map(|(key, value)| to_args(key, value))
        .collect())
}

/// `"no-merges": true` is `--no-merges`, `"number-of-authors": 5` is
/// `--number-of-authors=5`, and `"exclude": ["a", "b"]` is
/// `--exclude=a --exclude=b`.
fn to_args(key: &str, value: &Value) -> Vec<OsString> {
    match value {
        Value::Bool(true) => vec![format!("--{key}").into()],
        Value::Bool(false) => vec![],
        Value::Array(items) => items.iter().flat_map(|item| to_args(key, item)).collect(),
        Value::String(text) => vec![format!("--{key}={text}").into()],
        value => vec![format!("--{key}={value}").into()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::FieldKind;

    fn cli_with(config: &str, args: &[&str]) -> Result<Cli> {
        let mut all = vec![OsString::from("onefetch")];
        all.extend(config_args(config)?);
        all.extend(args.iter().map(OsString::from));
        Ok(Cli::try_parse_from(all)?)
    }

    #[test]
    fn sets_options() -> Result<()> {
        let cli = cli_with(
            r#"{
                // a comment
                "number-of-authors": 5,
                "no-merges": true,
                "email": false,
                "exclude": ["-x.lock", "vendor/"],
                "disabled-fields": ["churn", "url"],
            }"#,
            &[],
        )?;
        assert_eq!(cli.info.number_of_authors, 5);
        assert!(cli.info.no_merges);
        assert!(!cli.info.email);
        assert_eq!(cli.info.exclude, ["-x.lock", "vendor/"]);
        assert_eq!(cli.info.disabled_fields, [FieldKind::Churn, FieldKind::Url]);
        Ok(())
    }

    #[test]
    fn command_line_wins() -> Result<()> {
        let cli = cli_with(
            r#"{"number-of-authors": 5, "no-merges": true, "exclude": ["a"]}"#,
            &["--number-of-authors", "2", "--no-merges", "repo"],
        )?;
        assert_eq!(cli.info.number_of_authors, 2);
        assert!(cli.info.no_merges);
        assert_eq!(cli.input, PathBuf::from("repo"));
        Ok(())
    }

    #[test]
    fn rejects_bad_options() {
        assert!(cli_with(r#"{"nope": 1}"#, &[]).is_err());
        assert!(cli_with(r#"{"nerd-fots": false}"#, &[]).is_err());
        assert!(cli_with(r#"{"exclud": []}"#, &[]).is_err());
        assert!(cli_with(r#"{"number-of-authors": "many"}"#, &[]).is_err());
        assert!(cli_with(r#"{"no-merges": null}"#, &[]).is_err());
        assert!(cli_with("[]", &[]).is_err());
    }

    #[test]
    fn presets_are_valid() -> Result<()> {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("presets");
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            cli_with(&fs::read_to_string(&path)?, &[])
                .with_context(|| format!("Invalid preset `{}`", path.display()))?;
        }
        Ok(())
    }
}
