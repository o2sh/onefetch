use crate::info::display_options::DisplayOptions;
use crate::info::text::Line;

pub mod authors;
pub mod churn;
pub mod commits;
pub mod contributors;
pub mod created;
pub mod dependencies;
pub mod description;
pub mod head;
pub mod languages;
pub mod last_change;
pub mod license;
pub mod lines_of_code;
pub mod pending;
pub mod project;
pub mod size;
pub mod url;
pub mod version;

#[typetag::serialize]
pub trait InfoField {
    fn key(&self) -> String;
    fn value(&self, options: &DisplayOptions) -> Vec<Line>;
}

#[derive(Clone, clap::ValueEnum, Debug, Eq, PartialEq)]
pub enum InfoKind {
    Project,
    Description,
    Head,
    Pending,
    Version,
    Created,
    Languages,
    Dependencies,
    Authors,
    LastChange,
    Contributors,
    URL,
    Commits,
    Churn,
    LinesOfCode,
    Size,
    License,
}
