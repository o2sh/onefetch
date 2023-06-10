#[typetag::serialize]
pub trait InfoField {
    fn value(&self) -> String;
    fn title(&self) -> String;
}

#[derive(Clone, clap::ValueEnum, Debug, Eq, PartialEq)]
pub enum InfoType {
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
    Churn,
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
    }

    #[test]
    fn test_info_field_with_value() {
        let info = InfoFieldImpl("test");
        assert_eq!(info.title(), "title".to_string());
        assert_eq!(info.value(), "test".to_string());
    }
}
