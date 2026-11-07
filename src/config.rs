//! The config file: a JSONC file with `logo`, `display`, `general` and
//! `modules` sections, found in the paths shown by `--list-config-paths`. It
//! sets defaults for the command line: an option given on the command line
//! wins over the same option in the file.

use crate::cli::{Cli, When};
use crate::data::FieldKind;
use crate::data::git::bots::{BotRegex, NO_BOTS_DEFAULT_REGEX_PATTERN};
use crate::language::{Language, LanguageType};
use crate::render::UNSET_COLOR;
use crate::render::format::NumberSeparator;
use crate::render::image::ImageProtocol;
use anyhow::{Context, Result, bail};
use clap::parser::ValueSource;
use clap::{ArgMatches, ValueEnum};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Map, Value};
use std::collections::BTreeMap;
use std::io::Read as _;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

pub const SCHEMA_URL: &str = "https://github.com/o2sh/onefetch/raw/main/docs/json_schema.json";

const CONFIG_FILE_NAME: &str = "config.jsonc";
const DOCS_URL: &str = "https://github.com/o2sh/onefetch/wiki/configuration";

const ACTIVITY_MONTHS: [usize; 3] = [3, 6, 12];
const COLOR_RESOLUTIONS: [usize; 5] = [16, 32, 64, 128, 256];

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// `Some(None)` is `"logo": null`, which hides the logo.
    #[serde(
        default,
        deserialize_with = "deserialize_logo",
        skip_serializing_if = "Option::is_none"
    )]
    pub logo: Option<Option<Logo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub general: Option<General>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<Module>>,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Logo {
    #[serde(rename = "type", default)]
    pub kind: Option<Named<LogoType>>,
    #[serde(default)]
    pub source: Option<String>,
    /// Colors of the ascii art, keyed "1" to "9".
    #[serde(default)]
    pub color: Option<BTreeMap<String, u8>>,
    /// Color resolution of the sixel protocol.
    #[serde(default)]
    pub color_resolution: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum LogoType {
    /// The ascii art of the main language, or of `source` if it is a language
    /// name; otherwise `source` is an image shown with the detected protocol.
    Auto,
    /// The ascii art of the language named by `source`.
    Builtin,
    /// `source` is the ascii art itself.
    Data,
    /// `source` is the path to a text file holding the ascii art.
    File,
    Kitty,
    Sixel,
    Iterm,
    None,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Display {
    #[serde(default)]
    pub color: Option<DisplayColor>,
    #[serde(default)]
    pub bold: Option<bool>,
    #[serde(default)]
    pub number_separator: Option<Named<NumberSeparator>>,
    #[serde(default)]
    pub iso_time: Option<bool>,
    #[serde(default)]
    pub true_color: Option<Named<When>>,
    #[serde(default)]
    pub nerd_fonts: Option<bool>,
}

/// The text colors, in the order of `--text-colors`.
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DisplayColor {
    #[serde(default)]
    pub title: Option<u8>,
    #[serde(default)]
    pub tilde: Option<u8>,
    #[serde(default)]
    pub underline: Option<u8>,
    #[serde(default)]
    pub keys: Option<u8>,
    #[serde(default)]
    pub separator: Option<u8>,
    #[serde(default)]
    pub output: Option<u8>,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct General {
    #[serde(default)]
    pub exclude: Option<Vec<String>>,
    /// `true` uses the default pattern, a string is a custom pattern.
    #[serde(default)]
    pub no_bots: Option<NoBots>,
    #[serde(default)]
    pub no_merges: Option<bool>,
    #[serde(default)]
    pub include_hidden: Option<bool>,
    #[serde(default)]
    pub language_types: Option<Vec<Named<LanguageType>>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NoBots {
    Enabled(bool),
    Pattern(String),
}

/// An entry of `modules`: a name, or an object with a `type` and options.
#[derive(Debug, PartialEq)]
pub struct Module {
    pub kind: ModuleKind,
    pub options: ModuleOptions,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum ModuleKind {
    Title,
    Project,
    Description,
    Head,
    Pending,
    Version,
    Unreleased,
    Created,
    Languages,
    Dependencies,
    Authors,
    #[value(name = "lastchange", alias = "last-change")]
    LastChange,
    Activity,
    Contributors,
    Url,
    Commits,
    Churn,
    #[value(name = "linesofcode", alias = "lines-of-code")]
    LinesOfCode,
    Size,
    License,
    Colors,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ModuleOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool_size: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub months: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hide_token: Option<bool>,
}

impl ModuleKind {
    fn field(self) -> Option<FieldKind> {
        Some(match self {
            Self::Title | Self::Colors => return None,
            Self::Project => FieldKind::Project,
            Self::Description => FieldKind::Description,
            Self::Head => FieldKind::Head,
            Self::Pending => FieldKind::Pending,
            Self::Version => FieldKind::Version,
            Self::Unreleased => FieldKind::Unreleased,
            Self::Created => FieldKind::Created,
            Self::Languages => FieldKind::Languages,
            Self::Dependencies => FieldKind::Dependencies,
            Self::Authors => FieldKind::Authors,
            Self::LastChange => FieldKind::LastChange,
            Self::Activity => FieldKind::Activity,
            Self::Contributors => FieldKind::Contributors,
            Self::Url => FieldKind::Url,
            Self::Commits => FieldKind::Commits,
            Self::Churn => FieldKind::Churn,
            Self::LinesOfCode => FieldKind::LinesOfCode,
            Self::Size => FieldKind::Size,
            Self::License => FieldKind::License,
        })
    }

    fn from_field(field: FieldKind) -> Self {
        *Self::value_variants()
            .iter()
            .find(|kind| kind.field() == Some(field))
            .expect("every field is a module")
    }

    /// The options this module accepts, as written in the config.
    fn options(self) -> &'static [&'static str] {
        match self {
            Self::Authors => &["number", "email"],
            Self::Languages => &["number"],
            Self::Churn => &["number", "poolSize"],
            Self::Activity => &["months"],
            Self::Url => &["http", "hideToken"],
            _ => &[],
        }
    }
}

impl<'de> Deserialize<'de> for Module {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let mut object = match Value::deserialize(deserializer)? {
            Value::String(name) => {
                return Ok(Self {
                    kind: parse_name(&name).map_err(D::Error::custom)?,
                    options: ModuleOptions::default(),
                });
            }
            Value::Object(object) => object,
            _ => return Err(D::Error::custom("a module must be a string or an object")),
        };
        let kind: ModuleKind = match object.remove("type") {
            Some(Value::String(name)) => parse_name(&name).map_err(D::Error::custom)?,
            _ => {
                return Err(D::Error::custom(
                    "a module object must have a string `type`",
                ));
            }
        };
        let name = Named(kind);
        if let Some(key) = object
            .keys()
            .find(|key| !kind.options().contains(&key.as_str()))
        {
            return Err(D::Error::custom(format!(
                "unknown option `{key}` for module `{name}`"
            )));
        }
        let options = serde_json::from_value(Value::Object(object)).map_err(D::Error::custom)?;
        Ok(Self { kind, options })
    }
}

impl Serialize for Module {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let name = Named(self.kind).to_string();
        if self.options == ModuleOptions::default() {
            return serializer.serialize_str(&name);
        }
        let mut object = Map::new();
        object.insert("type".into(), Value::String(name));
        if let Value::Object(options) =
            serde_json::to_value(&self.options).map_err(serde::ser::Error::custom)?
        {
            object.extend(options);
        }
        object.serialize(serializer)
    }
}

/// A clap value enum, written in the config as on the command line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Named<T>(pub T);

impl<T: ValueEnum> std::fmt::Display for Named<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.0.to_possible_value().expect("no skipped values");
        f.write_str(value.get_name())
    }
}

impl<T: ValueEnum> Serialize for Named<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de, T: ValueEnum> Deserialize<'de> for Named<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let name = String::deserialize(deserializer)?;
        parse_name(&name).map(Named).map_err(D::Error::custom)
    }
}

fn parse_name<T: ValueEnum>(name: &str) -> Result<T, String> {
    T::from_str(name, true).map_err(|_| {
        let names: Vec<String> = T::value_variants()
            .iter()
            .map(|v| Named(v.clone()).to_string())
            .collect();
        format!(
            "unknown value `{name}`, expected one of: {}",
            names.join(", ")
        )
    })
}

fn deserialize_logo<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Option<Logo>>, D::Error> {
    Ok(Some(match Value::deserialize(deserializer)? {
        Value::Null => None,
        // `"logo": "rust"` only sets the source.
        Value::String(source) => Some(Logo {
            source: Some(source),
            ..Logo::default()
        }),
        value => Some(serde_json::from_value(value).map_err(D::Error::custom)?),
    }))
}

/// Parses a config: JSON with comments and trailing commas.
pub fn parse(text: &str) -> Result<Config> {
    let mut text = text.to_owned();
    json_strip_comments::strip(&mut text).context("Invalid JSONC")?;
    Ok(serde_json::from_str(&text)?)
}

/// The directories searched for config files, in order.
pub fn search_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    let mut push = |dir: Option<PathBuf>| {
        if let Some(dir) = dir.filter(|d| d.is_absolute())
            && !dirs.contains(&dir)
        {
            dirs.push(dir);
        }
    };
    push(user_config_dir().map(|dir| dir.join("onefetch")));
    if cfg!(windows) {
        for var in ["APPDATA", "LOCALAPPDATA", "ProgramData"] {
            push(env_path(var).map(|dir| dir.join("onefetch")));
        }
    } else {
        let xdg_dirs = env::var("XDG_CONFIG_DIRS")
            .ok()
            .filter(|dirs| !dirs.is_empty())
            .unwrap_or_else(|| "/etc/xdg".into());
        for dir in env::split_paths(&xdg_dirs) {
            push(Some(dir.join("onefetch")));
        }
        push(Some(PathBuf::from("/etc/onefetch")));
    }
    dirs
}

/// `$XDG_CONFIG_HOME`, or `~/.config`.
fn user_config_dir() -> Option<PathBuf> {
    env_path("XDG_CONFIG_HOME").or_else(|| env::home_dir().map(|home| home.join(".config")))
}

fn env_path(var: &str) -> Option<PathBuf> {
    env::var_os(var)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

/// Where `--gen-config` writes when no path is given.
pub fn default_path() -> Result<PathBuf> {
    let dir = user_config_dir().context("Could not determine the home directory")?;
    Ok(dir.join("onefetch").join(CONFIG_FILE_NAME))
}

/// Loads the config named by `--config`, or else the first `config.jsonc`
/// found in the search paths.
pub fn load(config_arg: Option<&str>) -> Result<Option<Config>> {
    let path = match config_arg {
        Some("none") => return Ok(None),
        Some("-") => {
            let mut text = String::new();
            io::stdin().read_to_string(&mut text)?;
            return parse(&text)
                .map(Some)
                .context("Failed to load the config from stdin");
        }
        Some(name) => Some(find_named(name)?),
        None => search_dirs()
            .into_iter()
            .map(|dir| dir.join(CONFIG_FILE_NAME))
            .find(|path| path.is_file()),
    };
    let Some(path) = path else {
        return Ok(None);
    };
    let text = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read the config `{}`", path.display()))?;
    parse(&text)
        .map(Some)
        .with_context(|| format!("Failed to load the config `{}`", path.display()))
}

/// A path, or a file name (with or without `.jsonc`) in the search paths.
fn find_named(name: &str) -> Result<PathBuf> {
    let path = Path::new(name);
    if path.is_file() {
        return Ok(path.to_path_buf());
    }
    if path.is_relative() {
        for dir in search_dirs() {
            for candidate in [dir.join(name), dir.join(format!("{name}.jsonc"))] {
                if candidate.is_file() {
                    return Ok(candidate);
                }
            }
        }
    }
    bail!("Config file or preset `{name}` not found, see `onefetch --list-config-paths`")
}

/// Writes the config generated by `--gen-config`, `--gen-config-full` or
/// `--gen-config-force`, if one of them was given.
pub fn generate_if_requested(cli: &Cli) -> Result<bool> {
    let args = &cli.config;
    let (path, full, force) = match (
        &args.gen_config,
        &args.gen_config_full,
        &args.gen_config_force,
    ) {
        (Some(path), _, _) => (path, false, false),
        (_, Some(path), _) => (path, true, false),
        (_, _, Some(path)) => (path, false, true),
        _ => return Ok(false),
    };
    let mut text = serde_json::to_string_pretty(&Config::from_cli(cli, full))?;
    text.push('\n');

    if path.as_deref() == Some(Path::new("-")) {
        print!("{text}");
        return Ok(true);
    }
    let path = match path {
        Some(path) => path.clone(),
        None => default_path()?,
    };
    if path.exists() && !force {
        bail!(
            "File `{}` exists. Use `--gen-config-force` to overwrite",
            path.display()
        );
    }
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, text).with_context(|| format!("Failed to write `{}`", path.display()))?;
    println!("Configuration file generated: `{}`", path.display());
    println!("Documentation: {DOCS_URL}");
    Ok(true)
}

impl Config {
    /// Sets each option of `cli` that the config has and the command line
    /// doesn't.
    pub fn apply(self, cli: &mut Cli, matches: &ArgMatches) -> Result<()> {
        let unset = |id: &str| matches.value_source(id) != Some(ValueSource::CommandLine);

        if let Some(logo) = self.logo {
            apply_logo(logo, cli, &unset)?;
        }

        if let Some(display) = self.display {
            if let Some(color) = display.color
                && unset("text_colors")
            {
                let slots = [
                    color.title,
                    color.tilde,
                    color.underline,
                    color.keys,
                    color.separator,
                    color.output,
                ];
                cli.text_formatting.text_colors = colors_to_slots(&slots, "display.color")?;
            }
            if let Some(bold) = display.bold
                && unset("no_bold")
            {
                cli.text_formatting.no_bold = !bold;
            }
            if let Some(Named(separator)) = display.number_separator
                && unset("number_separator")
            {
                cli.text_formatting.number_separator = separator;
            }
            if let Some(iso_time) = display.iso_time
                && unset("iso_time")
            {
                cli.text_formatting.iso_time = iso_time;
            }
            if let Some(Named(when)) = display.true_color
                && unset("true_color")
            {
                cli.ascii.true_color = when;
            }
            if let Some(nerd_fonts) = display.nerd_fonts
                && unset("nerd_fonts")
            {
                cli.visuals.nerd_fonts = nerd_fonts;
            }
        }

        if let Some(general) = self.general {
            if let Some(exclude) = general.exclude
                && unset("exclude")
            {
                cli.info.exclude = exclude;
            }
            if let Some(no_bots) = general.no_bots
                && unset("no_bots")
            {
                cli.info.no_bots = match no_bots {
                    NoBots::Enabled(false) => None,
                    NoBots::Enabled(true) => Some(NO_BOTS_DEFAULT_REGEX_PATTERN.parse()?),
                    NoBots::Pattern(pattern) => Some(
                        pattern
                            .parse::<BotRegex>()
                            .context("Invalid `general.noBots` pattern")?,
                    ),
                };
            }
            if let Some(no_merges) = general.no_merges
                && unset("no_merges")
            {
                cli.info.no_merges = no_merges;
            }
            if let Some(include_hidden) = general.include_hidden
                && unset("include_hidden")
            {
                cli.info.include_hidden = include_hidden;
            }
            if let Some(types) = general.language_types
                && unset("language_types")
            {
                cli.info.language_types = types.into_iter().map(|Named(t)| t).collect();
            }
        }

        if let Some(modules) = self.modules {
            apply_modules(modules, cli, &unset)?;
        }
        Ok(())
    }

    /// The config matching `cli`: only `modules` for a minimal config, every
    /// setting for a full one.
    pub fn from_cli(cli: &Cli, full: bool) -> Self {
        let mut config = Self {
            schema: Some(SCHEMA_URL.into()),
            modules: Some(modules_from_cli(cli, full)),
            ..Self::default()
        };
        if !full {
            return config;
        }

        config.logo = Some(Some(logo_from_cli(cli)));

        let text_colors = &cli.text_formatting.text_colors;
        let slot = |i: usize| text_colors.get(i).copied().filter(|&c| c != UNSET_COLOR);
        config.display = Some(Display {
            color: Some(DisplayColor {
                title: slot(0),
                tilde: slot(1),
                underline: slot(2),
                keys: slot(3),
                separator: slot(4),
                output: slot(5),
            }),
            bold: Some(!cli.text_formatting.no_bold),
            number_separator: Some(Named(cli.text_formatting.number_separator)),
            iso_time: Some(cli.text_formatting.iso_time),
            true_color: Some(Named(cli.ascii.true_color.clone())),
            nerd_fonts: Some(cli.visuals.nerd_fonts),
        });

        config.general = Some(General {
            exclude: Some(cli.info.exclude.clone()),
            no_bots: Some(match &cli.info.no_bots {
                None => NoBots::Enabled(false),
                Some(regex) if regex.0.as_str() == NO_BOTS_DEFAULT_REGEX_PATTERN => {
                    NoBots::Enabled(true)
                }
                Some(regex) => NoBots::Pattern(regex.0.as_str().into()),
            }),
            no_merges: Some(cli.info.no_merges),
            include_hidden: Some(cli.info.include_hidden),
            language_types: Some(cli.info.language_types.iter().cloned().map(Named).collect()),
        });
        config
    }
}

fn apply_logo(logo: Option<Logo>, cli: &mut Cli, unset: &impl Fn(&str) -> bool) -> Result<()> {
    let Some(logo) = logo else {
        // `"logo": null`
        if unset("no_art") {
            cli.visuals.no_art = true;
        }
        return Ok(());
    };

    // A logo picked on the command line replaces the config's one entirely.
    let logo_on_command_line = ["ascii_input", "ascii_language", "image", "no_art"]
        .iter()
        .any(|id| !unset(id));
    if !logo_on_command_line {
        let source = logo.source.filter(|s| !s.is_empty());
        let kind = logo.kind.map_or(LogoType::Auto, |Named(kind)| kind);
        let require_source = || {
            source.clone().with_context(|| {
                format!(
                    "`logo.source` is required with `\"type\": \"{}\"`",
                    Named(kind)
                )
            })
        };
        match kind {
            LogoType::Auto => {
                if let Some(source) = &source {
                    match Language::from_str(source, true) {
                        Ok(language) => cli.ascii.ascii_language = Some(language),
                        Err(_) => cli.image.image = Some(source.into()),
                    }
                }
            }
            LogoType::Builtin => {
                let source = require_source()?;
                cli.ascii.ascii_language = Some(
                    parse_name(&source)
                        .map_err(anyhow::Error::msg)
                        .context("Invalid `logo.source`")?,
                );
            }
            LogoType::Data => cli.ascii.ascii_input = Some(require_source()?),
            LogoType::File => {
                let path = require_source()?;
                let art = fs::read_to_string(&path)
                    .with_context(|| format!("Failed to read the logo file `{path}`"))?;
                cli.ascii.ascii_input = Some(art);
            }
            LogoType::Kitty | LogoType::Sixel | LogoType::Iterm => {
                cli.image.image = Some(require_source()?.into());
                if unset("image_protocol") {
                    cli.image.image_protocol = Some(match kind {
                        LogoType::Kitty => ImageProtocol::Kitty,
                        LogoType::Sixel => ImageProtocol::Sixel,
                        _ => ImageProtocol::Iterm,
                    });
                }
            }
            LogoType::None => cli.visuals.no_art = true,
        }
    }

    if let Some(colors) = logo.color
        && unset("ascii_colors")
    {
        let mut slots = [None; 9];
        for (key, color) in colors {
            let index = key
                .parse::<usize>()
                .ok()
                .filter(|i| (1..=9).contains(i))
                .with_context(|| format!("Invalid `logo.color` key `{key}`, expected 1 to 9"))?;
            slots[index - 1] = Some(color);
        }
        cli.ascii.ascii_colors = colors_to_slots(&slots, "logo.color")?;
    }

    if let Some(resolution) = logo.color_resolution
        && unset("color_resolution")
    {
        if !COLOR_RESOLUTIONS.contains(&resolution) {
            bail!(
                "Invalid `logo.colorResolution` {resolution}, expected one of {COLOR_RESOLUTIONS:?}"
            );
        }
        cli.image.color_resolution = resolution;
    }
    Ok(())
}

fn apply_modules(modules: Vec<Module>, cli: &mut Cli, unset: &impl Fn(&str) -> bool) -> Result<()> {
    let mut fields = Vec::new();
    let mut has_title = false;
    let mut has_colors = false;
    for module in modules {
        match module.kind.field() {
            Some(field) if fields.contains(&field) => {
                bail!("Module `{}` is listed twice", Named(module.kind))
            }
            Some(field) => fields.push(field),
            None if module.kind == ModuleKind::Title => has_title = true,
            None => has_colors = true,
        }
        let options = module.options;
        let number_id = match module.kind {
            ModuleKind::Authors => "number_of_authors",
            ModuleKind::Languages => "number_of_languages",
            _ => "number_of_file_churns",
        };
        if let Some(number) = options.number
            && unset(number_id)
        {
            match module.kind {
                ModuleKind::Authors => cli.info.number_of_authors = number,
                ModuleKind::Languages => cli.info.number_of_languages = number,
                _ => cli.info.number_of_file_churns = number,
            }
        }
        if let Some(email) = options.email
            && unset("email")
        {
            cli.info.email = email;
        }
        if let Some(pool_size) = options.pool_size
            && unset("churn_pool_size")
        {
            cli.info.churn_pool_size = Some(pool_size);
        }
        if let Some(months) = options.months
            && unset("activity_months")
        {
            if !ACTIVITY_MONTHS.contains(&months) {
                bail!(
                    "Invalid `months` {months} for module `activity`, expected one of {ACTIVITY_MONTHS:?}"
                );
            }
            cli.info.activity_months = months;
        }
        if let Some(http) = options.http
            && unset("http_url")
        {
            cli.info.http_url = http;
        }
        if let Some(hide_token) = options.hide_token
            && unset("hide_token")
        {
            cli.info.hide_token = hide_token;
        }
    }
    cli.info.fields = Some(fields);
    if unset("no_title") {
        cli.info.no_title = !has_title;
    }
    if unset("no_color_palette") {
        cli.visuals.no_color_palette = !has_colors;
    }
    Ok(())
}

/// Colors by position, with `UNSET_COLOR` in the gaps, trimmed after the last
/// color set.
fn colors_to_slots(slots: &[Option<u8>], key: &str) -> Result<Vec<u8>> {
    if let Some(color) = slots.iter().flatten().find(|&&c| c > 15) {
        bail!("Invalid color {color} in `{key}`, expected 0 to 15");
    }
    let len = slots.iter().rposition(Option::is_some).map_or(0, |i| i + 1);
    Ok(slots[..len]
        .iter()
        .map(|slot| slot.unwrap_or(UNSET_COLOR))
        .collect())
}

fn logo_from_cli(cli: &Cli) -> Logo {
    let (kind, source) = if cli.visuals.no_art {
        (LogoType::None, None)
    } else if let Some(image) = &cli.image.image {
        let kind = match cli.image.image_protocol {
            Some(ImageProtocol::Kitty) => LogoType::Kitty,
            Some(ImageProtocol::Sixel) => LogoType::Sixel,
            Some(ImageProtocol::Iterm) => LogoType::Iterm,
            None => LogoType::Auto,
        };
        (kind, Some(image.display().to_string()))
    } else if let Some(art) = &cli.ascii.ascii_input {
        (LogoType::Data, Some(art.clone()))
    } else if let Some(language) = cli.ascii.ascii_language {
        (LogoType::Builtin, Some(Named(language).to_string()))
    } else {
        (LogoType::Auto, None)
    };
    let color = cli
        .ascii
        .ascii_colors
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c != UNSET_COLOR)
        .map(|(i, &c)| ((i + 1).to_string(), c))
        .collect();
    Logo {
        kind: Some(Named(kind)),
        source,
        color: Some(color),
        color_resolution: Some(cli.image.color_resolution),
    }
}

fn modules_from_cli(cli: &Cli, full: bool) -> Vec<Module> {
    let plain = |kind| Module {
        kind,
        options: ModuleOptions::default(),
    };
    let info = &cli.info;
    let fields = info
        .fields
        .clone()
        .unwrap_or_else(|| FieldKind::value_variants().to_vec());

    let mut modules = Vec::new();
    if !info.no_title {
        modules.push(plain(ModuleKind::Title));
    }
    for field in fields {
        if info.disabled_fields.contains(&field) {
            continue;
        }
        let kind = ModuleKind::from_field(field);
        let options = if full {
            match kind {
                ModuleKind::Authors => ModuleOptions {
                    number: Some(info.number_of_authors),
                    email: Some(info.email),
                    ..ModuleOptions::default()
                },
                ModuleKind::Languages => ModuleOptions {
                    number: Some(info.number_of_languages),
                    ..ModuleOptions::default()
                },
                ModuleKind::Churn => ModuleOptions {
                    number: Some(info.number_of_file_churns),
                    pool_size: info.churn_pool_size,
                    ..ModuleOptions::default()
                },
                ModuleKind::Activity => ModuleOptions {
                    months: Some(info.activity_months),
                    ..ModuleOptions::default()
                },
                ModuleKind::Url => ModuleOptions {
                    http: Some(info.http_url),
                    hide_token: Some(info.hide_token),
                    ..ModuleOptions::default()
                },
                _ => ModuleOptions::default(),
            }
        } else {
            ModuleOptions::default()
        };
        modules.push(Module { kind, options });
    }
    if !cli.visuals.no_color_palette {
        modules.push(plain(ModuleKind::Colors));
    }
    modules
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{CommandFactory, FromArgMatches};

    fn cli_with(config: &str, args: &[&str]) -> Result<Cli> {
        let matches = Cli::command()
            .try_get_matches_from(std::iter::once("onefetch").chain(args.iter().copied()))?;
        let mut cli = Cli::from_arg_matches(&matches)?;
        parse(config)?.apply(&mut cli, &matches)?;
        Ok(cli)
    }

    #[test]
    fn parses_comments_and_trailing_commas() -> Result<()> {
        let config = parse(
            r#"{
                // a comment
                "$schema": "x",
                "modules": ["title", /* inline */ "authors",],
            }"#,
        )?;
        assert_eq!(config.modules.map(|m| m.len()), Some(2));
        Ok(())
    }

    #[test]
    fn rejects_unknown_keys() {
        assert!(parse(r#"{"display": {"nope": 1}}"#).is_err());
        assert!(parse(r#"{"nope": 1}"#).is_err());
        assert!(parse(r#"{"modules": [{"type": "head", "number": 2}]}"#).is_err());
        assert!(parse(r#"{"modules": ["nope"]}"#).is_err());
    }

    #[test]
    fn empty_config_changes_nothing() -> Result<()> {
        assert_eq!(cli_with("{}", &[])?, Cli::default());
        Ok(())
    }

    #[test]
    fn modules_set_fields_order_title_and_palette() -> Result<()> {
        let cli = cli_with(
            r#"{"modules": ["license", {"type": "authors", "number": 5, "email": true}, "head"]}"#,
            &[],
        )?;
        assert_eq!(
            cli.info.fields,
            Some(vec![
                FieldKind::License,
                FieldKind::Authors,
                FieldKind::Head
            ])
        );
        assert!(cli.info.no_title);
        assert!(cli.visuals.no_color_palette);
        assert_eq!(cli.info.number_of_authors, 5);
        assert!(cli.info.email);
        Ok(())
    }

    #[test]
    fn command_line_wins() -> Result<()> {
        let cli = cli_with(
            r#"{
                "display": {"isoTime": false, "numberSeparator": "comma"},
                "modules": [{"type": "authors", "number": 5}]
            }"#,
            &[
                "--number-of-authors",
                "2",
                "--number-separator",
                "space",
                "-z",
            ],
        )?;
        assert_eq!(cli.info.number_of_authors, 2);
        assert_eq!(cli.text_formatting.number_separator, NumberSeparator::Space);
        assert!(cli.text_formatting.iso_time);
        Ok(())
    }

    #[test]
    fn logo_from_config() -> Result<()> {
        let cli = cli_with(r#"{"logo": "rust"}"#, &[])?;
        assert_eq!(cli.ascii.ascii_language, Some(Language::Rust));

        let cli = cli_with(r#"{"logo": null}"#, &[])?;
        assert!(cli.visuals.no_art);

        let cli = cli_with(r#"{"logo": {"type": "sixel", "source": "a.png"}}"#, &[])?;
        assert_eq!(cli.image.image, Some(PathBuf::from("a.png")));
        assert_eq!(cli.image.image_protocol, Some(ImageProtocol::Sixel));

        // A logo on the command line replaces the config's one.
        let cli = cli_with(r#"{"logo": {"type": "none"}}"#, &["--ascii-language", "go"])?;
        assert!(!cli.visuals.no_art);
        assert_eq!(cli.ascii.ascii_language, Some(Language::Go));

        assert!(cli_with(r#"{"logo": {"type": "builtin"}}"#, &[]).is_err());
        Ok(())
    }

    #[test]
    fn colors_keep_defaults_in_gaps() -> Result<()> {
        let cli = cli_with(
            r#"{"logo": {"color": {"2": 4}}, "display": {"color": {"keys": 9}}}"#,
            &[],
        )?;
        assert_eq!(cli.ascii.ascii_colors, vec![UNSET_COLOR, 4]);
        assert_eq!(
            cli.text_formatting.text_colors,
            vec![UNSET_COLOR, UNSET_COLOR, UNSET_COLOR, 9]
        );
        assert!(cli_with(r#"{"display": {"color": {"keys": 16}}}"#, &[]).is_err());
        Ok(())
    }

    #[test]
    fn minimal_config_lists_modules() -> Result<()> {
        let cli = cli_with("{}", &["--disabled-fields", "churn", "--no-title"])?;
        let json = serde_json::to_value(Config::from_cli(&cli, false))?;
        let modules = json["modules"].as_array().unwrap();
        assert_eq!(json["$schema"], SCHEMA_URL);
        assert_eq!(modules.first().unwrap(), "project");
        assert_eq!(modules.last().unwrap(), "colors");
        assert!(!modules.iter().any(|m| m == "churn" || m == "title"));
        assert_eq!(json.as_object().unwrap().len(), 2);
        Ok(())
    }

    #[test]
    fn full_config_round_trips() -> Result<()> {
        let args = [
            "--number-of-authors",
            "4",
            "-E",
            "--no-bots",
            "--text-colors",
            "1",
            "2",
            "--ascii-language",
            "rust",
            "--ascii-colors",
            "3",
            "--activity-months",
            "6",
            "--exclude",
            "*.lock",
            "--number-separator",
            "comma",
            "--no-color-palette",
        ];
        let expected = cli_with("{}", &args)?;
        let text = serde_json::to_string_pretty(&Config::from_cli(&expected, true))?;
        let mut cli = cli_with(&text, &[])?;
        // Only the config sets the field order.
        cli.info.fields = None;
        assert_eq!(cli, expected);
        Ok(())
    }

    #[test]
    fn schema_matches_names() -> Result<()> {
        let schema: Value = serde_json::from_str(include_str!("../docs/json_schema.json"))?;
        let names = |values: &Value| -> Vec<String> {
            values
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap().to_owned())
                .collect()
        };
        let all = |variants: &[_]| -> Vec<String> {
            variants
                .iter()
                .map(|v: &ModuleKind| Named(*v).to_string())
                .collect()
        };
        assert_eq!(
            names(&schema["$defs"]["moduleName"]["enum"]),
            all(ModuleKind::value_variants())
        );
        let logo_types: Vec<String> = LogoType::value_variants()
            .iter()
            .map(|v| Named(*v).to_string())
            .collect();
        assert_eq!(
            names(&schema["properties"]["logo"]["oneOf"][2]["properties"]["type"]["enum"]),
            logo_types
        );
        Ok(())
    }
}
