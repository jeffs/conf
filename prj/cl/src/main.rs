//! Opens a log project for today (or -`-yesterday``) in VS Code, creating it if necessary.
//!
//! # TODO
//!
//! * Accept dates as arguments.
//! * Support a CLI mode to print the target directory path rather than spawning VS Code.
//!   - Document a shell function to call this program and `cd`` to the target directory.
//!   - This program cannot directly change the working directory of its parent shell.

use std::{
    env::{self, set_current_dir},
    fs, io,
    path::PathBuf,
    process::{exit, Command},
};

use chrono::{Local, TimeDelta};

enum Error {
    Args(Vec<String>),
    Dir(PathBuf, io::Error),
}

type Result<T> = std::result::Result<T, Error>;

fn main_imp() -> Result<()> {
    let args = env::args().skip(1).collect::<Vec<_>>();

    // Decide what date to create a log directory for, per argv.
    let date = match args.as_slice() {
        [] => Local::now(),
        [arg] if arg == "-y" || arg == "--yesterday" => Local::now() - TimeDelta::days(1),
        _ => return Err(Error::Args(args)),
    }
    .date_naive();

    // Build a target directory path of the form `~/log/YYYY/MM/DD`.
    let path = dirs::home_dir()
        .expect("can't find home directory")
        .join("log")
        .join(date.format("%Y/%m/%d").to_string());

    // Create the target directory, and cd to it.  It's OK if the directory already exists, so
    // ignore the result of `create_dir_all`. If the path ultimately is not a directory,
    // `set_current_dir` will fail.
    let _ = fs::create_dir_all(&path);
    set_current_dir(&path).map_err(|err| Error::Dir(path, err))?;

    // Initialize a git repository.  Re-initializing an extant repo is fine; see
    // <https://git-scm.com/docs/git-init>.
    match Command::new("git").arg("init").status() {
        Ok(status) if !status.success() => eprintln!("Warning: git init returned bad status"),
        Ok(_) => (),
        Err(err) => eprintln!("Warning: can't run git init: {err}"),
    }

    // Launch VS Code in the target directory.  By default, Code does not "trust" new directories;
    // so, as a hack, disable the entire "workspace trust" feature for this session.  See also:
    // <https://stackoverflow.com/questions/76987792/when-starting-vs-code-from-the-cli-can-i-make-the-workspace-trusted-without-dis>
    //     code --disable-workspace-trust .
    match Command::new("code")
        .args(["--disable-workspace-trust", "."])
        .status()
    {
        Ok(status) if !status.success() => eprintln!("Warning: VS Code returned bad status"),
        Ok(_) => (),
        Err(err) => eprintln!("Warning: can't spawn VS Code: {err}"),
    }

    Ok(())
}

fn main() {
    match main_imp() {
        Ok(()) => (),
        Err(Error::Args(args)) => {
            eprintln!("Error: bad arguments: {args:?}");
            eprintln!("Usage: cl [-y|--yesterday]");
            exit(2);
        }
        Err(Error::Dir(path, err)) => {
            eprintln!("Error: {}: can't chdir: {err}", path.display());
            exit(1);
        }
    }
}
