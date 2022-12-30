use anyhow::Result;
use git_repository::{open, Repository, ThreadSafeRepository};
use onefetch::cli::Config;
use onefetch::info::Info;

fn repo(name: &str) -> Result<Repository> {
    let name = name.to_string();
    let repo_path = git_testtools::scripted_fixture_read_only(name).unwrap();
    let safe_repo = ThreadSafeRepository::open_opts(repo_path, open::Options::isolated())?;
    Ok(safe_repo.to_thread_local())
}

#[test]
fn test_bare_repo() -> Result<()> {
    let repo = repo("bare_repo.sh")?;
    let res = Info::init_repo_path(&repo);
    assert!(res.is_err(), "oops, info was returned on a bare git repo");
    assert_eq!(
        res.unwrap_err().to_string(),
        "please run onefetch inside of a non-bare git repository"
    );
    Ok(())
}

#[test]
fn test_repo() -> Result<()> {
    let repo = repo("repo.sh")?;
    let config: Config = Config {
        input: repo.path().to_path_buf(),
        iso_time: true,
        ..Default::default()
    };
    let info = Info::new(&config)?;
    insta::assert_json_snapshot!(
        info,
        {
            ".title.gitVersion" => "git version",
            ".infoFields[].HeadInfo.headRefs.shortCommitId" => "short commit"
        }
    );

    Ok(())
}

#[test]
fn test_repo_without_remote() -> Result<()> {
    let repo = repo("basic_repo.sh")?;
    let config: Config = Config {
        input: repo.path().to_path_buf(),
        ..Default::default()
    };
    let info = Info::new(&config);
    assert!(info.is_ok());

    Ok(())
}
