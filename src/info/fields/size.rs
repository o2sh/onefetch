use crate::info::info_field::InfoField;
use crate::info::text::{Line, Span};
use crate::info::utils::quantity;
use byte_unit::{Byte, UnitType};
use gix::Repository;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SizeInfo {
    pub repo_size: String,
    pub file_count: u64,
}

impl SizeInfo {
    pub fn new(repo: &Repository) -> Self {
        let (repo_size, file_count) = get_repo_size(repo);
        Self {
            repo_size,
            file_count,
        }
    }
}

fn get_repo_size(repo: &Repository) -> (String, u64) {
    let (repo_size, file_count) = match repo.index() {
        Ok(index) => {
            let repo_size = index.entries().iter().map(|e| e.stat.size as u64).sum();
            (repo_size, index.entries().len() as u64)
        }
        _ => (0, 0),
    };

    (bytes_to_human_readable(repo_size), file_count)
}

fn bytes_to_human_readable(bytes: u64) -> String {
    let byte = Byte::from_u64(bytes);
    let adjusted_byte_based = byte.get_appropriate_unit(UnitType::Binary);
    format!("{adjusted_byte_based:#.2}")
}

#[typetag::serialize]
impl InfoField for SizeInfo {
    fn value(&self) -> Vec<Line> {
        let mut spans = vec![Span::value(self.repo_size.clone())];
        if let Some(files) = quantity(self.file_count, "file", "files") {
            spans.push(Span::value(" ("));
            spans.extend(files);
            spans.push(Span::value(")"));
        }
        vec![spans.into()]
    }
    fn key(&self) -> String {
        "Size".into()
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_display_size_info() {
        let size_info = SizeInfo {
            repo_size: "2.40 MiB".to_string(),
            file_count: 123,
        };

        assert_eq!(
            size_info.value(),
            vec![Line::from(vec![
                Span::value("2.40 MiB"),
                Span::value(" ("),
                Span::number(123),
                Span::value(" files"),
                Span::value(")")
            ])]
        );
    }

    #[test]
    fn test_display_size_info_no_files() {
        let size_info = SizeInfo {
            repo_size: "2.40 MiB".to_string(),
            file_count: 0,
        };

        assert_eq!(size_info.value(), vec![Line::from("2.40 MiB")]);
    }

    #[test]
    fn test_display_size_info_one_files() {
        let size_info = SizeInfo {
            repo_size: "2.40 MiB".to_string(),
            file_count: 1,
        };

        assert_eq!(
            size_info.value(),
            vec![Line::from(vec![
                Span::value("2.40 MiB"),
                Span::value(" ("),
                Span::number(1),
                Span::value(" file"),
                Span::value(")")
            ])]
        );
    }

    #[rstest(
        case(0, "0 B"),
        case(1023, "1023 B"),
        case(1024, "1 KiB"),
        case(2048, "2 KiB"),
        case(1048576, "1 MiB"),
        case(1099511627776, "1 TiB"),
        case(2577152, "2.46 MiB")
    )]
    fn test_bytes_to_human_readable(#[case] input: u64, #[case] expected: &str) {
        assert_eq!(bytes_to_human_readable(input), expected);
    }
}
