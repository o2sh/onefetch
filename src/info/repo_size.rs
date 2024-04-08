use crate::{
    cli::NumberSeparator,
    info::utils::{format_number, info_field::InfoField},
};
use byte_unit::{Byte, UnitType};
use gix::Repository;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoSizeInfo {
    pub repo_size: String,
    pub file_count: u64,
    #[serde(skip_serializing)]
    number_separator: NumberSeparator,
}

impl RepoSizeInfo {
    pub fn new(repo: &Repository, number_separator: NumberSeparator) -> Self {
        let (repo_size, file_count) = get_repo_size(repo);
        Self {
            repo_size,
            file_count,
            number_separator,
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

impl std::fmt::Display for RepoSizeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.file_count {
            0 => write!(f, "{}", &self.repo_size),
            1 => write!(f, "{} (1 file)", self.repo_size),
            _ => {
                write!(
                    f,
                    "{} ({} files)",
                    self.repo_size,
                    format_number(&self.file_count, self.number_separator)
                )
            }
        }
    }
}

#[typetag::serialize]
impl InfoField for RepoSizeInfo {
    fn value(&self) -> String {
        self.to_string()
    }
    fn title(&self) -> String {
        "Repository Size".into()
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_display_size_info() {
        let size_info = RepoSizeInfo {
            repo_size: "2.40 MiB".to_string(),
            file_count: 123,
            number_separator: NumberSeparator::Plain,
        };

        assert_eq!(size_info.value(), "2.40 MiB (123 files)".to_string());
    }

    #[test]
    fn test_display_size_info_no_files() {
        let size_info = RepoSizeInfo {
            repo_size: "2.40 MiB".to_string(),
            file_count: 0,
            number_separator: NumberSeparator::Plain,
        };

        assert_eq!(size_info.value(), "2.40 MiB".to_string());
    }

    #[test]
    fn test_display_size_info_one_files() {
        let size_info = RepoSizeInfo {
            repo_size: "2.40 MiB".to_string(),
            file_count: 1,
            number_separator: NumberSeparator::Plain,
        };

        assert_eq!(size_info.value(), "2.40 MiB (1 file)".to_string());
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
