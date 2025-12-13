//! Target expansion and type detection for jump targets.
//!
//! Supports three target types:
//! - URLs (`http://`, `https://`) - output verbatim
//! - Paths (`/`, `~`, `$`, `%`) - expanded with variable substitution
//! - Arbitrary strings - output verbatim

use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::{Component, Path, PathBuf};
use std::{env, fmt};

#[derive(Debug)]
pub enum Error {
    /// An expanded target was empty.
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

/// Represents the resolved value of a jump target lookup.
#[derive(Debug)]
pub enum Target {
    /// A filesystem path (expanded from `~`, `$VAR`, `%date`, or absolute paths).
    Path(PathBuf),
    /// A URL or arbitrary string (output verbatim).
    String(String),
}

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

pub struct Expand<'a> {
    home: &'a Path,
}

impl<'a> Expand<'a> {
    #[must_use]
    pub fn with_home(home: &'a Path) -> Self {
        Self { home }
    }

    fn special<'b>(&self, s: &OsStr) -> Result<Option<Expansion<'a, 'b>>> {
        let bytes = s.as_bytes();

        // Handle $ at byte level to support non-UTF-8 variable names.
        if bytes.first() == Some(&b'$') && bytes.len() > 1 {
            let var = OsStr::from_bytes(&bytes[1..]);
            let part = env::var_os(var).ok_or(Error::Unset)?;
            return Ok(Some(Expansion::PathBuf(part.into())));
        }

        // Other special expansions (%, ~) require UTF-8 values.
        //
        // TODO: Support non-UTF-8 values in `~` expansion.
        let Some(s) = s.to_str() else {
            return Ok(None);
        };
        Ok(if s.starts_with('%') {
            let today = chrono::Local::now().date_naive();
            Some(Expansion::String(today.format(s).to_string()))
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

    /// Expands a path by substituting `~`, `$VAR`, and `%date` patterns.
    ///
    /// # Panics
    ///
    /// Panics if a component begins with `%`, but is not a valid `strftime`
    /// format string. The panic is because the underlying
    /// [`chrono::NaiveDate::format`] works lazily, with the actual
    /// formatting done by [`chrono::format::DelayedFormat::to_string`], which
    /// has no good way to report an error.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Empty`] if the path is empty after expansion, or
    /// [`Error::Unset`] if an environment variable is not set.
    ///
    /// # TODO
    ///
    /// Avoid the panic. See the [`chrono` implementation][1].
    ///
    /// [1]: https://docs.rs/chrono/latest/src/chrono/format/formatting.rs.html#335
    pub fn path(&self, path: &Path) -> Result<PathBuf> {
        let parts = path
            .components()
            .map(|c| self.component(c))
            .collect::<Result<Vec<_>>>()?;
        if parts.is_empty() {
            return Err(Error::Empty);
        }
        Ok(parts.iter().map(AsRef::as_ref).collect())
    }

    /// Expands the value to an inferred target type.
    ///
    /// # Panics
    ///
    /// Panics if value is inferred to be a path containing an invalid
    /// `strftime` format string; see [`Self::path`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::Empty`] if `value` is empty. Returns [`Error::Unset`]
    /// if value is inferred to be a path, and contains any unset environment
    /// variable; e.g., `/$NONESUCH`.
    pub fn target(&self, value: &str) -> Result<Target> {
        if value.is_empty() {
            Err(Error::Empty)
        } else if value.starts_with(['/', '~', '$', '%']) {
            Ok(Target::Path(self.path(Path::new(value))?))
        } else {
            Ok(Target::String(value.to_owned()))
        }
    }
}
