pub trait InfoField {
    const TYPE: InfoType;
    fn value(&self) -> String;
    fn title(&self) -> String;
    fn get(&self, disabled_infos: &[InfoType]) -> Option<String> {
        let info_field_value = self.value();
        if disabled_infos.contains(&Self::TYPE) || info_field_value.is_empty() {
            return None;
        }
        Some(info_field_value)
    }
}

#[derive(Clone, clap::ValueEnum, Debug, Eq, PartialEq)]
pub enum InfoType {
    Title,
    Project,
    Head,
    Pending,
    Version,
    Created,
    Languages,
    Dependencies,
    Authors,
    LastChange,
    Contributors,
    Repo,
    Commits,
    LinesOfCode,
    Size,
    License,
}

#[cfg(test)]
mod test {
    use super::*;

    struct InfoFieldImpl {
        pub val: String,
    }
    impl InfoField for InfoFieldImpl {
        const TYPE: InfoType = InfoType::Project;

        fn value(&self) -> String {
            self.val.clone()
        }

        fn title(&self) -> String {
            String::from("title")
        }
    }

    #[test]
    fn test_info_field_get() {
        let info = InfoFieldImpl {
            val: String::from("test"),
        };

        assert_eq!(info.get(&[]), Some(String::from("test")));
    }

    #[test]
    fn test_info_field_get_none_when_type_disabled() {
        let info = InfoFieldImpl {
            val: String::from("test"),
        };

        assert_eq!(info.get(&[InfoType::Project]), None);
    }

    #[test]
    fn test_info_field_get_none_when_value_is_empty() {
        let info = InfoFieldImpl {
            val: String::from(""),
        };

        assert_eq!(info.get(&[]), None);
    }
}
