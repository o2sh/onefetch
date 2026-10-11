use super::FieldView;
use crate::info::fields::version::Version;
use crate::render::format::FormatOptions;
use crate::render::text::Line;

impl FieldView for Version {
    fn value(&self, _options: &FormatOptions) -> Vec<Line> {
        vec![Line::from(self.version.to_string())]
    }

    fn key(&self) -> String {
        "Version".into()
    }
}
