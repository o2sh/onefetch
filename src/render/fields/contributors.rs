use super::FieldView;
use crate::data::fields::contributors::Contributors;
use crate::render::RenderOptions;
use crate::render::text::Line;

impl FieldView for Contributors {
    fn value(&self, options: &RenderOptions) -> Vec<Line> {
        if self.total_number_of_authors > self.number_of_authors {
            vec![Line::from(options.number(&self.total_number_of_authors))]
        } else {
            Vec::new()
        }
    }

    fn key(&self) -> String {
        "Contributors".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_contributors() {
        let contributors = Contributors::new(12, 2);
        assert_eq!(
            contributors.value(&RenderOptions::default()),
            vec![Line::from("12")]
        );
        assert_eq!(contributors.key(), "Contributors".to_string());
    }

    #[test]
    fn test_display_contributors_less_than_authors_to_display() {
        let contributors = Contributors::new(1, 3);

        assert!(contributors.value(&RenderOptions::default()).is_empty());
    }
}
