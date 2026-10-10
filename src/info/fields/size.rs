use crate::info::display_options::DisplayOptions;
use crate::info::fields::InfoField;
use crate::info::text::Line;
use byte_unit::{Byte, UnitType};
use gix::Repository;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SizeInfo {
    pub repo_size: u64,
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

fn get_repo_size(repo: &Repository) -> (u64, u64) {
    match repo.index() {
        Ok(index) => {
            let repo_size = index.entries().iter().map(|e| e.stat.size as u64).sum();
            (repo_size, index.entries().len() as u64)
        }
        _ => (0, 0),
    }
}

fn bytes_to_human_readable(bytes: u64) -> String {
    let byte = Byte::from_u64(bytes);
    let adjusted_byte_based = byte.get_appropriate_unit(UnitType::Binary);
    format!("{adjusted_byte_based:#.2}")
}

#[typetag::serialize]
impl InfoField for SizeInfo {
    fn value(&self, options: &DisplayOptions) -> Vec<Line> {
        let repo_size = bytes_to_human_readable(self.repo_size);
        let size = match self.file_count {
            0 => repo_size,
            1 => format!("{repo_size} (1 file)"),
            _ => format!("{repo_size} ({} files)", options.number(&self.file_count)),
        };
        vec![Line::from(size)]
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
            repo_size: 2_577_152,
            file_count: 123,
        };

        assert_eq!(
            size_info.value(&DisplayOptions::default()),
            vec![Line::from("2.46 MiB (123 files)")]
        );
    }

    #[test]
    fn test_display_size_info_no_files() {
        let size_info = SizeInfo {
            repo_size: 2_577_152,
            file_count: 0,
        };

        assert_eq!(
            size_info.value(&DisplayOptions::default()),
            vec![Line::from("2.46 MiB")]
        );
    }

    #[test]
    fn test_display_size_info_one_files() {
        let size_info = SizeInfo {
            repo_size: 2_577_152,
            file_count: 1,
        };

        assert_eq!(
            size_info.value(&DisplayOptions::default()),
            vec![Line::from("2.46 MiB (1 file)")]
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
