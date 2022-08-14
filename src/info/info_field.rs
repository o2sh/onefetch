#[derive(Clone, clap::ValueEnum, Debug, Eq, PartialEq)]
pub enum InfoField {
    GitInfo,
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

#[derive(Default)]
pub struct InfoFieldOff {
    pub git_info: bool,
    pub project: bool,
    pub head: bool,
    pub pending: bool,
    pub version: bool,
    pub created: bool,
    pub languages: bool,
    pub dependencies: bool,
    pub authors: bool,
    pub last_change: bool,
    pub contributors: bool,
    pub repo: bool,
    pub commits: bool,
    pub lines_of_code: bool,
    pub size: bool,
    pub license: bool,
}

impl From<&Vec<InfoField>> for InfoFieldOff {
    fn from(fields_to_hide: &Vec<InfoField>) -> Self {
        let mut info_field_off = InfoFieldOff {
            ..Default::default()
        };

        for field in fields_to_hide.iter() {
            match field {
                InfoField::GitInfo => info_field_off.git_info = true,
                InfoField::Project => info_field_off.project = true,
                InfoField::Head => info_field_off.head = true,
                InfoField::Pending => info_field_off.pending = true,
                InfoField::Version => info_field_off.version = true,
                InfoField::Created => info_field_off.created = true,
                InfoField::Languages => info_field_off.languages = true,
                InfoField::Dependencies => info_field_off.dependencies = true,
                InfoField::Authors => info_field_off.authors = true,
                InfoField::LastChange => info_field_off.last_change = true,
                InfoField::Contributors => info_field_off.contributors = true,
                InfoField::Repo => info_field_off.repo = true,
                InfoField::Commits => info_field_off.commits = true,
                InfoField::LinesOfCode => info_field_off.lines_of_code = true,
                InfoField::Size => info_field_off.size = true,
                InfoField::License => info_field_off.license = true,
            }
        }

        info_field_off
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_vec_of_info_fields() {
        let fields_to_hide = vec![InfoField::Version, InfoField::Repo];
        let info_field_off = InfoFieldOff::from(&fields_to_hide);

        assert!(!info_field_off.git_info);
        assert!(!info_field_off.project);
        assert!(!info_field_off.head);
        assert!(!info_field_off.pending);
        assert!(!info_field_off.pending);
        assert!(info_field_off.version);
        assert!(!info_field_off.created);
        assert!(!info_field_off.languages);
        assert!(!info_field_off.dependencies);
        assert!(!info_field_off.authors);
        assert!(!info_field_off.last_change);
        assert!(!info_field_off.contributors);
        assert!(info_field_off.repo);
        assert!(!info_field_off.commits);
        assert!(!info_field_off.lines_of_code);
        assert!(!info_field_off.size);
        assert!(!info_field_off.license);
    }
}
