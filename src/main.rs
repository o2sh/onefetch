// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]
#![cfg_attr(feature = "fail-on-deprecated", deny(deprecated))]

use onefetch::{cli::Cli, cli_utils, error::*, info};

use {
    process::{Command, Stdio},
    std::{io, process},
};

mod onefetch;

fn run() -> Result<()> {
    #[cfg(windows)]
    let _ = ansi_term::enable_ansi_support();

    if !is_git_installed() {
        return Err("Git failed to execute!".into());
    }

    // Load command line options.
    let options = Cli::new()?;

    if options.print_languages {
        return cli_utils::print_supported_languages();
    }

    let info = info::Info::new(options)?;

    let mut printer = cli_utils::Printer::new(io::BufWriter::new(io::stdout()), info);

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

fn is_git_installed() -> bool {
    Command::new("git").arg("--version").stdout(Stdio::null()).status().is_ok()
}
