use super::gitoxide_time_to_formatted_time;
use crate::info::{
    git::Commits,
    info_field::{InfoField, InfoType},
};

pub struct LastChangeInfo {
    pub last_change: String,
}

impl LastChangeInfo {
    pub fn new(iso_time: bool, commits: &Commits) -> Self {
        let last_change = get_date_of_last_commit(commits, iso_time);

        Self { last_change }
    }
}

pub fn get_date_of_last_commit(commits: &Commits, iso_time: bool) -> String {
    gitoxide_time_to_formatted_time(commits.time_of_most_recent_commit, iso_time)
}

impl InfoField for LastChangeInfo {
    const TYPE: InfoType = InfoType::LastChange;

    fn value(&self) -> String {
        self.last_change.to_string()
    }

    fn title(&self) -> String {
        String::from("Last change")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_last_change_info() {
        let last_change_info = LastChangeInfo {
            last_change: "34 minutes ago".to_string(),
        };

        assert_eq!(last_change_info.value(), "34 minutes ago".to_string());
    }
}
