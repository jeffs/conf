#![allow(dead_code, unused_assignments, unused_imports, unused_variables)]
//! # Notes
//!
//! Reads config from `~/.config/jump/targets.csv`. The `targets.csv` file
//! supports blank lines, comment lines (beginning with `#`), and jagged lines.
//! The first column in each row is a directory path, and all subsequent columns
//! are short names for that path.
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

enum ArgError {
    /// Too many arguments were specified.
    Extra(String),
    /// An unrecognized flag was specified.
    Flag(String),
    /// No target was specified.
    Missing,
}

enum Error {
    /// Command-line arguments were incorrect.
    Args(ArgError),
    /// The jump operation failed.
    Jump(jump::Error),
}

impl From<ArgError> for Error {
    fn from(value: ArgError) -> Self {
        Error::Args(value)
    }
}

impl From<jump::Error> for Error {
    fn from(value: jump::Error) -> Self {
        Error::Jump(value)
    }
}

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Extra(s) => write!(f, "{s} is an unexpected argument"),
            Self::Flag(s) => write!(f, "{s} is not a recognized flag"),
            Self::Missing => "expected target".fmt(f),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Args(e) => e.fmt(f),
            Error::Jump(e) => e.fmt(f),
        }
    }
}

struct Args {
    target: String,
}

fn parse_args() -> Result<Args, ArgError> {
    let mut target = None;
    for arg in env::args().skip(1) {
        if arg.starts_with('-') {
            return Err(ArgError::Flag(arg));
        }
        if target.is_some() {
            return Err(ArgError::Extra(arg));
        }
        target = Some(arg);
    }
    let target = target.ok_or(ArgError::Missing)?;
    Ok(Args { target })
}

fn write(mut w: impl Write, s: &[u8]) {
    w.write_all(s).expect("output should be writable");
}

fn main_imp() -> Result<(), Error> {
    let args = parse_args()?;
    let app = jump::App::from_env()?;
    let stdout = io::stdout();
    match app.resolve(&args.target)? {
        jump::Target::Path(path) => write(&stdout, path.as_os_str().as_bytes()),
        jump::Target::String(s) => write(&stdout, s.as_bytes()),
    }
    Ok(())
}

fn main() -> ExitCode {
    // TODO: Print usage on ArgError.
    if let Err(err) = main_imp() {
        eprintln!("jump: {err}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
