use super::FieldView;
use crate::info::fields::license::License;
use crate::render::format::DisplayOptions;
use crate::render::text::Line;

impl FieldView for License {
    fn value(&self, _options: &DisplayOptions) -> Vec<Line> {
        vec![Line::from(self.license.to_string())]
    }

    fn key(&self) -> String {
        "License".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_license() {
        let license = License {
            license: "MIT".into(),
        };
        assert_eq!(
            license.value(&DisplayOptions::default()),
            vec![Line::from("MIT")]
        );
    }
}
