use std::fs;

#[test]
fn linux_release_uses_ubuntu_22_04() {
    let workflow = fs::read_to_string(".github/workflows/cd.yml").unwrap();

    assert!(workflow.contains("os: [ubuntu-22.04, macos-latest, windows-latest]"));
    assert!(workflow.contains("if: matrix.os == 'ubuntu-22.04'"));
    assert!(workflow.contains("Debian 12-compatible glibc baseline"));
    assert!(!workflow.contains("if: matrix.os == 'ubuntu-latest'"));
}
