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
