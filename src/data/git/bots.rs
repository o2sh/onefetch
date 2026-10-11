use anyhow::Result;
use gix::bstr::{BString, ByteSlice};
use regex::Regex;
use std::str::FromStr;

pub const NO_BOTS_DEFAULT_REGEX_PATTERN: &str = r"(?:-|\s)[Bb]ot$|\[[Bb]ot\]";

/// Matches the names of the authors excluded by `--no-bots`.
#[derive(Clone, Debug)]
pub struct BotRegex(pub Regex);

impl Eq for BotRegex {}

impl PartialEq for BotRegex {
    fn eq(&self, other: &BotRegex) -> bool {
        self.0.as_str() == other.0.as_str()
    }
}

impl FromStr for BotRegex {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(BotRegex(Regex::new(s)?))
    }
}

pub fn is_bot(author_name: &BString, bot_regex: Option<&BotRegex>) -> bool {
    bot_regex.is_some_and(|regex| regex.0.is_match(author_name.to_str_lossy().as_ref()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("John Doe", false)]
    #[case("dependabot[bot]", true)]
    #[case("foo bot", true)]
    #[case("foo-bot", true)]
    #[case("bot", false)]
    fn test_is_bot(#[case] author_name: &str, #[case] expected: bool) -> Result<()> {
        let from_str = BotRegex::from_str(NO_BOTS_DEFAULT_REGEX_PATTERN);
        let no_bots: Option<BotRegex> = Some(from_str?);
        assert_eq!(is_bot(&author_name.into(), no_bots.as_ref()), expected);
        Ok(())
    }
}
