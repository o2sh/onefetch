//! How each field is shown in the terminal: its key and its value lines.
//! One file per field, mirroring `data/fields/`.

use crate::data::Field;
use crate::render::RenderOptions;
use crate::render::style::Line;

mod activity;
mod authors;
mod churn;
mod commits;
mod contributors;
mod created;
mod dependencies;
mod description;
mod head;
mod languages;
mod last_change;
mod license;
mod lines_of_code;
mod pending;
mod project;
mod size;
mod tooling;
mod url;
mod version;

pub(crate) trait FieldView {
    fn key(&self) -> String;
    fn value(&self, options: &RenderOptions) -> Vec<Line>;
}

pub(crate) fn view(field: &Field) -> &dyn FieldView {
    match field {
        Field::Project(f) => f,
        Field::Description(f) => f,
        Field::Head(f) => f,
        Field::Pending(f) => f,
        Field::Version(f) => f,
        Field::Created(f) => f,
        Field::Languages(f) => f,
        Field::Dependencies(f) => f,
        Field::Tooling(f) => f,
        Field::Authors(f) => f,
        Field::LastChange(f) => f,
        Field::Activity(f) => f,
        Field::Contributors(f) => f,
        Field::Url(f) => f,
        Field::Commits(f) => f,
        Field::Churn(f) => f,
        Field::LinesOfCode(f) => f,
        Field::Size(f) => f,
        Field::License(f) => f,
    }
}
