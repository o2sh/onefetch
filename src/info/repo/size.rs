use crate::info::info_field::{InfoField, InfoType};
use byte_unit::Byte;
use git_repository::Repository;

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
            let repo_size = index.entries().iter().map(|e| e.stat.size as u128).sum();
            (repo_size, index.entries().len() as u64)
        }
        _ => (0, 0),
    };

    (bytes_to_human_readable(repo_size), file_count)
}

fn bytes_to_human_readable(bytes: u128) -> String {
    let byte = Byte::from_bytes(bytes);
    byte.get_appropriate_unit(true).to_string()
}

impl std::fmt::Display for SizeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.file_count {
            0 => write!(f, "{}", &self.repo_size),
            1 => write!(f, "{} (1 file)", self.repo_size),
            _ => {
                write!(f, "{} ({} files)", self.repo_size, self.file_count)
            }
        }
    }
}

impl InfoField for SizeInfo {
    const TYPE: InfoType = InfoType::Size;

    fn value(&self) -> String {
        self.to_string()
    }
    fn title(&self) -> String {
        "Size".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_size_info() {
        let size_info = SizeInfo {
            repo_size: "2.40 MiB".to_string(),
            file_count: 123,
        };

        assert_eq!(size_info.value(), "2.40 MiB (123 files)".to_string());
    }

    #[test]
    fn test_display_size_info_no_files() {
        let size_info = SizeInfo {
            repo_size: "2.40 MiB".to_string(),
            file_count: 0,
        };

        assert_eq!(size_info.value(), "2.40 MiB".to_string());
    }

    #[test]
    fn test_display_size_info_one_files() {
        let size_info = SizeInfo {
            repo_size: "2.40 MiB".to_string(),
            file_count: 1,
        };

        assert_eq!(size_info.value(), "2.40 MiB (1 file)".to_string());
    }
}
