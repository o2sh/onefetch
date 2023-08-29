use anyhow::{anyhow, Result};

use gengo::Language;
use std::path::Path;

pub mod language;

pub fn get_main_language(loc_by_language: &[(Language, usize)]) -> Language {
    loc_by_language[0].0.clone()
}

/// Returns a vector of tuples containing all the languages detected inside the repository.
/// Each tuple is composed of the language and its corresponding loc (lines of code).
/// The vector is sorted by loc in descending order.
pub fn get_size_by_language_sorted(dir: &Path) -> Result<Vec<(Language, usize)>> {
    let builder = gengo::Builder::new(dir);
    let gengo = match builder.build() {
        Ok(gengo) => gengo,
        Err(e) => return Err(anyhow!("Could not analyze repository: {}", e)),
    };
    let analysis = match gengo.analyze("HEAD") {
        Ok(analysis) => analysis,
        Err(e) => return Err(anyhow!("Could not analyze repository: {}", e)),
    };
    let summary = analysis.summary();
    let mut vec: Vec<(Language, usize)> =
        summary.into_iter().map(|(k, v)| (k.clone(), *v)).collect();
    vec.sort_by(|a, b| b.1.cmp(&a.1));

    Ok(vec)
}
