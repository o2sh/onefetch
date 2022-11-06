use super::gitoxide_time_to_formatted_time;
use crate::info::{
    git::Commits,
    info_field::{InfoField, InfoType},
};

pub struct CreatedInfo {
    pub creation_date: String,
}

impl CreatedInfo {
    pub fn new(iso_time: bool, commits: &Commits) -> Self {
        let creation_date = get_creation_date(commits, iso_time);
        Self { creation_date }
    }
}

fn get_creation_date(commits: &Commits, iso_time: bool) -> String {
    gitoxide_time_to_formatted_time(commits.time_of_first_commit, iso_time)
}

impl InfoField for CreatedInfo {
    const TYPE: InfoType = InfoType::Created;

    fn value(&self) -> String {
        self.creation_date.to_string()
    }

    fn title(&self) -> String {
        "Created".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_created_info() {
        let created_info = CreatedInfo {
            creation_date: "2 years ago".to_string(),
        };

        assert_eq!(created_info.value(), "2 years ago".to_string());
    }
}
