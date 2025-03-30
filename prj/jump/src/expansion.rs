use std::convert::Infallible;
use std::ffi::OsStr;
use std::path::{Component, Path, PathBuf};

/// Expansion currently panics on error, but future versions will replace instead return [`Err`].
pub type Error = Infallible;
pub type Result<T> = std::result::Result<T, Error>;

enum Expansion<'a, 'b> {
    Path(&'a Path),
    Component(Component<'b>),
    String(String),
}

impl AsRef<Path> for Expansion<'_, '_> {
    fn as_ref(&self) -> &Path {
        match self {
            Self::Path(p) => p,
            Self::Component(c) => c.as_ref(),
            Self::String(s) => Path::new(s),
        }
    }
}

fn get_normal(part: Component) -> Option<&OsStr> {
    match part {
        Component::Normal(s) => Some(s),
        _ => None,
    }
}

pub struct Expand<'a> {
    home: &'a Path,
}

impl<'a> Expand<'a> {
    pub fn with_home(home: &'a Path) -> Self {
        Self { home }
    }

    fn special<'b>(&self, s: &str) -> Option<Expansion<'a, 'b>> {
        if s.starts_with('%') {
            let today = chrono::Local::now().date_naive();
            Some(Expansion::String(today.format(s).to_string()))
        } else if s == "~" {
            Some(Expansion::Path(self.home))
        } else {
            None
        }
    }

    fn component<'b>(&self, part: Component<'b>) -> Expansion<'a, 'b> {
        get_normal(part)
            .and_then(OsStr::to_str)
            .and_then(|s| self.special(s))
            .unwrap_or(Expansion::Component(part))
    }

    /// # Panics
    ///
    /// Panics if the component begins with `%`, but is not a valid `strftime` format string. The
    /// panic is because the underlying [`chrono::NaiveDate::format`] works lazily, with the actual
    /// formatting done by [`chrono::format::DelayedFormat::to_string`], which has no good to way to
    /// report an error.
    ///
    /// # TODO
    ///
    /// Avoid the panic.  See the [`chrono` implementation][1].
    ///
    /// [1]: https://docs.rs/chrono/latest/src/chrono/format/formatting.rs.html#335
    pub fn path(&self, path: &Path) -> Result<PathBuf> {
        Ok(path.components().map(|c| self.component(c)).collect())
    }
}
