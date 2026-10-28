use super::FieldView;
use crate::data::fields::tooling::Tooling;
use crate::render::RenderOptions;
use crate::render::style::Line;

impl FieldView for Tooling {
    fn value(&self, _options: &RenderOptions) -> Vec<Line> {
        vec![Line::from(self.tools.join(", "))]
    }

    fn key(&self) -> String {
        "Tooling".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_tooling() {
        let tooling = Tooling {
            tools: vec!["GitHub Actions".into(), "Renovate".into(), "Make".into()],
        };

        assert_eq!(
            tooling.value(&RenderOptions::default()),
            vec![Line::from("GitHub Actions, Renovate, Make")]
        );
    }
}
