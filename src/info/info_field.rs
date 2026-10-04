use crate::info::text::Line;

#[typetag::serialize]
pub trait InfoField {
    fn key(&self) -> String;
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
