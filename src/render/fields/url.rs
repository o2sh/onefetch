use super::FieldView;
use crate::info::fields::url::Url;
use crate::render::format::FormatOptions;
use crate::render::text::Line;

impl FieldView for Url {
    fn value(&self, _options: &FormatOptions) -> Vec<Line> {
        vec![Line::from(self.repo_url.to_string())]
    }

    fn key(&self) -> String {
        "URL".into()
    }
}
