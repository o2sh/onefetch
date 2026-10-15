use super::FieldView;
use crate::data::fields::last_change::LastChange;
use crate::render::RenderOptions;
use crate::render::style::Line;

impl FieldView for LastChange {
    fn value(&self, options: &RenderOptions) -> Vec<Line> {
        vec![Line::from(options.time(self.last_change))]
    }

    fn key(&self) -> String {
        "Last change".into()
    }
}
