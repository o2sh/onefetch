//! The config file: a JSONC object whose keys are long command-line options,
//! such as `{"number-of-authors": 5, "no-merges": true}`. An option given on the
//! command line wins over the same option in the file.

use crate::cli::Cli;
use anyhow::{Context, Result, bail};
use clap::parser::ValueSource;
use clap::{ArgAction, ArgMatches, CommandFactory, FromArgMatches};
use serde_json::{Map, Value};
use std::ffi::OsString;
use std::path::PathBuf;
use std::{env, fs};

/// Options that only make sense on the command line: the ones that pick the
/// config, and the ones that print something else and exit.
const NOT_IN_CONFIG: [&str; 6] = [
    "config",
    "help",
    "version",
    "languages",
    "package-managers",
    "generate",
];

/// Parses the command line, adding the options of the config file.
pub fn parse_args(args: Vec<OsString>) -> Result<Cli> {
    let matches = Cli::command().get_matches_from(&args);
    let default = default_path(env::var_os("XDG_CONFIG_HOME"), env::home_dir());
    let Some(path) = config_path(matches.get_one::<PathBuf>("config").cloned(), default) else {
        return Ok(Cli::from_arg_matches(&matches)?);
    };
    let text = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read the config file `{}`", path.display()))?;
    with_config(args, &matches, &text)
        .with_context(|| format!("Invalid config file `{}`", path.display()))
}

/// The file given with `--config`, which must exist, or else the default file
/// if it exists.
fn config_path(explicit: Option<PathBuf>, default: Option<PathBuf>) -> Option<PathBuf> {
    explicit.or_else(|| default.filter(|path| path.is_file()))
}

/// `$XDG_CONFIG_HOME/onefetch/config.jsonc`, or `~/.config/onefetch/config.jsonc`.
/// A relative `XDG_CONFIG_HOME` is ignored, as the XDG spec says.
fn default_path(xdg_config_home: Option<OsString>, home: Option<PathBuf>) -> Option<PathBuf> {
    let dir = xdg_config_home
        .map(PathBuf::from)
        .filter(|dir| dir.is_absolute())
        .or_else(|| home.map(|home| home.join(".config")))?;
    Some(dir.join("onefetch").join("config.jsonc"))
}

/// Parses `args` again with the config's options added to them.
fn with_config(mut args: Vec<OsString>, matches: &ArgMatches, text: &str) -> Result<Cli> {
    let mut text = text.to_owned();
    json_strip_comments::strip(&mut text).context("Invalid JSONC")?;
    let config: Map<String, Value> = serde_json::from_str(&text)?;

    // Before a `--`, after which everything is the input path.
    let end = args
        .iter()
        .position(|arg| arg == "--")
        .unwrap_or(args.len());
    args.splice(
        end..end,
        to_args(&config, matches)?.into_iter().map(OsString::from),
    );
    Ok(Cli::from_arg_matches(
        &Cli::command().try_get_matches_from(args)?,
    )?)
}

/// The config as command-line arguments, without the options already given on
/// the command line.
fn to_args(config: &Map<String, Value>, matches: &ArgMatches) -> Result<Vec<String>> {
    let command = Cli::command();
    let mut args = Vec::new();
    for (key, value) in config {
        // Such as `$schema`, which editors use.
        if key.starts_with('$') {
            continue;
        }
        if NOT_IN_CONFIG.contains(&key.as_str()) {
            bail!("`{key}` only works on the command line");
        }
        let arg = command
            .get_arguments()
            .find(|arg| arg.get_long() == Some(key.as_str()))
            .with_context(|| format!("Unknown option `{key}`"))?;
        if matches.value_source(arg.get_id().as_str()) == Some(ValueSource::CommandLine) {
            continue;
        }
        let flag = format!("--{key}");
        match (value, arg.get_action()) {
            (Value::Bool(true), ArgAction::SetTrue) => args.push(flag),
            (Value::Bool(false), ArgAction::SetTrue) => {}
            // `--key=value` keeps a value that starts with `-` a value.
            (Value::Array(values), ArgAction::Append) => {
                for value in values {
                    args.push(format!("{flag}={}", to_string(key, value)?));
                }
            }
            // A list for an option with one value is a list of lines, such
            // as the lines of `ascii-input`.
            (Value::Array(lines), ArgAction::Set) => {
                let lines = lines
                    .iter()
                    .map(|line| to_string(key, line))
                    .collect::<Result<Vec<_>>>()?;
                args.push(format!("{flag}={}", lines.join("\n")));
            }
            (Value::Bool(_) | Value::Array(_), _) => {
                bail!("Invalid value `{value}` for `{key}`")
            }
            (value, _) => args.push(format!("{flag}={}", to_string(key, value)?)),
        }
    }
    Ok(args)
}

fn to_string(key: &str, value: &Value) -> Result<String> {
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
    use crate::language::Language;

    fn cli_with(config: &str, args: &[&str]) -> Result<Cli> {
        let args: Vec<OsString> = std::iter::once("onefetch")
            .chain(args.iter().copied())
            .map(OsString::from)
            .collect();
        let matches = Cli::command().try_get_matches_from(&args)?;
        with_config(args, &matches, config)
    }

    #[test]
    fn empty_config_changes_nothing() -> Result<()> {
        // `Cli` isn't `PartialEq` (`Regex` isn't), so compare the `Debug` output.
        assert_eq!(
            format!("{:?}", cli_with("{}", &[])?),
            format!("{:?}", Cli::default())
        );
        Ok(())
    }

    #[test]
    fn sets_options() -> Result<()> {
        let cli = cli_with(
            r#"{
                // a comment
                "number-of-authors": 5,
                "no-merges": true,
                "email": false,
                "exclude": ["*.lock", "vendor/"],
                "disabled-fields": ["churn", "url"],
                "ascii-language": "rust",
                "text-colors": [1, 2],
                "ignore-author": "^bors$",
            }"#,
            &[],
        )?;
        assert_eq!(cli.info.number_of_authors, 5);
        assert!(cli.info.no_merges);
        assert!(!cli.info.email);
        assert_eq!(cli.info.exclude, ["*.lock", "vendor/"]);
        assert_eq!(cli.info.disabled_fields, [FieldKind::Churn, FieldKind::Url]);
        assert_eq!(cli.ascii.ascii_language, Some(Language::Rust));
        assert_eq!(cli.text_formatting.text_colors, [1, 2]);
        assert_eq!(
            cli.info
                .ignore_author
                .map(|regex| regex.as_str().to_owned()),
            Some("^bors$".into())
        );
        Ok(())
    }

    #[test]
    fn command_line_wins() -> Result<()> {
        let cli = cli_with(
            r#"{"number-of-authors": 5, "exclude": ["a"], "iso-time": true}"#,
            &["--number-of-authors", "2", "-e", "b", "c"],
        )?;
        assert_eq!(cli.info.number_of_authors, 2);
        assert_eq!(cli.info.exclude, ["b", "c"]);
        assert!(cli.text_formatting.iso_time);
        Ok(())
    }

    #[test]
    fn keeps_the_input_path() -> Result<()> {
        let cli = cli_with(r#"{"exclude": ["a"]}"#, &["--", "repo"])?;
        assert_eq!(cli.input, PathBuf::from("repo"));
        assert_eq!(cli.info.exclude, ["a"]);
        Ok(())
    }

    #[test]
    fn joins_lines_of_single_values() -> Result<()> {
        let cli = cli_with(r#"{"ascii-input": ["{0} /\\", "/  \\"]}"#, &[])?;
        assert_eq!(cli.ascii.ascii_input.as_deref(), Some("{0} /\\\n/  \\"));
        Ok(())
    }

    #[test]
    fn rejects_bad_options() {
        assert!(cli_with(r#"{"nope": 1}"#, &[]).is_err());
        assert!(cli_with(r#"{"config": "a.jsonc"}"#, &[]).is_err());
        assert!(cli_with(r#"{"number-of-authors": "many"}"#, &[]).is_err());
        assert!(cli_with(r#"{"no-merges": null}"#, &[]).is_err());
        assert!(cli_with("[]", &[]).is_err());
        assert!(cli_with(r#"{"languages": true}"#, &[]).is_err());
        assert!(cli_with(r#"{"package-managers": true}"#, &[]).is_err());
        assert!(cli_with(r#"{"generate": "bash"}"#, &[]).is_err());
    }

    #[test]
    fn values_follow_the_kind_of_option() -> Result<()> {
        let cli = cli_with(r#"{"exclude": ["-x.lock", "vendor/"]}"#, &[])?;
        assert_eq!(cli.info.exclude, ["-x.lock", "vendor/"]);
        // A list for a flag must not become the input path.
        assert!(cli_with(r#"{"no-merges": ["repo"]}"#, &[]).is_err());
        assert!(cli_with(r#"{"number-of-authors": false}"#, &[]).is_err());
        assert!(cli_with(r#"{"exclude": true}"#, &[]).is_err());
        Ok(())
    }

    #[test]
    fn ignores_dollar_keys() -> Result<()> {
        let cli = cli_with(
            r#"{"$schema": "https://example.com/schema.json", "no-merges": true}"#,
            &[],
        )?;
        assert!(cli.info.no_merges);
        Ok(())
    }

    #[test]
    fn finds_the_config_file() {
        let missing = env::temp_dir().join("onefetch-missing-config.jsonc");
        let existing = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("presets/cowsay.jsonc");
        // `--config` is used even if it doesn't exist, so that reading it fails.
        assert_eq!(
            config_path(Some(missing.clone()), None),
            Some(missing.clone())
        );
        assert_eq!(config_path(None, Some(missing)), None);
        assert_eq!(config_path(None, Some(existing.clone())), Some(existing));
    }

    #[test]
    fn default_path_ignores_a_relative_xdg_config_home() {
        let home = env::temp_dir();
        let in_home = home.join(".config/onefetch/config.jsonc");
        let xdg = home.join("xdg");
        assert_eq!(
            default_path(Some("cfg".into()), Some(home.clone())),
            Some(in_home.clone())
        );
        assert_eq!(
            default_path(Some("".into()), Some(home.clone())),
            Some(in_home)
        );
        assert_eq!(
            default_path(Some(xdg.clone().into_os_string()), Some(home)),
            Some(xdg.join("onefetch/config.jsonc"))
        );
        assert_eq!(default_path(None, None), None);
    }

    #[test]
    fn parse_args_reads_the_given_file() -> Result<()> {
        let preset = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("presets/cowsay.jsonc");
        let args = |path: PathBuf| vec!["onefetch".into(), "--config".into(), path.into()];
        assert!(parse_args(args(preset))?.info.no_merges);
        let missing = env::temp_dir().join("onefetch-missing-config.jsonc");
        assert!(parse_args(args(missing)).is_err());
        Ok(())
    }

    #[test]
    fn presets_are_valid() -> Result<()> {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("presets");
        let mut count = 0;
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.extension().is_some_and(|ext| ext == "jsonc") {
                cli_with(&fs::read_to_string(&path)?, &[])
                    .with_context(|| format!("Invalid preset `{}`", path.display()))?;
                count += 1;
            }
        }
        assert!(count > 0);
        Ok(())
    }
}
