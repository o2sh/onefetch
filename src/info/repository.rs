use anyhow::{Context, Result, bail};
use gix::ObjectId;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Repository {
    git: gix::Repository,
    work_dir: PathBuf,
    jujutsu_head: Option<ObjectId>,
}

impl Repository {
    pub fn discover(input: &Path) -> Result<Self> {
        if let Some(root) = find_jujutsu_root(input)?
            && !root.join(".git").exists()
        {
            return Self::open_jujutsu(root);
        }

        let git = gix::discover(input)?;
        let work_dir = git
            .workdir()
            .context("please run onefetch inside of a non-bare git repository")?
            .to_owned();

        Ok(Self {
            git,
            work_dir,
            jujutsu_head: None,
        })
    }

    fn open_jujutsu(root: PathBuf) -> Result<Self> {
        let git_dir = run_jj(&root, &["git", "root"])
            .context("Failed to locate the Git store backing the Jujutsu repository")?;
        let git = gix::open(git_dir.trim()).context("Failed to open the Jujutsu Git store")?;

        let head = run_jj(&root, &["log", "-r", "@", "--no-graph", "-T", "commit_id"])
            .context("Failed to determine the Jujutsu working-copy commit")?;
        let head_id = ObjectId::from_hex(head.trim().as_bytes())
            .context("Jujutsu returned an invalid working-copy commit ID")?;

        Ok(Self {
            git,
            work_dir: root,
            jujutsu_head: Some(head_id),
        })
    }

    pub fn git(&self) -> &gix::Repository {
        &self.git
    }

    pub fn work_dir(&self) -> &Path {
        &self.work_dir
    }

    pub fn head_id(&self) -> Result<ObjectId> {
        match self.jujutsu_head {
            Some(head_id) => Ok(head_id),
            None => Ok(self
                .git
                .head_id()
                .context("Failed to retrieve HEAD ID")?
                .detach()),
        }
    }

    pub fn jujutsu_head(&self) -> Option<ObjectId> {
        self.jujutsu_head
    }

    pub fn is_jujutsu(&self) -> bool {
        self.jujutsu_head.is_some()
    }
}

fn find_jujutsu_root(input: &Path) -> Result<Option<PathBuf>> {
    let input = input
        .canonicalize()
        .with_context(|| format!("Failed to resolve repository path '{}'.", input.display()))?;
    let start = if input.is_dir() {
        input.as_path()
    } else {
        input
            .parent()
            .context("The repository path has no parent directory")?
    };

    Ok(start
        .ancestors()
        .find(|path| path.join(".jj").is_dir())
        .map(Path::to_owned))
}

fn run_jj(root: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("jj")
        .args([
            "--ignore-working-copy",
            "--no-pager",
            "--color",
            "never",
            "-R",
        ])
        .arg(root)
        .args(args)
        .output()
        .context("Failed to execute `jj`; is Jujutsu installed?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("`jj` exited with {}: {}", output.status, stderr.trim());
    }

    String::from_utf8(output.stdout).context("Jujutsu returned non-UTF-8 output")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_jujutsu_root_from_nested_directory() -> Result<()> {
        let fixture =
            std::env::temp_dir().join(format!("onefetch-jj-root-test-{}", std::process::id()));
        let nested = fixture.join("a/b");
        std::fs::create_dir_all(fixture.join(".jj"))?;
        std::fs::create_dir_all(&nested)?;

        assert_eq!(find_jujutsu_root(&nested)?, Some(fixture.clone()));

        std::fs::remove_dir_all(fixture)?;
        Ok(())
    }
}
