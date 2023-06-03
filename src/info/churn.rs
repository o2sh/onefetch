use super::utils::{git::CommitMetrics, info_field::InfoField};
use crate::{cli::NumberSeparator, info::format_number};
use owo_colors::{DynColors, OwoColorize};
use serde::Serialize;
use std::fmt::Write;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Churn {
    file_path: String,
    nbr_of_commits: usize,
    #[serde(skip_serializing)]
    number_separator: NumberSeparator,
}

impl Churn {
    pub fn new(
        file_path: String,
        nbr_of_commits: usize,
        number_separator: NumberSeparator,
    ) -> Self {
        Self {
            file_path,
            nbr_of_commits,
            number_separator,
        }
    }
}

impl std::fmt::Display for Churn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            truncate_file_path(&self.file_path, 2),
            format_number(&self.nbr_of_commits, self.number_separator)
        )
    }
}

#[derive(Serialize)]
pub struct ChurnInfo {
    pub churn: Vec<Churn>,
    #[serde(skip_serializing)]
    pub info_color: DynColors,
}
impl ChurnInfo {
    pub fn new(info_color: DynColors, commit_metrics: &CommitMetrics) -> Self {
        let churn = commit_metrics.churns_to_display.clone();
        Self { churn, info_color }
    }
}
impl std::fmt::Display for ChurnInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut churn_info = String::new();

        let pad = self.title().len() + 2;

        for (i, churn) in self.churn.iter().enumerate() {
            let churn_str = churn.color(self.info_color);

            if i == 0 {
                let _ = write!(churn_info, "{churn_str}");
            } else {
                let _ = write!(churn_info, "\n{:<width$}{}", "", churn_str, width = pad);
            }
        }

        write!(f, "{churn_info}")
    }
}

#[typetag::serialize]
impl InfoField for ChurnInfo {
    fn value(&self) -> String {
        self.to_string()
    }

    fn title(&self) -> String {
        "Churn".into()
    }

    fn should_color(&self) -> bool {
        false
    }
}

fn truncate_file_path(path: &str, depth: usize) -> String {
    let components: Vec<&str> = path.split('/').collect();

    if components.len() <= depth {
        return path.to_string();
    }

    let truncated_components: Vec<&str> = components
        .iter()
        .skip(components.len() - depth)
        .copied()
        .collect();
    let truncated_path = format!(".../{}", truncated_components.join("/"));

    truncated_path
}
