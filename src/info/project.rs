use super::info_field::{InfoField, InfoFieldType, InfoFieldValue};

pub struct ProjectInfoField {
    pub repo_name: String,
    pub number_of_tags: usize,
    pub number_of_branches: usize,
}

impl std::fmt::Display for ProjectInfoField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let branches_str = match self.number_of_branches {
            0 => String::new(),
            1 => String::from("1 branch"),
            _ => format!("{} branches", self.number_of_branches),
        };

        let tags_str = match self.number_of_tags {
            0 => String::new(),
            1 => String::from("1 tag"),
            _ => format!("{} tags", self.number_of_tags),
        };

        if tags_str.is_empty() && branches_str.is_empty() {
            write!(f, "{}", self.repo_name)
        } else if branches_str.is_empty() || tags_str.is_empty() {
            write!(f, "{} ({}{})", self.repo_name, tags_str, branches_str)
        } else {
            write!(f, "{} ({}, {})", self.repo_name, branches_str, tags_str)
        }
    }
}

impl InfoField for ProjectInfoField {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: InfoFieldType::Project,
            value: format!("{}", self),
        }
    }
}
