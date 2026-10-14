use super::FieldView;
use crate::data::fields::commits::Commits;
use crate::render::RenderOptions;
use crate::render::text::Line;

impl FieldView for Commits {
    fn value(&self, options: &RenderOptions) -> Vec<Line> {
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
    use rstest::rstest;

    #[rstest]
    #[case(false, "3")]
    #[case(true, "3 (shallow)")]
    fn test_display_commits(#[case] is_shallow: bool, #[case] expected: &str) {
        let commits = Commits {
            number_of_commits: 3,
            is_shallow,
        };

        assert_eq!(
            commits.value(&RenderOptions::default()),
            vec![Line::from(expected)]
        );
    }
}
