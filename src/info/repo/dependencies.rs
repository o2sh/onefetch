use crate::info::info_field::{InfoField, InfoType};
use manifest::Manifest;

pub struct DependenciesInfo {
    pub dependencies: String,
}

impl DependenciesInfo {
    pub fn new(manifest: &Option<Manifest>) -> Self {
        let dependencies = match manifest {
            Some(m) => {
                if m.number_of_dependencies == 0 {
                    String::new()
                } else {
                    format!("{} ({})", m.number_of_dependencies, m.manifest_type)
                }
            }
            None => String::new(),
        };

        Self { dependencies }
    }
}

impl InfoField for DependenciesInfo {
    const TYPE: InfoType = InfoType::Dependencies;

    fn value(&self) -> String {
        self.dependencies.clone()
    }

    fn title(&self) -> String {
        String::from("Dependencies")
    }
}
