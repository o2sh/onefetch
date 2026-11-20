use anyhow::Result;
use gix::{Repository, ThreadSafeRepository, open};
use onefetch::cli::{Cli, InfoArgs};
use onefetch::config::Config;
use onefetch::data::build_info;
use onefetch::data::git::get_work_dir;
use onefetch::options;
use onefetch::render::RenderOptions;
use onefetch::render::info::info_lines;
use regex::Regex;
use rstest::rstest;

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
    let cli: Cli = Cli {
        input: repo.path().to_path_buf(),
        info: InfoArgs {
            email: true,
            churn_pool_size: Some(10),
            ..Default::default()
        },
        ..Default::default()
    };
    let (info_options, _) = options::new(cli, Config::default())?;
    let info = build_info(&info_options)?;
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
    let cli: Cli = Cli {
        input: repo.path().to_path_buf(),
        ..Default::default()
    };
    let (info_options, _) = options::new(cli, Config::default())?;
    let info = build_info(&info_options);
    assert!(info.is_ok());

    Ok(())
}

#[test]
fn test_repo_with_non_origin_remote() -> Result<()> {
    let repo = named_repo(
        "make_repo_with_non_origin_remote.sh",
        "repo_with_non_origin_remote",
    )?;
    let cli: Cli = Cli {
        input: repo.path().to_path_buf(),
        ..Default::default()
    };
    let (info_options, _) = options::new(cli, Config::default())?;
    let info = serde_json::to_value(build_info(&info_options)?)?;
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
    let cli: Cli = Cli {
        input: repo.path().to_path_buf(),
        ..Default::default()
    };
    let (info_options, _) = options::new(cli, Config::default())?;
    let _info = build_info(&info_options).expect("no error");
    Ok(())
}

#[test]
fn test_treeless_partial_repo() -> Result<()> {
    let repo = named_repo("make_partial_repo.sh", "partial_repo/partial_treeless")?;
    let cli: Cli = Cli {
        input: repo.path().to_path_buf(),
        ..Default::default()
    };
    let (info_options, _) = options::new(cli, Config::default())?;
    let _info = build_info(&info_options).expect("no error");
    Ok(())
}

#[test]
fn test_repo_with_pre_epoch_dates() -> Result<()> {
    let repo = named_repo("make_pre_epoch_repo.sh", "pre_epoch_repo")?;
    let cli: Cli = Cli {
        input: repo.path().to_path_buf(),
        ..Default::default()
    };
    let (info_options, _) = options::new(cli, Config::default())?;
    let info = build_info(&info_options).expect("no error");

    let json = serde_json::to_value(&info)?;
    let created = json["infoFields"]
        .as_array()
        .and_then(|fields| fields.iter().find_map(|field| field.get("CreatedInfo")))
        .and_then(|created| created["creationDate"].as_str());
    assert_eq!(created, Some("1803-03-14T23:51:00Z"));

    let created_line = |iso_time| {
        let options = RenderOptions {
            iso_time,
            ..Default::default()
        };
        info_lines(&info, &options)
            .into_iter()
            .map(|line| {
                line.0
                    .into_iter()
                    .map(|segment| segment.text)
                    .collect::<String>()
            })
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
    let cli = Cli {
        input: repo.path().to_path_buf(),
        ..Default::default()
    };
    let (info_options, _) = options::new(cli, Config::default())?;
    let error = match build_info(&info_options) {
        Ok(_) => panic!("reftable repository should be rejected"),
        Err(error) => error,
    };

    assert_eq!(
        error.to_string(),
        "reftable repositories are not yet supported"
    );
    Ok(())
}

#[rstest]
#[case::default(None, 12, &[("Author One", 9, 38), ("Author Two", 7, 29), ("Author Three", 6, 25), ("dependabot[bot]", 1, 4), ("renovate[bot]", 1, 4)])]
#[case::ignore_author(Some(r"\[bot\]"), 12, &[("Author One", 9, 41), ("Author Two", 7, 32), ("Author Three", 6, 27)])]
fn test_repo_with_co_authors(
    #[case] ignore_author: Option<&str>,
    #[case] expected_number_of_commits: u64,
    #[case] expected_authors: &[(&str, u64, u64)],
) -> Result<()> {
    let repo = named_repo("make_repo_with_co_authors.sh", "repo_with_co_authors")?;
    let cli = Cli {
        input: repo.path().to_path_buf(),
        info: InfoArgs {
            number_of_authors: Some(5),
            ignore_author: ignore_author.map(Regex::new).transpose()?,
            ..Default::default()
        },
        ..Default::default()
    };
    let (info_options, _) = options::new(cli, Config::default())?;
    let info = serde_json::to_value(build_info(&info_options)?)?;
    let field = |name: &str| {
        info["infoFields"]
            .as_array()
            .and_then(|fields| fields.iter().find_map(|field| field.get(name)))
            .cloned()
            .unwrap_or_default()
    };

    assert_eq!(
        field("CommitsInfo")["numberOfCommits"].as_u64(),
        Some(expected_number_of_commits)
    );
    let authors: Vec<(String, u64, u64)> = field("AuthorsInfo")["authors"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|author| {
            (
                author["name"].as_str().unwrap_or_default().to_owned(),
                author["nbrOfCommits"].as_u64().unwrap_or_default(),
                author["contribution"].as_u64().unwrap_or_default(),
            )
        })
        .collect();
    let expected_authors: Vec<(String, u64, u64)> = expected_authors
        .iter()
        .map(|&(name, commits, contribution)| (name.to_owned(), commits, contribution))
        .collect();
    assert_eq!(authors, expected_authors);
    Ok(())
}

#[test]
fn test_repo_without_code() -> Result<()> {
    let repo = named_repo("make_repo_without_code.sh", "repo_without_code")?;
    let cli: Cli = Cli {
        input: repo.path().to_path_buf(),
        ..Default::default()
    };
    let (info_options, _) = options::new(cli, Config::default())?;
    let _info = build_info(&info_options).expect("no error");
    Ok(())
}
