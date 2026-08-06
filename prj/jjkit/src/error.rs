//! The failure type every module here reports.

use std::{
    fmt, io,
    path::{Path, PathBuf},
    process::ExitStatus,
    string::FromUtf8Error,
};

/// Everything that can end a wrapper's work early, one variant per kind.
#[derive(Debug)]
pub enum Error {
    /// A child process could not be started or awaited.
    Io { program: String, source: io::Error },
    /// A child process ran and failed.  Its standard error rides along when
    /// it was captured rather than inherited.
    Exit {
        program: String,
        status: ExitStatus,
        stderr: Vec<u8>,
    },
    /// A child process wrote something other than UTF-8.
    Utf8 {
        program: String,
        source: FromUtf8Error,
    },
    /// A child process wrote something this crate cannot read.
    Parse { program: String, text: String },
    /// A file could not be read or written.
    File { path: PathBuf, source: io::Error },
    /// A path this crate would write already holds something else.
    Occupied(PathBuf),
    /// A git worktree of the name one workspace wants is registered to
    /// another, which shares its directory name.
    Registered { admin: PathBuf, other: PathBuf },
    /// A workspace root that names no directory, and so no worktree.
    Unnamed(PathBuf),
}

impl Error {
    /// Report failing to read or write `path`.
    #[must_use]
    pub fn file(path: &Path, source: io::Error) -> Self {
        Self::File {
            path: path.to_owned(),
            source,
        }
    }

    /// The code for a `main` to end the process with: a failed child's own
    /// exit code where there is one, otherwise 1.
    #[must_use]
    pub fn code(&self) -> i32 {
        match self {
            Self::Exit { status, .. } => status.code().unwrap_or(1),
            _ => 1,
        }
    }

    /// Captured standard error awaiting forwarding; empty for every other
    /// kind of error.
    #[must_use]
    pub fn stderr(&self) -> &[u8] {
        match self {
            Self::Exit { stderr, .. } => stderr,
            _ => &[],
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { program, source } => write!(f, "running {program}: {source}"),
            Self::Exit {
                program, status, ..
            } => write!(f, "{program}: {status}"),
            Self::Utf8 { program, source } => write!(f, "{program} wrote invalid UTF-8: {source}"),
            Self::Parse { program, text } => write!(f, "{program} wrote unreadable output: {text}"),
            Self::File { path, source } => write!(f, "{}: {source}", path.display()),
            Self::Occupied(path) => write!(f, "{}: already in use", path.display()),
            Self::Registered { admin, other } => write!(
                f,
                "{}: registered to {}",
                admin.display(),
                other.display()
            ),
            Self::Unnamed(path) => write!(f, "{}: names no directory", path.display()),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } | Self::File { source, .. } => Some(source),
            Self::Utf8 { source, .. } => Some(source),
            _ => None,
        }
    }
}
