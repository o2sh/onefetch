//! `info` and `git` collect data and must not depend on the command line or on
//! how the data is displayed. See ARCHITECTURE.md.

use std::fs;
use std::path::Path;

const DATA_MODULES: [&str; 2] = ["src/info", "src/git"];
const FORBIDDEN: [&str; 6] = [
    "crate::cli",
    "crate::options",
    "crate::render",
    "clap::",
    "owo_colors",
    "DisplayOptions",
];

fn rust_files(dir: &Path, files: &mut Vec<std::path::PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            rust_files(&path, files);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            files.push(path);
        }
    }
}

#[test]
fn data_modules_do_not_depend_on_cli_or_render() {
    let mut files = Vec::new();
    for dir in DATA_MODULES {
        rust_files(Path::new(dir), &mut files);
    }
    assert!(!files.is_empty());

    let violations: Vec<String> = files
        .iter()
        .flat_map(|file| {
            let source = fs::read_to_string(file).unwrap();
            source
                .lines()
                .enumerate()
                .filter(|(_, line)| FORBIDDEN.iter().any(|pattern| line.contains(pattern)))
                .map(|(i, line)| format!("{}:{}: {}", file.display(), i + 1, line.trim()))
                .collect::<Vec<_>>()
        })
        .collect();

    assert!(
        violations.is_empty(),
        "data modules must not use the CLI or rendering code:\n{}",
        violations.join("\n")
    );
}
