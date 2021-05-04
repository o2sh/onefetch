// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]
#![cfg_attr(feature = "fail-on-deprecated", deny(deprecated))]

use onefetch::{cli::Cli, cli_utils, error::*, info, printer::Printer, repo};

use std::{io, process};

mod onefetch;

fn get_repo(config: &Cli) -> Result<git2::Repository> {
    let repo = if config.is_remote {
        let temp_directory = std::env::temp_dir();
        let repo_digest = md5::compute(&config.repo_path);
        let mut repo_directory = temp_directory;
        repo_directory.push("onefetch");
        repo_directory.push(hex::encode(repo_digest.0));
        if repo_directory.exists() {
            git2::Repository::discover(repo_directory)?
        } else {
            repo::clone_remote(&config.repo_path, &repo_directory)?
        }
        
    } else {
        if !repo::is_valid(&config.repo_path)? {
            return Err("please run onefetch inside of a non-bare git repository".into());
        }
        git2::Repository::discover(&config.repo_path)?
    };
    Ok(repo)
}

fn run() -> Result<()> {
    #[cfg(windows)]
    let _ = ansi_term::enable_ansi_support();

    let config = Cli::new()?;

    if config.print_languages {
        return cli_utils::print_supported_languages();
    }

    if config.print_package_managers {
        return cli_utils::print_supported_package_managers();
    }

    let repo = get_repo(&config)?;

    let info = info::Info::new(config, repo)?;

    let mut printer = Printer::new(io::BufWriter::new(io::stdout()), info);

    printer.print()?;

    Ok(())
}

fn main() {
    let result = run();
    match result {
        Ok(_) => {
            process::exit(0);
        }
        Err(error) => {
            let stderr = io::stderr();
            default_error_handler(&error, &mut stderr.lock());
            process::exit(1);
        }
    }
}
