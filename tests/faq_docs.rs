use std::fs;
use std::path::Path;

fn faq() -> String {
    let faq_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/wiki/faq.md");
    fs::read_to_string(&faq_path)
        .expect("failed to read FAQ at repo path docs/wiki/faq.md")
}

#[test]
fn faq_has_title() {
    let faq = faq();
    let first_line = faq.lines().next().unwrap_or("").trim();

    assert_eq!(first_line, "# FAQ");
}

#[test]
fn faq_explains_language_detection_limits() {
    let faq = faq();

    assert!(faq.contains("Linguist"));
    assert!(faq.contains("very small"));
    assert!(faq.contains("uncommon or unsupported extension"));
    assert!(faq.contains("generated, vendored, binary, ignored"));
}

#[test]
fn faq_guides_unsupported_language_requests() {
    let faq = faq();

    assert!(faq.contains("check whether it is supported"));
    assert!(faq.contains("open a new language request"));
}
