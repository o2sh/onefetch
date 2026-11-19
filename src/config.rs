//! The config file: a JSONC object whose keys are long command-line options,
//! such as `{"number-of-authors": 5, "no-merges": true}`.

use crate::cli::Cli;
use anyhow::{Context, Result, bail};
use clap::Parser;
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
        let config =
            to_args(&text).with_context(|| format!("Invalid config file `{}`", path.display()))?;
        args.splice(1..1, config);
    }
    Ok(Cli::parse_from(args))
}

/// `$XDG_CONFIG_HOME/onefetch/config.jsonc`, or `~/.config/onefetch/config.jsonc`.
fn default_path() -> Option<PathBuf> {
    let dir = env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|dir| dir.is_absolute())
        .or_else(|| env::home_dir().map(|home| home.join(".config")))?;
    Some(dir.join("onefetch").join("config.jsonc"))
}

/// The config as command-line arguments.
fn to_args(text: &str) -> Result<Vec<OsString>> {
    let mut text = text.to_owned();
    json_strip_comments::strip(&mut text)?;
    let config: Map<String, Value> = serde_json::from_str(&text)?;

    let mut args = Vec::new();
    for (key, value) in config {
        match value {
            Value::Bool(true) => args.push(format!("--{key}")),
            Value::Bool(false) => {}
            Value::Array(items) => {
                for item in items {
                    args.push(format!("--{key}={}", plain(&key, &item)?));
                }
            }
            value => args.push(format!("--{key}={}", plain(&key, &value)?)),
        }
    }
    Ok(args.into_iter().map(OsString::from).collect())
}

fn plain(key: &str, value: &Value) -> Result<String> {
    match value {
        Value::String(s) => Ok(s.clone()),
        Value::Number(n) => Ok(n.to_string()),
        _ => bail!("Invalid value `{value}` for `{key}`"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::FieldKind;

    fn cli_with(config: &str, args: &[&str]) -> Result<Cli> {
        let mut all = vec![OsString::from("onefetch")];
        all.extend(to_args(config)?);
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
