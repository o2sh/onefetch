pub mod authors;
pub mod churn;
pub mod commits;
pub mod contributors;
pub mod created;
pub mod dependencies;
pub mod description;
pub mod head;
pub mod languages;
pub mod last_change;
pub mod license;
pub mod lines_of_code;
pub mod pending;
pub mod project;
pub mod size;
pub mod url;
pub mod version;

use self::authors::Authors;
use self::churn::Churn;
use self::commits::Commits;
use self::contributors::Contributors;
use self::created::Created;
use self::dependencies::Dependencies;
use self::description::Description;
use self::head::Head;
use self::languages::Languages;
use self::last_change::LastChange;
use self::license::License;
use self::lines_of_code::LinesOfCode;
use self::pending::Pending;
use self::project::Project;
use self::size::Size;
use self::url::Url;
use self::version::Version;
use serde::ser::{Serialize, SerializeMap, Serializer};

/// The fields `--disabled-fields` can turn off.
#[derive(Clone, Copy, clap::ValueEnum, Debug, Eq, PartialEq)]
pub enum FieldKind {
    Project,
    Description,
    Head,
    Pending,
    Version,
    Created,
    Languages,
    Dependencies,
    Authors,
    LastChange,
    Contributors,
    Url,
    Commits,
    Churn,
    LinesOfCode,
    Size,
    License,
}

/// One info line (or block of lines) and its data.
pub enum Field {
    Project(Project),
    Description(Description),
    Head(Head),
    Pending(Pending),
    Version(Version),
    Created(Created),
    Languages(Languages),
    Dependencies(Dependencies),
    Authors(Authors),
    LastChange(LastChange),
    Contributors(Contributors),
    Url(Url),
    Commits(Commits),
    Churn(Churn),
    LinesOfCode(LinesOfCode),
    Size(Size),
    License(License),
}

/// Serializes as a single-entry map, e.g. `{"HeadInfo": {...}}`, which is the
/// JSON/YAML format of `-o`. A derived impl would match it in JSON but write
/// YAML tags (`!HeadInfo`) instead.
impl Serialize for Field {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;
        match self {
            Self::Project(f) => map.serialize_entry("ProjectInfo", f)?,
            Self::Description(f) => map.serialize_entry("DescriptionInfo", f)?,
            Self::Head(f) => map.serialize_entry("HeadInfo", f)?,
            Self::Pending(f) => map.serialize_entry("PendingInfo", f)?,
            Self::Version(f) => map.serialize_entry("VersionInfo", f)?,
            Self::Created(f) => map.serialize_entry("CreatedInfo", f)?,
            Self::Languages(f) => map.serialize_entry("LanguagesInfo", f)?,
            Self::Dependencies(f) => map.serialize_entry("DependenciesInfo", f)?,
            Self::Authors(f) => map.serialize_entry("AuthorsInfo", f)?,
            Self::LastChange(f) => map.serialize_entry("LastChangeInfo", f)?,
            Self::Contributors(f) => map.serialize_entry("ContributorsInfo", f)?,
            Self::Url(f) => map.serialize_entry("UrlInfo", f)?,
            Self::Commits(f) => map.serialize_entry("CommitsInfo", f)?,
            Self::Churn(f) => map.serialize_entry("ChurnInfo", f)?,
            Self::LinesOfCode(f) => map.serialize_entry("LocInfo", f)?,
            Self::Size(f) => map.serialize_entry("SizeInfo", f)?,
            Self::License(f) => map.serialize_entry("LicenseInfo", f)?,
        }
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_as_single_entry_map() {
        let fields = vec![Field::Url(Url::new("https://github.com/o2sh/onefetch"))];

        assert_eq!(
            serde_json::to_string(&fields).unwrap(),
            r#"[{"UrlInfo":{"repoUrl":"https://github.com/o2sh/onefetch"}}]"#
        );
        assert_eq!(
            serde_yaml::to_string(&fields).unwrap(),
            "- UrlInfo:\n    repoUrl: https://github.com/o2sh/onefetch\n"
        );
    }
}
