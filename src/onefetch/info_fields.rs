use {
    crate::onefetch::error::*,
    std::str::FromStr,
    strum::{EnumCount, EnumIter, EnumString, IntoStaticStr},
};

#[derive(PartialEq, Eq, EnumString, EnumCount, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum InfoField {
    GitInfo,
    Project,
    HEAD,
    Version,
    Created,
    Dependencies,
    Languages,
    Authors,
    LastChange,
    Repo,
    Commits,
    Pending,
    LinesOfCode,
    Size,
    License,
    UnrecognizedField,
}

#[derive(Default)]
pub struct InfoFieldOff {
    pub git_info: bool,
    pub project: bool,
    pub head: bool,
    pub version: bool,
    pub created: bool,
    pub dependencies: bool,
    pub languages: bool,
    pub authors: bool,
    pub last_change: bool,
    pub repo: bool,
    pub commits: bool,
    pub pending: bool,
    pub lines_of_code: bool,
    pub size: bool,
    pub license: bool,
}

impl InfoFieldOff {
    pub fn new(fields_to_hide: Vec<String>) -> Result<Self> {
        let mut disabled_fields = InfoFieldOff { ..Default::default() };

        for field in fields_to_hide.iter() {
            let item = InfoField::from_str(field.to_lowercase().as_str())?;

            match item {
                InfoField::GitInfo => disabled_fields.git_info = true,
                InfoField::Project => disabled_fields.project = true,
                InfoField::HEAD => disabled_fields.head = true,
                InfoField::Version => disabled_fields.version = true,
                InfoField::Created => disabled_fields.created = true,
                InfoField::Dependencies => disabled_fields.dependencies = true,
                InfoField::Languages => disabled_fields.languages = true,
                InfoField::Authors => disabled_fields.authors = true,
                InfoField::LastChange => disabled_fields.last_change = true,
                InfoField::Repo => disabled_fields.repo = true,
                InfoField::Pending => disabled_fields.pending = true,
                InfoField::Commits => disabled_fields.commits = true,
                InfoField::LinesOfCode => disabled_fields.lines_of_code = true,
                InfoField::Size => disabled_fields.size = true,
                InfoField::License => disabled_fields.license = true,
                _ => (),
            }
        }

        Ok(disabled_fields)
    }
}
