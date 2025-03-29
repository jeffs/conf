//! # Notes
//!
//! Reads config from `~/.config/jump/targets.csv`, where `~` is returned by [`std::env::home_dir`].
//! That function is deprecated because it behaved inconsistently on Windows before Rust 1.85, but
//! it does what we want here.
//!
//! The `targets.csv` file supports blank lines, comment lines (beginning with `#`), and jagged
//! lines.  The first column in each row is a directory path, and all subsequent columns are short
//! names for that path.
//!
//! # TODO
//!
//! * [ ] Support database file specification at runtime, via args or env.
//! * [ ] Support complex expansions like "yesterday's date."
//! * [ ] Add DB path list to error messages about missing or empty targets.

use std::io::Write;
use std::os::unix::ffi::OsStrExt;
use std::path::{Component, Path, PathBuf};
use std::process::ExitCode;
use std::{env, fmt, io};

use jump::{cmd, db};

#[derive(Debug)]
enum TargetErrorKind {
    NotFound,
    Empty,
}

impl fmt::Display for TargetErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "No such target"),
            Self::Empty => write!(f, "Empty target"),
        }
    }
}

#[derive(Debug)]
struct TargetError {
    arg: String,
    kind: TargetErrorKind,
}

impl TargetError {
    fn new(arg: impl Into<String>, kind: TargetErrorKind) -> Self {
        Self {
            arg: arg.into(),
            kind,
        }
    }
}

#[derive(Debug)]
enum Error {
    Db(db::Error),
    Target(TargetError),
}

impl Error {
    fn target_not_found(arg: impl Into<String>) -> Error {
        TargetError::new(arg, TargetErrorKind::NotFound).into()
    }

    fn target_empty(arg: impl Into<String>) -> Error {
        TargetError::new(arg, TargetErrorKind::Empty).into()
    }
}

impl std::error::Error for Error {}

impl From<db::Error> for Error {
    fn from(value: db::Error) -> Self {
        Self::Db(value)
    }
}

impl From<TargetError> for Error {
    fn from(value: TargetError) -> Self {
        Self::Target(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Db(e) => e.fmt(f),
            Self::Target(TargetError { arg, kind }) => write!(f, "{arg}: {kind}"),
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

trait AsBytes {
    fn as_bytes(&self) -> &[u8];
}

impl<const N: usize> AsBytes for [u8; N] {
    fn as_bytes(&self) -> &[u8] {
        self
    }
}

/// # Implementation note
///
/// We can't `impl<P: AsRef<Path>> AsBytes for P`, because, according to rustc:
///
/// > conflicting implementations of trait `AsBytes` for type `[u8; _]`
/// >
/// > upstream crates may add a new impl of trait `std::convert::AsRef<std::path::Path>` for type
/// > `[u8; _]` in future versions
///
/// In other words:  We can't have both a specific implementation for arrays, and a blank
/// implementation for paths, because _some future version_ of the upstream implementation might
/// make arrays [`AsRef<Path>`]. But neither can we remove the specific implementation for arrays,
/// because no such expansion has yet taken place.  So, we can't have nice things; viz. a blanket
/// implementation for [`Path`], [`PathBuf`], and [`Component`], as well as an implementation (blank
/// or otherwise) for arrays.  (The same applies to slices, in case you were wondering.)
impl AsBytes for Path {
    fn as_bytes(&self) -> &[u8] {
        self.as_os_str().as_bytes()
    }
}

impl AsBytes for PathBuf {
    fn as_bytes(&self) -> &[u8] {
        self.as_path().as_bytes()
    }
}

impl AsBytes for Component<'_> {
    fn as_bytes(&self) -> &[u8] {
        self.as_os_str().as_bytes()
    }
}

macro_rules! write_command_line {
    ($command:ident, $($arg:expr),*) => {{
        let mut stdout = io::stdout();
        stdout.write_all(cmd::$command.as_bytes()).unwrap();
        stdout.write_all(b" ").unwrap();
        $(
            stdout.write_all($arg.as_bytes()).unwrap();
        )*
        stdout.write_all(b"\n").unwrap();
    }};
}

fn main_imp() -> Result<()> {
    #[allow(deprecated)]
    let home = env::home_dir().expect("user should have a home directory");
    let db = db::Database::read_file(home.join(".config/jump/targets.csv"))?;
    let expander = jump::Expander::with_home(&home);

    for arg in env::args().skip(1) {
        let path = db.get(&arg).ok_or_else(|| Error::target_not_found(&arg))?;
        let path = expander.expand(path);

        let mut parts = path.components();
        let first = parts.next().ok_or(Error::target_empty(arg))?;

        if let Some("http:" | "https:") = first.as_os_str().to_str() {
            write_command_line!(OPEN, first, b"//", parts.collect::<PathBuf>());
        } else {
            write_command_line!(CD, path);
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
