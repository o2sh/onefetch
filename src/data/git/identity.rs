use gix::bstr::ByteSlice;
use regex::Regex;

/// An author's name and email, after mailmap resolution.
#[derive(Debug, Hash, PartialOrd, Ord, Eq, PartialEq)]
pub struct Identity {
    pub name: gix::bstr::BString,
    pub email: gix::bstr::BString,
}

impl Identity {
    /// `actor` as mapped by the mailmap, keeping whatever it doesn't remap.
    pub fn resolve(actor: gix::actor::IdentityRef<'_>, mailmap: &gix::mailmap::Snapshot) -> Self {
        let actor = actor.trim();
        let resolved = mailmap.resolve_cow(gix::actor::SignatureRef {
            name: actor.name,
            email: actor.email,
            // The mailmap matches on name and email only.
            time: "",
        });
        Self {
            name: resolved.name.into_owned(),
            email: resolved.email.into_owned(),
        }
    }

    /// Whether this identity's name matches the `--ignore-author` pattern.
    pub fn is_ignored(&self, pattern: Option<&Regex>) -> bool {
        pattern.is_some_and(|pattern| pattern.is_match(self.name.to_str_lossy().as_ref()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case("John Doe", "john@example.org", None, false)]
    #[case("John Doe", "john@example.org", Some(r"\[bot\]"), false)]
    #[case("dependabot[bot]", "bot@example.org", Some(r"\[bot\]"), true)]
    #[case("John Doe", "john@example.org", Some(r"\[bot\]|^John Doe$"), true)]
    #[case("John Doe", "john@example.org", Some("^Doe"), false)]
    #[case("John Doe", "bot@example.org", Some("bot@example"), false)]
    fn test_is_ignored(
        #[case] name: &str,
        #[case] email: &str,
        #[case] pattern: Option<&str>,
        #[case] expected: bool,
    ) -> Result<()> {
        let pattern = pattern.map(Regex::new).transpose()?;
        let identity = Identity {
            name: name.into(),
            email: email.into(),
        };
        assert_eq!(identity.is_ignored(pattern.as_ref()), expected);
        Ok(())
    }
}
