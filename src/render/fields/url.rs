use super::FieldView;
use crate::info::fields::url::Url;
use crate::render::RenderOptions;
use crate::render::text::Line;

impl FieldView for Url {
    fn value(&self, _options: &RenderOptions) -> Vec<Line> {
        vec![Line::from(self.repo_url.to_string())]
    }

    fn key(&self) -> String {
        "URL".into()
    }
}
