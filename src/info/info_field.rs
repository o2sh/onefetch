pub struct InfoFieldValue {
    pub r#type: FieldType,
    pub value: String,
}

pub trait InfoField {
    fn value(&self) -> InfoFieldValue;
}

pub trait InfoFieldValueGetter {
    fn get(&self, disabled_fields: &[FieldType]) -> Option<InfoFieldValue>;
}

impl InfoFieldValueGetter for dyn InfoField {
    fn get(&self, disabled_fields: &[FieldType]) -> Option<InfoFieldValue> {
        let infor_field = self.value();
        if disabled_fields.contains(&infor_field.r#type) || infor_field.value.is_empty() {
            return None;
        }
        Some(infor_field)
    }
}

#[derive(Clone, clap::ValueEnum, Debug, Eq, PartialEq)]
pub enum FieldType {
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

impl FieldType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FieldType::Title => "",
            FieldType::Project => "Project",
            FieldType::Head => "HEAD",
            FieldType::Pending => "Pending",
            FieldType::Version => "Version",
            FieldType::Created => "Create",
            FieldType::Languages => "Languages",
            FieldType::Dependencies => "Dependencies",
            FieldType::Authors => "Authors",
            FieldType::LastChange => "Last Change",
            FieldType::Contributors => "Contributors",
            FieldType::Repo => "Repo",
            FieldType::Commits => "CommitS",
            FieldType::LinesOfCode => "Lines of code",
            FieldType::Size => "Size",
            FieldType::License => "License",
        }
    }
}
