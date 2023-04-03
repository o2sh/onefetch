use anyhow::Result;
use gix::{open, Repository, ThreadSafeRepository};
use onefetch::cli::{
    AsciiCliOptions, CliOptions, TextForamttingCliOptions, VisualsCliOptions, When,
};
use onefetch::info::{get_work_dir, Info};

fn repo(name: &str) -> Result<Repository> {
    let name = name.to_string();
    let repo_path = gix_testtools::scripted_fixture_read_only(name).unwrap();
    let safe_repo = ThreadSafeRepository::open_opts(repo_path, open::Options::isolated())?;
    Ok(safe_repo.to_thread_local())
}

#[test]
fn test_bare_repo() -> Result<()> {
    let repo = repo("bare_repo.sh")?;
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
    let repo = repo("repo.sh")?;
    let config: CliOptions = CliOptions {
        input: repo.path().to_path_buf(),
        text_formatting: TextForamttingCliOptions {
            iso_time: true,
            ..Default::default()
        },
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
    let config: CliOptions = CliOptions {
        input: repo.path().to_path_buf(),
        ..Default::default()
    };
    let info = Info::new(&config);
    assert!(info.is_ok());

    Ok(())
}

#[test]
fn test_repo_with_token_url() -> Result<()> {
    let repo = repo("repo_with_token_url.sh")?;
    let config: CliOptions = CliOptions {
        input: repo.path().to_path_buf(),
        // NOTE: Normalizing for a snapshot
        text_formatting: TextForamttingCliOptions {
            text_colors: Vec::new(),
            iso_time: true,
            no_bold: true,
            ..Default::default()
        },
        ascii: AsciiCliOptions {
            ascii_colors: Vec::new(),
            true_color: When::Never,
            ..Default::default()
        },
        visuals: VisualsCliOptions {
            no_color_palette: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let info = Info::new(&config)?;
    insta::assert_display_snapshot!(info);

    Ok(())
}
