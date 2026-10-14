use super::FieldView;
use crate::data::fields::project::Project;
use crate::render::RenderOptions;
use crate::render::text::Line;

impl FieldView for Project {
    fn value(&self, options: &RenderOptions) -> Vec<Line> {
        if self.repo_name.is_empty() {
            return Vec::new();
        }

        let branches = match self.number_of_branches {
            0 => String::new(),
            1 => "1 branch".into(),
            _ => format!("{} branches", options.number(&self.number_of_branches)),
        };

        let tags = match self.number_of_tags {
            0 => String::new(),
            1 => "1 tag".into(),
            _ => format!("{} tags", options.number(&self.number_of_tags)),
        };

        let project = if tags.is_empty() && branches.is_empty() {
            self.repo_name.clone()
        } else if branches.is_empty() || tags.is_empty() {
            format!("{} ({}{})", self.repo_name, tags, branches)
        } else {
            format!("{} ({}, {})", self.repo_name, branches, tags)
        };
        vec![Line::from(project)]
    }

    fn key(&self) -> String {
        "Project".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    fn project(repo_name: &str, number_of_branches: usize, number_of_tags: usize) -> Project {
        Project {
            repo_name: repo_name.to_string(),
            number_of_branches,
            number_of_tags,
        }
    }

    #[rstest]
    #[case(project("onefetch", 3, 2), "onefetch (3 branches, 2 tags)")]
    #[case(project("onefetch", 0, 0), "onefetch")]
    #[case(project("onefetch", 3, 0), "onefetch (3 branches)")]
    #[case(project("onefetch", 0, 2), "onefetch (2 tags)")]
    #[case(project("onefetch", 1, 1), "onefetch (1 branch, 1 tag)")]
    fn test_display_project(#[case] project: Project, #[case] expected: &str) {
        assert_eq!(
            project.value(&RenderOptions::default()),
            vec![Line::from(expected)]
        );
    }

    #[test]
    fn test_display_project_when_no_repo_name() {
        assert!(
            project("", 0, 0)
                .value(&RenderOptions::default())
                .is_empty()
        );
    }
}
