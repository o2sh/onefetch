use anyhow::Result;
use gix::{Repository, ThreadSafeRepository, open};
use onefetch::cli::{Cli, InfoArgs};
use onefetch::git::get_work_dir;
use onefetch::info::display_options::DisplayOptions;
use onefetch::info::{InfoOptions, build_info};
use onefetch::ui::layout::info_lines;

pub fn named_repo(fixture_name: &str, name: &str) -> Result<Repository> {
    let repo_path = gix_testtools::scripted_fixture_read_only(fixture_name)
        .unwrap()
        .join(name);
    let safe_repo = ThreadSafeRepository::open_opts(repo_path, open::Options::isolated())?;
    Ok(safe_repo.to_thread_local())
}

#[test]
fn test_bare_repo() -> Result<()> {
    let repo = named_repo("make_bare_repo.sh", "bare_repo")?;
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
    let repo = named_repo("make_repo.sh", "repo")?;
    let config: Cli = Cli {
        input: repo.path().to_path_buf(),
        info: InfoArgs {
            email: true,
            churn_pool_size: Some(10),
            ..Default::default()
        },
        ..Default::default()
    };
    let info = build_info(&InfoOptions::from(&config))?;
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
    let repo = named_repo("make_repo_without_remote.sh", "repo_without_remote")?;
    let config: Cli = Cli {
        input: repo.path().to_path_buf(),
        ..Default::default()
    };
    let info = build_info(&InfoOptions::from(&config));
    assert!(info.is_ok());

    Ok(())
}

#[test]
fn test_repo_with_non_origin_remote() -> Result<()> {
    let repo = named_repo(
        "make_repo_with_non_origin_remote.sh",
        "repo_with_non_origin_remote",
    )?;
    let config: Cli = Cli {
        input: repo.path().to_path_buf(),
        ..Default::default()
    };
    let info = serde_json::to_value(build_info(&InfoOptions::from(&config))?)?;
    let repo_url = info["infoFields"]
        .as_array()
        .and_then(|fields| fields.iter().find_map(|field| field.get("UrlInfo")))
        .and_then(|url_info| url_info["repoUrl"].as_str());
    assert_eq!(repo_url, Some("https://github.com/user/upstream.git"));

    Ok(())
}

#[test]
fn test_partial_repo() -> Result<()> {
    let repo = named_repo("make_partial_repo.sh", "partial_repo/partial")?;
    let config: Cli = Cli {
        input: repo.path().to_path_buf(),
        ..Default::default()
    };
    let _info = build_info(&InfoOptions::from(&config)).expect("no error");
    Ok(())
}

#[test]
fn test_treeless_partial_repo() -> Result<()> {
    let repo = named_repo("make_partial_repo.sh", "partial_repo/partial_treeless")?;
    let config: Cli = Cli {
        input: repo.path().to_path_buf(),
        ..Default::default()
    };
    let _info = build_info(&InfoOptions::from(&config)).expect("no error");
    Ok(())
}

#[test]
fn test_repo_with_pre_epoch_dates() -> Result<()> {
    let repo = named_repo("make_pre_epoch_repo.sh", "pre_epoch_repo")?;
    let config: Cli = Cli {
        input: repo.path().to_path_buf(),
        ..Default::default()
    };
    let info = build_info(&InfoOptions::from(&config)).expect("no error");

    let json = serde_json::to_value(&info)?;
    let created = json["infoFields"]
        .as_array()
        .and_then(|fields| fields.iter().find_map(|field| field.get("CreatedInfo")))
        .and_then(|created| created["creationDate"].as_str());
    assert_eq!(created, Some("1803-03-14T23:51:00Z"));

    let created_line = |iso_time| {
        let options = DisplayOptions {
            iso_time,
            ..Default::default()
        };
        info_lines(&info, &options)
            .into_iter()
            .map(|line| line.0.into_iter().map(|span| span.text).collect::<String>())
            .find(|line| line.starts_with("Created:"))
    };
    assert_eq!(
        created_line(true).as_deref(),
        Some("Created: 1803-03-14T23:51:00Z")
    );
    assert!(created_line(false).is_some_and(|line| line.ends_with(" years ago")));
    Ok(())
}

#[test]
fn test_reftable_repo_is_rejected() -> Result<()> {
    let repo = named_repo("make_reftable_repo.sh", "reftable")?;
    let config = Cli {
        input: repo.path().to_path_buf(),
        ..Default::default()
    };
    let error = match build_info(&InfoOptions::from(&config)) {
        Ok(_) => panic!("reftable repository should be rejected"),
        Err(error) => error,
    };

    assert_eq!(
        error.to_string(),
        "reftable repositories are not yet supported"
    );
    Ok(())
}

#[test]
fn test_repo_without_code() -> Result<()> {
    let repo = named_repo("make_repo_without_code.sh", "repo_without_code")?;
    let config: Cli = Cli {
        input: repo.path().to_path_buf(),
        ..Default::default()
    };
    let _info = build_info(&InfoOptions::from(&config)).expect("no error");
    Ok(())
}
