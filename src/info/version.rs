use crate::info::utils::info_field::InfoField;
use anyhow::Result;
use gix::Repository;
use onefetch_manifest::Manifest;
use serde::Serialize;

#[derive(Serialize)]
pub struct VersionInfo {
    pub version: String,
}

impl VersionInfo {
    pub fn new(repo: &Repository, manifest: Option<&Manifest>) -> Result<Self> {
        let version = get_version(repo, manifest)?;
        Ok(Self { version })
    }
}

fn get_version(repo: &Repository, manifest: Option<&Manifest>) -> Result<String> {
    if let Some(m) = manifest {
        if !m.version.is_empty() {
            return Ok(m.version.clone());
        }
    }

    let mut version = String::new();
    let mut most_recent = 0;

    for tag in repo.references()?.tags()?.peeled()?.filter_map(Result::ok) {
        if let Ok(commit) = tag.id().object()?.try_into_commit() {
            let current_time = commit.time()?.seconds;
            if current_time > most_recent {
                most_recent = current_time;
                version = tag.name().shorten().to_string();
            }
        }
    }

    Ok(version)
}

#[typetag::serialize]
impl InfoField for VersionInfo {
    fn value(&self) -> String {
        self.version.to_string()
    }

    fn title(&self) -> String {
        "Version".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_version_info() {
        let version_info = VersionInfo {
            version: "v.1.50.0".to_string(),
        };

        assert_eq!(version_info.value(), "v.1.50.0".to_string(),);
    }
}
