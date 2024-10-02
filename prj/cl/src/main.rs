//! Opens a log project for today (or -`-yesterday``) in VS Code, creating it if necessary.
//!
//! # TODO
//!
//! * Accept dates as arguments.
//! * Support an option to print the target directory path rather than spawning VS Code.
//!   - Document a shell function to call this program and `cd`` to the target directory.
//!   - This program cannot directly change the working directory of its parent shell.
//! * Directly support installation of a macOS app bundle.
//!   - Embed the app icon, rather than setting the icon via drag on drop.
//!   - Automate the addition of the binary, which is currently done thus:
//!     ```sh
//!     cargo build --release
//!     cp target/release/cl ../../app/cl.app/Contents/MacOS
//!     ```
//!   - Generate the bundle in `~/Applications`, and remove `../../app` from version control.

use std::{
    env::{self, set_current_dir},
    fs, io,
    path::PathBuf,
    process::{exit, Command},
};

use chrono::{Local, TimeDelta};

#[derive(Debug)]
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

    // Launch VS Code in the target directory.
    //
    // We hard-code a path to the VS Code binary, because simply passing "code" does not seem to
    // work when this program is the main binary of a MacOS bundle double-clicked in Finder. The
    // issue may be that PATH is not set appropriately, yet `.status()` returns `Ok` despite the
    // failure.  VS Code provides a CLI command at `/usr/local/bin/code`, but it's merely a shell
    // script that doesn't appear to do anything we need here, and still causes an extra VS Code
    // dock icon to briefly appear and bounce.
    //
    // By default, Code does not "trust" new directories; so, as a hack, disable the entire
    // "workspace trust" feature for this session.  See also:
    // <https://stackoverflow.com/questions/76987792/when-starting-vs-code-from-the-cli-can-i-make-the-workspace-trusted-without-dis>
    match Command::new("/Applications/Visual Studio Code.app/Contents/Resources/app/bin/code")
        .args(["--disable-workspace-trust", "."])
        .status()
    {
        Ok(status) if !status.success() => {
            eprintln!("Warning: Visual Studio Code returned bad status")
        }

        Ok(_) => (),
        Err(err) => eprintln!("Warning: can't spawn Visual Studio Code: {err}"),
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
