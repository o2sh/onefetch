use super::FieldView;
use crate::data::fields::head::{Head, HeadRefs};
use crate::render::RenderOptions;
use crate::render::text::Line;

impl FieldView for Head {
    fn value(&self, _options: &RenderOptions) -> Vec<Line> {
        let HeadRefs {
            short_commit_id,
            refs,
        } = &self.head_refs;
        let head = if refs.is_empty() {
            short_commit_id.clone()
        } else {
            format!("{short_commit_id} ({})", refs.join(", "))
        };
        vec![Line::from(head)]
    }

    fn key(&self) -> String {
        "HEAD".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_head_refs() {
        let head = Head {
            head_refs: HeadRefs::new("be561d5".into(), vec!["main".into(), "origin/main".into()]),
        };
        assert_eq!(
            head.value(&RenderOptions::default()),
            vec![Line::from("be561d5 (main, origin/main)")]
        );
    }

    #[test]
    fn test_display_head_refs_with_no_refs() {
        let head = Head {
            head_refs: HeadRefs::new("be561d5".into(), vec![]),
        };
        assert_eq!(
            head.value(&RenderOptions::default()),
            vec![Line::from("be561d5")]
        );
    }
}
