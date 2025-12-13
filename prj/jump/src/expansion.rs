//! Target expansion and type detection for jump targets.
//!
//! Supports three target types:
//! - URLs (`http://`, `https://`) - output verbatim
//! - Paths (`/`, `~`, `$`, `%`) - expanded with variable substitution
//! - Arbitrary strings - output verbatim

use std::ffi::OsStr;
use std::path::{Component, Path, PathBuf};
use std::{env, fmt};

/// Returns `true` if the value is a URL (starts with `http://` or `https://`).
#[must_use]
pub fn is_url_target(s: &str) -> bool {
    s.starts_with("http://") || s.starts_with("https://")
}

/// Returns `true` if the value should be treated as a path and expanded.
///
/// Path targets start with `/`, `~`, `$`, or `%`.
#[must_use]
pub fn is_path_target(s: &str) -> bool {
    s.starts_with('/') || s.starts_with('~') || s.starts_with('$') || s.starts_with('%')
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

    /// Returns a [`Target`] for the given value.
    ///
    /// Detection order:
    /// 1. URLs (`http://`, `https://`) → `Target::String` (verbatim)
    /// 2. Paths (`/`, `~`, `$`, `%`) → `Target::Path` (expanded)
    /// 3. Everything else → `Target::String` (verbatim)
    ///
    /// # Errors
    ///
    /// Returns [`Error::Empty`] if a path target expands to empty, or
    /// [`Error::Unset`] if an environment variable is not set.
    pub fn target(&self, value: &str) -> Result<crate::Target> {
        if is_url_target(value) {
            return Ok(crate::Target::String(value.to_owned()));
        }
        if is_path_target(value) {
            return Ok(crate::Target::Path(self.path(Path::new(value))?));
        }
        Ok(crate::Target::String(value.to_owned()))
    }
}
