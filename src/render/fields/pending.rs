use super::FieldView;
use crate::data::fields::pending::Pending;
use crate::render::RenderOptions;
use crate::render::text::Line;

impl FieldView for Pending {
    fn value(&self, _options: &RenderOptions) -> Vec<Line> {
        let mut pending = String::new();
        if self.modified > 0 {
            pending = format!("{}+-", self.modified);
        }

        if self.added > 0 {
            pending = format!("{pending} {}+", self.added);
        }

        if self.deleted > 0 {
            pending = format!("{pending} {}-", self.deleted);
        }

        vec![Line::from(pending.trim())]
    }

    fn key(&self) -> String {
        "Pending".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_pending() {
        let pending = Pending {
            added: 0,
            deleted: 0,
            modified: 4,
        };

        assert_eq!(
            pending.value(&RenderOptions::default()),
            vec![Line::from("4+-")]
        );
    }
}
