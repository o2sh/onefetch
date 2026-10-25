use super::FieldView;
use crate::data::fields::activity::Activity;
use crate::render::RenderOptions;
use crate::render::style::{Line, Segment, Style};

const BARS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

impl FieldView for Activity {
    fn value(&self, options: &RenderOptions) -> Vec<Line> {
        let period = match self.commits_by_month.len() {
            12 => "1y".to_string(),
            months => format!("{months}m"),
        };
        let summary = match self.total() {
            1 => format!("1 commit ({period})"),
            n => format!("{} commits ({period})", options.number(&n)),
        };
        vec![Line::from(vec![
            Segment::new(sparkline(&self.commits_by_month), Style::Sparkline),
            Segment::value(format!(" {summary}")),
        ])]
    }

    fn key(&self) -> String {
        "Activity".into()
    }
}

/// One bar per month, scaled to the busiest month. Months without commits get
/// the lowest bar and any month with commits at least the second lowest, so a
/// single commit still shows.
fn sparkline(counts: &[usize]) -> String {
    let max = counts.iter().copied().max().unwrap_or(0);
    counts
        .iter()
        .map(|&count| {
            if count == 0 {
                return BARS[0];
            }
            let steps = BARS.len() - 1;
            let level = (count * steps).div_ceil(max);
            BARS[level]
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(&[0, 0, 0], "▁▁▁")]
    #[case(&[0, 1, 7], "▁▂█")]
    #[case(&[1, 100, 50], "▂█▅")]
    #[case(&[3, 3, 3], "███")]
    fn test_sparkline(#[case] counts: &[usize], #[case] expected: &str) {
        assert_eq!(sparkline(counts), expected);
    }

    #[rstest]
    #[case(vec![0; 12], "▁▁▁▁▁▁▁▁▁▁▁▁", " 0 commits (1y)")]
    #[case(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], "▁▁▁▁▁▁▁▁▁▁▁█", " 1 commit (1y)")]
    #[case(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2], "▁▁▁▁▁▁▁▁▁▁▅█", " 3 commits (1y)")]
    #[case(vec![0, 1, 2, 0, 0, 4], "▁▃▅▁▁█", " 7 commits (6m)")]
    #[case(vec![1, 0, 0], "█▁▁", " 1 commit (3m)")]
    fn test_display_activity(
        #[case] commits_by_month: Vec<usize>,
        #[case] bars: &str,
        #[case] summary: &str,
    ) {
        let activity = Activity { commits_by_month };
        assert_eq!(
            activity.value(&RenderOptions::default()),
            vec![Line::from(vec![
                Segment::new(bars, Style::Sparkline),
                Segment::value(summary),
            ])]
        );
    }
}
