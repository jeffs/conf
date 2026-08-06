//! The failure type this program reports.

use std::fmt;

/// Everything that can end a sync early, one variant per kind.
pub enum Error {
    /// The command line held something other than a subcommand.
    Argument(String),
    /// jj, git, or the repository's own files refused the work.
    Kit(jjkit::Error),
    /// `gh pr list` printed unreadable JSON.
    Json(serde_json::Error),
    /// No remote is named `origin`, and there is not exactly one.
    NoOrigin,
    /// The remote URL fits no recognized git URL form.
    RemoteUrl(String),
}

impl Error {
    /// The code for main to end the process with: a failed child's own exit
    /// code where there is one, otherwise 1.
    pub fn code(&self) -> i32 {
        match self {
            Self::Kit(error) => error.code(),
            _ => 1,
        }
    }

    /// Captured standard error awaiting forwarding; empty for every other kind
    /// of error.
    pub fn stderr(&self) -> &[u8] {
        match self {
            Self::Kit(error) => error.stderr(),
            _ => &[],
        }
    }
}

impl From<jjkit::Error> for Error {
    fn from(error: jjkit::Error) -> Self {
        Self::Kit(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Argument(arg) => write!(f, "{arg}: unexpected argument\n{}", crate::USAGE),
            Self::Kit(error) => error.fmt(f),
            Self::Json(source) => write!(f, "reading pull requests: {source}"),
            Self::NoOrigin => write!(f, "expected an origin remote"),
            Self::RemoteUrl(url) => write!(f, "{url}: unsupported remote URL"),
        }
    }
}
