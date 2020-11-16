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
        let mut info_field_off = InfoFieldOff { ..Default::default() };

        for field in fields_to_hide.iter() {
            let item = InfoField::from_str(field.to_lowercase().as_str())?;

            match item {
                InfoField::GitInfo => info_field_off.git_info = true,
                InfoField::Project => info_field_off.project = true,
                InfoField::HEAD => info_field_off.head = true,
                InfoField::Version => info_field_off.version = true,
                InfoField::Created => info_field_off.created = true,
                InfoField::Dependencies => info_field_off.dependencies = true,
                InfoField::Languages => info_field_off.languages = true,
                InfoField::Authors => info_field_off.authors = true,
                InfoField::LastChange => info_field_off.last_change = true,
                InfoField::Repo => info_field_off.repo = true,
                InfoField::Pending => info_field_off.pending = true,
                InfoField::Commits => info_field_off.commits = true,
                InfoField::LinesOfCode => info_field_off.lines_of_code = true,
                InfoField::Size => info_field_off.size = true,
                InfoField::License => info_field_off.license = true,
            }
        }

        Ok(info_field_off)
    }
}
