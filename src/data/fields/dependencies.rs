use crate::data::manifest::Manifest;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Dependencies {
    pub number_of_dependencies: usize,
    pub manifest_type: Option<String>,
}

impl Dependencies {
    pub fn new(manifest: Option<&Manifest>) -> Self {
        Self {
            number_of_dependencies: manifest.map_or(0, |m| m.number_of_dependencies),
            manifest_type: manifest.map(|m| m.manifest_type.to_string()),
        }
    }
}
