use super::FieldView;
use crate::info::fields::last_change::LastChange;
use crate::render::format::FormatOptions;
use crate::render::text::Line;

impl FieldView for LastChange {
    fn value(&self, options: &FormatOptions) -> Vec<Line> {
        vec![Line::from(options.time(self.last_change))]
    }

    fn key(&self) -> String {
        "Last change".into()
    }
}
