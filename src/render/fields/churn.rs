use super::FieldView;
use crate::info::fields::churn::Churn;
use crate::render::RenderOptions;
use crate::render::style::Line;

impl FieldView for Churn {
    fn value(&self, options: &RenderOptions) -> Vec<Line> {
        self.file_churns
            .iter()
            .map(|file_churn| {
                Line::from(format!(
                    "{} {}",
                    shorten_file_path(&file_churn.file_path, 2),
                    options.number(&file_churn.nbr_of_commits)
                ))
            })
            .collect()
    }

    fn key(&self) -> String {
        format!("Churn ({})", self.churn_pool_size)
    }
}

fn shorten_file_path(file_path: &str, depth: usize) -> String {
    let components: Vec<&str> = file_path.split('/').collect();

    if depth == 0 || components.len() <= depth {
        return file_path.to_string();
    }

    let truncated_components: Vec<&str> = components
        .iter()
        .skip(components.len() - depth)
        .copied()
        .collect();

    format!("\u{2026}/{}", truncated_components.join("/"))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::info::fields::churn::FileChurn;

    #[test]
    fn test_display_churn_with_two_file_churns() {
        let churn = Churn {
            file_churns: vec![
                FileChurn::new("path/to/file.txt".into(), 50),
                FileChurn::new("file_2.txt".into(), 30),
            ],
            churn_pool_size: 5,
        };

        assert_eq!(
            churn.value(&RenderOptions::default()),
            vec![
                Line::from("\u{2026}/to/file.txt 50"),
                Line::from("file_2.txt 30"),
            ]
        );
    }

    #[test]
    fn test_truncate_file_path() {
        assert_eq!(shorten_file_path("path/to/file.txt", 3), "path/to/file.txt");
        assert_eq!(shorten_file_path("another/file.txt", 2), "another/file.txt");
        assert_eq!(shorten_file_path("file.txt", 1), "file.txt");
        assert_eq!(
            shorten_file_path("path/to/file.txt", 2),
            "\u{2026}/to/file.txt"
        );
        assert_eq!(
            shorten_file_path("another/file.txt", 1),
            "\u{2026}/file.txt"
        );
        assert_eq!(shorten_file_path("file.txt", 0), "file.txt");
    }
}
