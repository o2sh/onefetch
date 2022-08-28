pub struct InfoFieldValue {
    pub r#type: InfoType,
    pub value: String,
}

pub trait InfoField {
    fn value(&self) -> InfoFieldValue;
    fn title(&self) -> String;
}

pub trait InfoFieldValueGetter {
    fn get(&self, disabled_infos: &[InfoType]) -> Option<InfoFieldValue>;
}

impl<T> InfoFieldValueGetter for T
where
    T: InfoField,
{
    fn get(&self, disabled_infos: &[InfoType]) -> Option<InfoFieldValue> {
        let info_field_value = self.value();
        if disabled_infos.contains(&info_field_value.r#type) || info_field_value.value.is_empty() {
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
