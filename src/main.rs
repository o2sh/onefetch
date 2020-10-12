// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

use onefetch::{cli::Options, error::*, info};

use {
    process::{Command, Stdio},
    std::process,
};

mod onefetch;

fn run() -> Result<()> {
    #[cfg(windows)]
    let _ = ansi_term::enable_ansi_support();

    if !is_git_installed() {
        return Err("Git failed to execute!".into());
    }

    // Load command line options.
    let options = Options::new()?;

    let info = info::Info::new(options)?;

    print!("{}", info);
    Ok(())
}

fn main() {
    let result = run();
    match result {
        Ok(_) => {
            process::exit(0);
        }
        Err(error) => {
            let stderr = std::io::stderr();
            default_error_handler(&error, &mut stderr.lock());
            process::exit(1);
        }
    }
}

fn is_git_installed() -> bool {
    Command::new("git")
        .arg("--version")
        .stdout(Stdio::null())
        .status()
        .is_ok()
}
