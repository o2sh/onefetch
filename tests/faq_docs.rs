use std::fs;

fn faq() -> String {
    fs::read_to_string("docs/wiki/faq.md").unwrap()
}

#[test]
fn faq_has_title() {
    assert!(faq().starts_with("# FAQ\n"));
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
