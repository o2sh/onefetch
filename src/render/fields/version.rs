use super::FieldView;
use crate::data::fields::version::Version;
use crate::render::RenderOptions;
use crate::render::text::Line;

impl FieldView for Version {
    fn value(&self, _options: &RenderOptions) -> Vec<Line> {
        vec![Line::from(self.version.to_string())]
    }

    fn key(&self) -> String {
        "Version".into()
    }
}
