use super::FieldView;
use crate::data::fields::unreleased::Unreleased;
use crate::render::RenderOptions;
use crate::render::style::Line;

impl FieldView for Unreleased {
    fn value(&self, options: &RenderOptions) -> Vec<Line> {
        let commits = match self.commits_since_tag {
            0 => "nothing".to_string(),
            1 => "1 commit".to_string(),
            n => format!("{} commits", options.number(&n)),
        };
        vec![Line::from(format!(
            "{commits} since {} ({})",
            self.tag,
            options.time(self.tag_date)
        ))]
    }

    fn key(&self) -> String {
        "Unreleased".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::render::format::NumberSeparator;
    use gix::date::Time;
    use rstest::rstest;

    #[rstest]
    #[case(0, "nothing since v2.0.0 (2000-01-02T00:00:00Z)")]
    #[case(1, "1 commit since v2.0.0 (2000-01-02T00:00:00Z)")]
    #[case(1234, "1,234 commits since v2.0.0 (2000-01-02T00:00:00Z)")]
    fn test_display_unreleased(#[case] commits_since_tag: usize, #[case] expected: &str) {
        let unreleased = Unreleased {
            tag: "v2.0.0".into(),
            commits_since_tag,
            tag_date: Time::new(946_771_200, 0),
        };
        let options = RenderOptions {
            iso_time: true,
            number_separator: NumberSeparator::Comma,
            ..RenderOptions::default()
        };

        assert_eq!(unreleased.value(&options), vec![Line::from(expected)]);
    }
}
