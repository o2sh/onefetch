use {
    crate::onefetch::error::*,
    std::str::FromStr,
    strum::{EnumCount, EnumIter, EnumString, IntoStaticStr},
};

#[derive(Default)]
pub struct InfoFieldOn {
    pub git_info: bool,
    pub project: bool,
    pub head: bool,
    pub version: bool,
    pub created: bool,
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

#[derive(PartialEq, Eq, EnumString, EnumCount, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum InfoFields {
    GitInfo,
    Project,
    HEAD,
    Version,
    Created,
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

pub fn get_disabled_fields(fields_to_hide: Vec<String>) -> Result<InfoFieldOn> {
    let mut disabled_fields = InfoFieldOn { ..Default::default() };

    for field in fields_to_hide.iter() {
        let item = InfoFields::from_str(field.to_lowercase().as_str())
            .unwrap_or(InfoFields::UnrecognizedField);

        match item {
            InfoFields::GitInfo => disabled_fields.git_info = true,
            InfoFields::Project => disabled_fields.project = true,
            InfoFields::HEAD => disabled_fields.head = true,
            InfoFields::Version => disabled_fields.version = true,
            InfoFields::Created => disabled_fields.created = true,
            InfoFields::Languages => disabled_fields.languages = true,
            InfoFields::Authors => disabled_fields.authors = true,
            InfoFields::LastChange => disabled_fields.last_change = true,
            InfoFields::Repo => disabled_fields.repo = true,
            InfoFields::Pending => disabled_fields.pending = true,
            InfoFields::Commits => disabled_fields.commits = true,
            InfoFields::LinesOfCode => disabled_fields.lines_of_code = true,
            InfoFields::Size => disabled_fields.size = true,
            InfoFields::License => disabled_fields.license = true,
            _ => (),
        }
    }

    Ok(disabled_fields)
}
