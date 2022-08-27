pub struct InfoFieldValue {
    pub r#type: InfoFieldType,
    pub value: String,
}

pub trait InfoField {
    fn value(&self) -> InfoFieldValue;
}

pub trait InfoFieldValueGetter {
    fn get(&self, disabled_fields: &[InfoFieldType]) -> Option<InfoFieldValue>;
}

impl InfoFieldValueGetter for dyn InfoField {
    fn get(&self, disabled_fields: &[InfoFieldType]) -> Option<InfoFieldValue> {
        let infor_field = self.value();
        if disabled_fields.contains(&infor_field.r#type) || infor_field.value.is_empty() {
            return None;
        }
        Some(infor_field)
    }
}

#[derive(Clone, clap::ValueEnum, Debug, Eq, PartialEq)]
pub enum InfoFieldType {
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

impl InfoFieldType {
    pub fn as_str(&self) -> &'static str {
        match self {
            InfoFieldType::Project => "Project",
            InfoFieldType::Head => "HEAD",
            InfoFieldType::Pending => "Pending",
            InfoFieldType::Version => "Version",
            InfoFieldType::Created => "Create",
            InfoFieldType::Languages => "Languages",
            InfoFieldType::Dependencies => "Dependencies",
            InfoFieldType::Authors => "Authors",
            InfoFieldType::LastChange => "Last Change",
            InfoFieldType::Contributors => "Contributors",
            InfoFieldType::Repo => "Repo",
            InfoFieldType::Commits => "CommitS",
            InfoFieldType::LinesOfCode => "Lines of code",
            InfoFieldType::Size => "Size",
            InfoFieldType::License => "License",
        }
    }
}
