use super::FieldView;
use crate::data::fields::unreleased::Unreleased;
use crate::render::RenderOptions;
use crate::render::style::Line;

impl FieldView for Unreleased {
    fn value(&self, options: &RenderOptions) -> Vec<Line> {
        let commits = match self.commits_since_tag {
            1 => "1 commit".to_string(),
            n => format!("{} commits", options.number(&n)),
        };
        vec![Line::from(commits)]
    }

    fn key(&self) -> String {
        "Unreleased".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::render::format::NumberSeparator;
    use rstest::rstest;

    #[rstest]
    #[case(0, "0 commits")]
    #[case(1, "1 commit")]
    #[case(1234, "1,234 commits")]
    fn test_display_unreleased(#[case] commits_since_tag: usize, #[case] expected: &str) {
        let unreleased = Unreleased {
            tag: "v2.0.0".into(),
            commits_since_tag,
        };
        let options = RenderOptions {
            number_separator: NumberSeparator::Comma,
            ..RenderOptions::default()
        };

        assert_eq!(unreleased.value(&options), vec![Line::from(expected)]);
    }
}
