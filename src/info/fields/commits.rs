use crate::git::metrics::GitMetrics;
use crate::info::display_options::DisplayOptions;
use crate::info::fields::InfoField;
use crate::info::text::Line;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitsInfo {
    pub number_of_commits: usize,
    is_shallow: bool,
}

impl CommitsInfo {
    pub fn new(git_metrics: &GitMetrics, is_shallow: bool) -> Self {
        Self {
            number_of_commits: git_metrics.total_number_of_commits,
            is_shallow,
        }
    }
}

#[typetag::serialize]
impl InfoField for CommitsInfo {
    fn value(&self, options: &DisplayOptions) -> Vec<Line> {
        let commits = format!(
            "{}{}",
            options.number(&self.number_of_commits),
            if self.is_shallow {
                " (shallow)"
            } else {
                Default::default()
            }
        );
        vec![Line::from(commits)]
    }

    fn key(&self) -> String {
        "Commits".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_commits_info() {
        let commits_info = CommitsInfo {
            number_of_commits: 3,
            is_shallow: false,
        };

        assert_eq!(
            commits_info.value(&DisplayOptions::default()),
            vec![Line::from("3")]
        );
    }

    #[test]
    fn test_display_commits_info_shallow() {
        let commits_info = CommitsInfo {
            number_of_commits: 2,
            is_shallow: true,
        };

        assert_eq!(
            commits_info.value(&DisplayOptions::default()),
            vec![Line::from("2 (shallow)")]
        );
    }
}
