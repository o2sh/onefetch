use anyhow::Result;
use gix::{open, Repository, ThreadSafeRepository};
use onefetch::cli::{CliOptions, InfoCliOptions, TextForamttingCliOptions};
use onefetch::info::{build_info, get_work_dir};

fn repo(name: &str) -> Result<Repository> {
    let repo_path = gix_testtools::scripted_fixture_read_only(name).unwrap();
    let safe_repo = ThreadSafeRepository::open_opts(repo_path, open::Options::isolated())?;
    Ok(safe_repo.to_thread_local())
}

pub fn named_repo(fixture: &str, name: &str) -> Result<Repository> {
    let repo_path = gix_testtools::scripted_fixture_read_only(fixture)
        .unwrap()
        .join(name);
    let safe_repo = ThreadSafeRepository::open_opts(repo_path, open::Options::isolated())?;
    Ok(safe_repo.to_thread_local())
}

#[test]
fn test_bare_repo() -> Result<()> {
    let repo = repo("make_bare_repo.sh")?;
    let work_dir = get_work_dir(&repo);
    assert!(
        work_dir.is_err(),
        "oops, info was returned on a bare git repo"
    );
    assert_eq!(
        work_dir.unwrap_err().to_string(),
        "please run onefetch inside of a non-bare git repository"
    );
    Ok(())
}

#[test]
fn test_repo() -> Result<()> {
    let repo = repo("make_repo.sh")?;
    let config: CliOptions = CliOptions {
        input: repo.path().to_path_buf(),
        info: InfoCliOptions {
            email: true,
            churn_pool_size: Some(10),
            ..Default::default()
        },
        text_formatting: TextForamttingCliOptions {
            iso_time: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let info = build_info(&config)?;
    insta::assert_json_snapshot!(
        info,
        {
            ".title.gitVersion" => "git version",
            ".infoFields[].HeadInfo.headRefs.shortCommitId" => "short commit",
        }
    );

    Ok(())
}

#[test]
fn test_repo_without_remote() -> Result<()> {
    let repo = repo("make_repo_without_remote.sh")?;
    let config: CliOptions = CliOptions {
        input: repo.path().to_path_buf(),
        ..Default::default()
    };
    let info = build_info(&config);
    assert!(info.is_ok());

    Ok(())
}

#[test]
fn test_partial_repo() -> Result<()> {
    let repo = named_repo("make_partial_repo.sh", "partial")?;
    let config: CliOptions = CliOptions {
        input: repo.path().to_path_buf(),
        ..Default::default()
    };
    let _info = build_info(&config).expect("no error");
    Ok(())
}

#[test]
fn test_repo_with_pre_epoch_dates() -> Result<()> {
    let repo = repo("make_pre_epoch_repo.sh")?;
    let config: CliOptions = CliOptions {
        input: repo.path().to_path_buf(),
        ..Default::default()
    };
    let _info = build_info(&config).expect("no error");
    Ok(())
}
