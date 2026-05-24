//! # Notes
//!
//! Reads `jump.yaml` from each directory in `$JUMP_DIRS` (colon-separated).
//! If `$JUMP_DIRS` is empty or unset, falls back to `$XDG_CONFIG_HOME`
//! (defaulting to `~/.config`).
//!
//! Each `jump.yaml` maps target values (paths, URLs) to short names or lists,
//! like `~/conf: conf` or `~/conf: [c, conf]`.
//!
//! # TODO
//!
//! - [ ] Tab completion/expansion; for example:
//!   + Name: `j mo<Tab>` => `jump month`
//!   + Path: `j log<Tab>` => `jump /Users/jeff/log/2025/03/27`
//! - [ ] Parameters; e.g., `jump linear TIK-42` or `jump github my-repo#42`
//! - [ ] Relative date expansion; e.g, yesterday (syntax TBD)

use std::io::Write;
use std::os::unix::ffi::OsStrExt;
use std::process::ExitCode;
use std::{env, fmt, io};

enum ArgError {
    /// Too many arguments were specified.
    Extra(String),
    /// An unrecognized flag was specified.
    Flag(String),
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
    target: Option<String>,
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
    Ok(Args { target })
}

fn write(mut w: impl Write, s: &[u8]) {
    w.write_all(s).expect("output should be writable");
}

fn main_imp() -> Result<(), Error> {
    let args = parse_args()?;
    let app = jump::App::from_env()?;
    let stdout = io::stdout();
    match app.resolve(&args.target.unwrap_or_default())? {
        jump::Target::Path(path) => write(&stdout, path.as_os_str().as_bytes()),
        jump::Target::String(s) => write(&stdout, s.as_bytes()),
    }
    Ok(())
}

fn main() -> ExitCode {
    if let Err(err) = main_imp() {
        eprintln!("error: {err}");
        if matches!(err, Error::Args(_)) {
            eprintln!("usage: jump TARGET");
        }
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
