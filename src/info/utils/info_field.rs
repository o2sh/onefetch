use crate::info::utils::text::Line;

#[typetag::serialize]
pub trait InfoField {
    fn key(&self) -> String;

    /// Returns the lines of the field's value. Nothing is displayed for the
    /// field if all of them are empty.
    fn value(&self) -> Vec<Line>;
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
