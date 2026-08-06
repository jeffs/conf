//! The failure type every module here reports.

use std::{
    fmt, io,
    path::{Path, PathBuf},
    process::ExitStatus,
    string::FromUtf8Error,
};

/// Everything that can end a sync early, one variant per kind.
#[derive(Debug)]
pub enum Error {
    /// The command line held something other than a subcommand.
    Argument(String),
    /// A child process could not be started or awaited.
    Io { program: String, source: io::Error },
    /// A child process ran and failed.  Its standard error rides along when it
    /// was captured rather than inherited.
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
    /// A child process wrote something this program cannot read.
    Parse { program: String, text: String },
    /// A file could not be read or written.
    File { path: PathBuf, source: io::Error },
    /// A path this program would write already holds something else.
    Occupied(PathBuf),
    /// A git worktree of the name one workspace wants is registered to
    /// another, which shares its directory name.
    Registered { admin: PathBuf, other: PathBuf },
    /// A workspace root that names no directory, and so no worktree.
    Unnamed(PathBuf),
    /// `gh pr list` printed unreadable JSON.
    Json(serde_json::Error),
    /// No remote is named `origin`, and there is not exactly one.
    NoOrigin,
    /// The remote URL fits no recognized git URL form.
    RemoteUrl(String),
}

impl Error {
    /// Report failing to read or write `path`.
    pub fn file(path: &Path, source: io::Error) -> Self {
        Self::File {
            path: path.to_owned(),
            source,
        }
    }

    /// The code for main to end the process with: a failed child's own exit
    /// code where there is one, otherwise 1.
    pub fn code(&self) -> i32 {
        match self {
            Self::Exit { status, .. } => status.code().unwrap_or(1),
            _ => 1,
        }
    }

    /// Captured standard error awaiting forwarding; empty for every other kind
    /// of error.
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
            Self::Argument(arg) => write!(f, "{arg}: unexpected argument\n{}", crate::USAGE),
            Self::Io { program, source } => write!(f, "running {program}: {source}"),
            Self::Exit {
                program, status, ..
            } => write!(f, "{program}: {status}"),
            Self::Utf8 { program, source } => write!(f, "{program} wrote invalid UTF-8: {source}"),
            Self::Parse { program, text } => write!(f, "{program} wrote unreadable output: {text}"),
            Self::File { path, source } => write!(f, "{}: {source}", path.display()),
            Self::Occupied(path) => write!(f, "{}: already in use", path.display()),
            Self::Registered { admin, other } => {
                write!(f, "{}: registered to {}", admin.display(), other.display())
            }
            Self::Unnamed(path) => write!(f, "{}: names no directory", path.display()),
            Self::Json(source) => write!(f, "reading pull requests: {source}"),
            Self::NoOrigin => write!(f, "expected an origin remote"),
            Self::RemoteUrl(url) => write!(f, "{url}: unsupported remote URL"),
        }
    }
}
