//! TODO: Support URLs, not only paths. The `command` feature probably should go
//!  away, and be replaced by shell-specific bindings.

use std::ffi::OsStr;
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use std::{env, fmt};

use crate::as_bytes::AsBytes;

/// Maps semantic command names (such as `cd`) to their implementation in the
/// calling shell.
///
/// TODO: Read shell commands from config, rather than hard-coding them here.
mod cmd {
    /// Change directory.
    pub const CD: &str = "mc";

    /// Use the OS native file association.
    ///
    /// TODO: Compare macOS `open`, Windows/Nushell `start`, and Linux
    /// `xdg-open`.
    pub const OPEN: &str = "open";
}

#[derive(Debug)]
pub enum Error {
    /// An expanded path was empty.
    Empty,
    /// An environment variable was unset.
    Unset,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "Empty target"),
            Self::Unset => write!(f, "Unset variable"),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

enum Expansion<'a, 'b> {
    Path(&'a Path),
    PathBuf(PathBuf),
    Component(Component<'b>),
    String(String),
}

impl AsRef<Path> for Expansion<'_, '_> {
    fn as_ref(&self) -> &Path {
        match self {
            Self::Path(p) => p,
            Self::PathBuf(p) => p,
            Self::Component(c) => c.as_ref(),
            Self::String(s) => Path::new(s),
        }
    }
}

macro_rules! command {
    ($command:ident, $($arg:expr),*) => {{
        let mut buffer = Vec::new();
        buffer.write_all(cmd::$command.as_bytes()).unwrap();
        buffer.write_all(b" ").unwrap();
        $(
            buffer.write_all($arg.as_bytes()).unwrap();
        )*
        buffer.write_all(b"\n").unwrap();
        Ok(buffer)
    }};
}

pub struct Expand<'a> {
    home: &'a Path,
}

impl<'a> Expand<'a> {
    #[must_use]
    pub fn with_home(home: &'a Path) -> Self {
        Self { home }
    }

    fn special<'b>(&self, s: &OsStr) -> Result<Option<Expansion<'a, 'b>>> {
        let Some(s) = s.to_str() else {
            return Ok(None);
        };
        Ok(if s.starts_with('%') {
            let today = chrono::Local::now().date_naive();
            Some(Expansion::String(today.format(s).to_string()))
        } else if let Some(var) = s.strip_prefix('$') {
            // TODO: Support non-UTF-8 variable names.
            let part = env::var_os(var).ok_or(Error::Unset)?;
            Some(Expansion::PathBuf(part.into()))
        } else if s == "~" {
            Some(Expansion::Path(self.home))
        } else {
            None
        })
    }

    fn component<'b>(&self, part: Component<'b>) -> Result<Expansion<'a, 'b>> {
        // The input may be a "normal" path component (as opposed to, say, a
        // Windows drive specifier), yet be subject to "special" expansion;
        // e.g., `~` or `$VARIABLE`.
        if let Component::Normal(normal) = part
            && let Some(special) = self.special(normal)?
        {
            Ok(special)
        } else {
            Ok(Expansion::Component(part))
        }
    }

    /// # Panics
    ///
    /// Panics if the component begins with `%`, but is not a valid `strftime`
    /// format string. The panic is because the underlying
    /// [`chrono::NaiveDate::format`] works lazily, with the actual
    /// formatting done by [`chrono::format::DelayedFormat::to_string`], which
    /// has no good to way to report an error.
    ///
    /// # Errors
    ///
    /// Path expansion currently panics on error, but future versions will
    /// instead return [`Err`].
    ///
    /// # TODO
    ///
    /// Avoid the panic.  See the [`chrono` implementation][1].
    ///
    /// [1]: https://docs.rs/chrono/latest/src/chrono/format/formatting.rs.html#335
    pub fn path(&self, path: &Path) -> Result<PathBuf> {
        let mut parts = path
            .components()
            .map(|c| self.component(c))
            .collect::<Result<Vec<_>>>()?
            .into_iter();
        let first = parts.next().ok_or(Error::Empty)?;
        if let Some("http:" | "https:") = first.as_ref().to_str() {
            todo!()
        } else {
            todo!()
        }
    }

    /// Returns a snippet of shell script suitable for opening the specified
    /// path.  The path should _not_ already be expanded, as this function
    /// performs path expansion automatically.
    ///
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if the path is empty.
    pub fn command(&self, path: &Path) -> Result<Vec<u8>> {
        let path = self.path(path)?;
        let mut parts = path.components();
        let first = parts.next().ok_or(Error::Empty)?;
        if let Some("http:" | "https:") = first.as_os_str().to_str() {
            command!(OPEN, first, b"//", parts.collect::<PathBuf>())
        } else {
            command!(CD, path)
        }
    }
}
