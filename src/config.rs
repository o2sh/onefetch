use serde::{Deserialize, Serialize};
use crate::{
    cli::{NumberSeparator, When},
    info::utils::info_field::InfoType,
};

#[derive(Serialize, Deserialize)]
pub struct ConfigOptions {
    // THIS IS JUST A RAW, REALLY WIP CONFIG STRUCTURE
    #[serde(default)]
    pub disabled_fields: Vec<InfoType>,
    // Lol is this really will turn into comment?
    /// Or maybe this?
    #[serde(default)]
    pub no_title: bool,
    #[serde(default)]
    pub number_of_authors: usize,
    #[serde(default)]
    pub number_of_languages: usize,
    #[serde(default)]
    pub number_of_file_churns: usize,
    #[serde(default)]
    pub no_merges: bool,
    #[serde(default)]
    pub include_hidden: bool,
    #[serde(default)]
    pub iso_time: bool,
    #[serde(default)]
    pub number_separator: NumberSeparator,
    #[serde(default)]
    pub no_bold: bool,
    #[serde(default)]
    pub true_color: When,
    #[serde(default)]
    pub nerd_fonts: bool,
}
