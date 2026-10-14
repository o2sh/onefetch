/// Replaces control characters so untrusted repository data can't inject
/// terminal escape sequences.
pub(super) fn sanitize(text: &str) -> String {
    text.replace(char::is_control, "\u{FFFD}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sanitize() {
        // OSC set-title sequence
        let sanitized = sanitize("1.0.0\u{1b}]0;PWNED\u{07}");
        assert_eq!(sanitized, "1.0.0\u{FFFD}]0;PWNED\u{FFFD}");
    }

    #[test]
    fn test_sanitize_keeps_printable_text() {
        assert_eq!(sanitize("Rust (100.0 %) ●"), "Rust (100.0 %) ●");
    }
}
