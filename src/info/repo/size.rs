use crate::info::{
    git::Repo,
    info_field::{InfoField, InfoFieldValue, InfoType},
};

pub struct SizeInfo {
    pub repo_size: String,
    pub file_count: u64,
}

impl SizeInfo {
    pub fn new(repo: &Repo) -> Self {
        let (repo_size, file_count) = repo.get_repo_size();
        Self {
            repo_size,
            file_count,
        }
    }
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
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: InfoType::Size,
            value: self.to_string(),
        }
    }
    fn title(&self) -> String {
        String::from("Size")
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

        assert_eq!(size_info.value().value, "2.40 MiB (123 files)".to_string());
    }

    #[test]
    fn test_display_size_info_no_files() {
        let size_info = SizeInfo {
            repo_size: "2.40 MiB".to_string(),
            file_count: 0,
        };

        assert_eq!(size_info.value().value, "2.40 MiB".to_string());
    }

    #[test]
    fn test_display_size_info_one_files() {
        let size_info = SizeInfo {
            repo_size: "2.40 MiB".to_string(),
            file_count: 1,
        };

        assert_eq!(size_info.value().value, "2.40 MiB (1 file)".to_string());
    }
}
