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
use super::context::RepoContext;
use anyhow::Result;
use serde::ser::{Serialize, SerializeMap, Serializer};
use strum::{EnumDiscriminants, IntoStaticStr, VariantArray};

/// One line (or block of lines) of repository information.
///
/// Variants are declared in display order. `FieldKind` is generated from this
/// enum and is what `--disabled-fields` takes.
#[derive(EnumDiscriminants)]
#[strum_discriminants(
    name(FieldKind),
    derive(VariantArray, IntoStaticStr),
    strum(serialize_all = "kebab-case")
)]
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

impl Field {
    /// Computes the field of the given kind, or `None` if there is nothing to
    /// show (no source code was found for the language fields).
    pub(crate) fn collect(kind: FieldKind, ctx: &RepoContext) -> Result<Option<Self>> {
        let manifest = ctx.manifest.as_ref();
        let metrics = &ctx.git_metrics;
        let options = ctx.options;
        let field = match kind {
            FieldKind::Project => Self::Project(Project::new(&ctx.repo, &ctx.repo_url, manifest)?),
            FieldKind::Description => Self::Description(Description::new(manifest)),
            FieldKind::Head => Self::Head(Head::new(&ctx.repo)?),
            FieldKind::Pending => Self::Pending(Pending::new(&ctx.repo)?),
            FieldKind::Version => Self::Version(Version::new(&ctx.repo, manifest)?),
            FieldKind::Created => Self::Created(Created::new(metrics)),
            FieldKind::Languages => match &ctx.loc_by_language {
                Some(loc_by_language) => Self::Languages(Languages::new(loc_by_language)),
                None => return Ok(None),
            },
            FieldKind::Dependencies => Self::Dependencies(Dependencies::new(manifest)),
            FieldKind::Authors => Self::Authors(Authors::new(
                &metrics.number_of_commits_by_identity,
                metrics.total_number_of_commits,
                options.number_of_authors,
                options.email,
            )),
            FieldKind::LastChange => Self::LastChange(LastChange::new(metrics)),
            FieldKind::Contributors => Self::Contributors(Contributors::new(
                metrics.total_number_of_authors,
                options.number_of_authors,
            )),
            FieldKind::Url => Self::Url(Url::new(&ctx.repo_url)),
            FieldKind::Commits => Self::Commits(Commits::new(metrics, ctx.repo.is_shallow())),
            FieldKind::Churn => Self::Churn(Churn::new(
                &metrics.number_of_commits_by_file_path,
                metrics.churn_pool_size,
                options.number_of_file_churns,
                &options.exclude,
            )?),
            FieldKind::LinesOfCode => match &ctx.loc_by_language {
                Some(loc_by_language) => Self::LinesOfCode(LinesOfCode::new(loc_by_language)),
                None => return Ok(None),
            },
            FieldKind::Size => Self::Size(Size::new(&ctx.repo)),
            FieldKind::License => Self::License(License::new(&ctx.repo_path, manifest)?),
        };
        Ok(Some(field))
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

    #[test]
    fn field_kind_names_are_kebab_case() {
        let names: Vec<&str> = FieldKind::VARIANTS.iter().map(<&str>::from).collect();
        assert_eq!(
            names,
            [
                "project",
                "description",
                "head",
                "pending",
                "version",
                "created",
                "languages",
                "dependencies",
                "authors",
                "last-change",
                "contributors",
                "url",
                "commits",
                "churn",
                "lines-of-code",
                "size",
                "license",
            ]
        );
    }
}
