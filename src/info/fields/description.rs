use crate::info::manifest::Manifest;
use serde::Serialize;

#[derive(Serialize)]
pub struct Description {
    pub description: Option<String>,
}

impl Description {
    pub fn new(manifest: Option<&Manifest>) -> Self {
        let description = match manifest {
            Some(m) => m.description.clone(),
            None => None,
        };

        Self { description }
    }
}
