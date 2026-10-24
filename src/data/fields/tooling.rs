use serde::Serialize;
use std::fs;
use std::path::Path;

/// The tools a repository is set up with (CI, dependency updates, containers,
/// development environments, git hooks, task runners), found from their files.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tooling {
    pub tools: Vec<String>,
}

/// Each tool and the paths, relative to the repository root, that give it
/// away. Grouped by kind, in the order they are shown.
const TOOLS: &[(&str, &[&str])] = &[
    // CI
    ("GitHub Actions", &[".github/workflows"]),
    ("GitLab CI", &[".gitlab-ci.yml"]),
    ("CircleCI", &[".circleci/config.yml"]),
    ("Travis CI", &[".travis.yml"]),
    (
        "Azure Pipelines",
        &["azure-pipelines.yml", "azure-pipelines.yaml"],
    ),
    ("Jenkins", &["Jenkinsfile"]),
    ("Bitbucket Pipelines", &["bitbucket-pipelines.yml"]),
    ("AppVeyor", &["appveyor.yml", ".appveyor.yml"]),
    ("Drone", &[".drone.yml"]),
    (
        "Woodpecker",
        &[".woodpecker", ".woodpecker.yml", ".woodpecker.yaml"],
    ),
    ("Buildkite", &[".buildkite"]),
    // Dependency updates
    (
        "Dependabot",
        &[".github/dependabot.yml", ".github/dependabot.yaml"],
    ),
    (
        "Renovate",
        &[
            "renovate.json",
            "renovate.json5",
            ".renovaterc",
            ".renovaterc.json",
            ".renovaterc.json5",
            ".github/renovate.json",
            ".github/renovate.json5",
            ".gitlab/renovate.json",
            ".gitlab/renovate.json5",
        ],
    ),
    // Containers and development environments
    (
        "Docker",
        &[
            "Dockerfile",
            "Containerfile",
            "docker-compose.yml",
            "docker-compose.yaml",
            "compose.yml",
            "compose.yaml",
        ],
    ),
    ("Dev Container", &[".devcontainer", ".devcontainer.json"]),
    ("Nix", &["flake.nix", "shell.nix", "default.nix"]),
    // Git hooks
    (
        "pre-commit",
        &[".pre-commit-config.yaml", ".pre-commit-config.yml"],
    ),
    ("Husky", &[".husky"]),
    (
        "Lefthook",
        &["lefthook.yml", "lefthook.yaml", ".lefthook.yml"],
    ),
    // Task runners
    ("Make", &["Makefile", "makefile", "GNUmakefile"]),
    ("just", &["justfile", "Justfile", ".justfile"]),
    ("Task", &["Taskfile.yml", "Taskfile.yaml"]),
];

impl Tooling {
    /// `None` when no tool is found.
    pub fn new(repo_path: &Path) -> Option<Self> {
        let root_entries: Vec<String> = fs::read_dir(repo_path)
            .map(|entries| {
                entries
                    .filter_map(Result::ok)
                    .map(|entry| entry.file_name().to_string_lossy().into_owned())
                    .collect()
            })
            .unwrap_or_default();

        let tools: Vec<String> = TOOLS
            .iter()
            .filter(|(name, paths)| {
                paths.iter().any(|path| repo_path.join(path).exists())
                    || (*name == "Docker" && root_entries.iter().any(|entry| is_dockerfile(entry)))
            })
            .map(|(name, _)| (*name).to_string())
            .collect();

        (!tools.is_empty()).then_some(Self { tools })
    }
}

/// `Dockerfile.dev`, `app.Dockerfile` and the like.
fn is_dockerfile(file_name: &str) -> bool {
    ["Dockerfile.", "Containerfile."]
        .iter()
        .any(|prefix| file_name.starts_with(prefix))
        || file_name.ends_with(".Dockerfile")
        || file_name.ends_with(".Containerfile")
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    /// A fresh directory with the given files (and their parent directories).
    fn repo_with(name: &str, files: &[&str]) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("onefetch-tooling-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        for file in files {
            let path = dir.join(file);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, "").unwrap();
        }
        dir
    }

    #[test]
    fn finds_tools_in_display_order() {
        let dir = repo_with(
            "order",
            &[
                "Makefile",
                "flake.nix",
                ".github/workflows/ci.yml",
                "renovate.json",
                "Dockerfile",
            ],
        );
        let tooling = Tooling::new(&dir).unwrap();
        fs::remove_dir_all(&dir).unwrap();

        assert_eq!(
            tooling.tools,
            vec!["GitHub Actions", "Renovate", "Docker", "Nix", "Make"]
        );
    }

    #[test]
    fn finds_each_tool_once() {
        let dir = repo_with(
            "once",
            &["docker-compose.yml", "Dockerfile", ".github/dependabot.yml"],
        );
        let tooling = Tooling::new(&dir).unwrap();
        fs::remove_dir_all(&dir).unwrap();

        assert_eq!(tooling.tools, vec!["Dependabot", "Docker"]);
    }

    #[test]
    fn finds_suffixed_dockerfiles() {
        let dir = repo_with("dockerfile", &["Dockerfile.dev"]);
        let tooling = Tooling::new(&dir).unwrap();
        fs::remove_dir_all(&dir).unwrap();

        assert_eq!(tooling.tools, vec!["Docker"]);
    }

    #[test]
    fn nothing_found() {
        let dir = repo_with("nothing", &["README.md", ".github/CODEOWNERS"]);
        let tooling = Tooling::new(&dir);
        fs::remove_dir_all(&dir).unwrap();

        assert!(tooling.is_none());
    }

    #[test]
    fn serializes_tools() {
        let tooling = Tooling {
            tools: vec!["GitHub Actions".into(), "Docker".into()],
        };
        assert_eq!(
            serde_json::to_value(&tooling).unwrap(),
            serde_json::json!({ "tools": ["GitHub Actions", "Docker"] })
        );
    }
}
