#[typetag::serialize]
pub trait InfoField {
    fn value(&self) -> String;
    fn title(&self) -> String;
    fn r#type(&self) -> InfoType;
    fn should_color(&self) -> bool {
        true
    }
    fn has_value(&self, disabled_infos: &[InfoType]) -> bool {
        !(disabled_infos.contains(&self.r#type()) || self.value().is_empty())
    }
}

#[derive(Clone, clap::ValueEnum, Debug, Eq, PartialEq)]
pub enum InfoType {
    Title,
    Project,
    Description,
    Head,
    Pending,
    Version,
    Created,
    Languages,
    Dependencies,
    Authors,
    LastChange,
    Contributors,
    URL,
    Commits,
    LinesOfCode,
    Size,
    License,
}

#[cfg(test)]
mod test {
    use serde::Serialize;

    use super::*;

    #[derive(Serialize)]
    struct InfoFieldImpl(&'static str);

    #[typetag::serialize]
    impl InfoField for InfoFieldImpl {
        fn value(&self) -> String {
            self.0.into()
        }

        fn title(&self) -> String {
            "title".into()
        }

        fn r#type(&self) -> InfoType {
            InfoType::Project
        }
    }

    #[test]
    fn test_info_field_get() {
        let info = InfoFieldImpl("test");
        assert_eq!(info.has_value(&[]), true);
        assert_eq!(info.value(), "test".to_string());
    }

    #[test]
    fn test_info_field_get_none_when_type_disabled() {
        let info = InfoFieldImpl("test");
        assert_eq!(info.has_value(&[InfoType::Project]), false);
    }

    #[test]
    fn test_info_field_get_none_when_value_is_empty() {
        let info = InfoFieldImpl("");
        assert_eq!(info.has_value(&[]), false);
    }
}
