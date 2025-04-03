//! # Notes
//!
//! Reads config from `~/.config/jump/targets.csv`. The `targets.csv` file supports blank lines,
//! comment lines (beginning with `#`), and jagged lines.  The first column in each row is a
//! directory path, and all subsequent columns are short names for that path.
//!
//! # TODO
//!
//! * [ ] Support database file specification at runtime, via args or env.
//! * [ ] Support complex expansions like "yesterday's date."
//! * [ ] Add DB path list to error messages about missing or empty targets.

use std::io::Write;
use std::os::unix::ffi::OsStrExt;
use std::process::ExitCode;
use std::{env, fmt, io};

enum Error {
    Flag(String),
    Jump(jump::Error),
}

impl From<jump::Error> for Error {
    fn from(value: jump::Error) -> Self {
        Error::Jump(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Flag(s) => write!(f, "{s} is not a recognized flag"),
            Error::Jump(err) => err.fmt(f),
        }
    }
}

fn write(mut w: impl Write, s: &[u8]) {
    w.write_all(s).expect("output should be writable")
}

fn main_imp() -> Result<(), Error> {
    let app = jump::App::from_env()?;
    let mut is_command = false;
    let stdout = io::stdout();
    for target in env::args().skip(1) {
        match target.as_str() {
            "-c" | "--command" => is_command = true,
            s if s.starts_with("-") => Err(Error::Flag(target))?,
            s if is_command => write(&stdout, &app.command(s)?),
            s => write(&stdout, &app.path(s)?.as_os_str().as_bytes()),
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    if let Err(err) = main_imp() {
        eprintln!("jump: {err}");
        return ExitCode::FAILURE;
    };
    ExitCode::SUCCESS
}
