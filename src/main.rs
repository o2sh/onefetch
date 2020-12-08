// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]
#![cfg_attr(feature = "fail-on-deprecated", deny(deprecated))]

use onefetch::{cli::Cli, cli_utils, error::*, info, printer::Printer, repo};

use std::{io, process};

mod onefetch;

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

    if !cli_utils::is_git_installed() {
        return Err("git failed to execute!".into());
    }

    if !repo::is_valid(&config.repo_path)? {
        return Err("please run onefetch inside of a non-bare git repository".into());
    }

    let print_json = config.print_json;

    let info = info::Info::new(config)?;

    let mut printer = Printer::new(io::BufWriter::new(io::stdout()), info);

    if print_json {
        printer.print_json()?;
    } else {
        printer.print()?;
    }

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
