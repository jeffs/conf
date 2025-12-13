mod error;
mod expansion;

pub mod db;

use std::env;
use std::path::{Path, PathBuf};

pub use db::Database;
pub use error::Error;
pub use expansion::Expand;

/// Represents the resolved value of a jump target lookup.
#[derive(Debug)]
pub enum Target {
    /// A filesystem path (expanded from `~`, `$VAR`, `%date`, or absolute paths).
    Path(PathBuf),
    /// A URL or arbitrary string (output verbatim).
    String(String),
}

pub type Result<T> = std::result::Result<T, Error>;

fn db_from_env(home: &Path) -> Result<Database> {
    let mut prefixes = env::var("JUMP_PREFIXES")
        .map(|s| s.split(':').map(str::to_owned).collect::<Vec<_>>())
        .unwrap_or_default()
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .collect::<Vec<_>>();

    if prefixes.is_empty() {
        prefixes.push(home.join(".config/jump"));
    }

    let mut db = Database::new();
    for prefix in prefixes {
        db.read_file(prefix.join("targets.csv"))?;
    }
    Ok(db)
}

/// Maps target names to paths from a [`Database`].
pub struct App {
    home: PathBuf,
    db: Database,
}

impl App {
    /// Returns an app that reads from all `PREFIX/targets.csv` files,
    /// where `PREFIX` is each path in the `JUMP_PREFIXES` environment
    /// variable. If `JUMP_PREFIXES` is empty or unset, reads from
    /// `~/.config/jump/targets.csv`.
    ///
    /// # Panics
    ///
    /// Panics if [`env::home_dir`] returns [`Err`].
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if the target database cannot be read.
    ///
    /// # TODO
    ///
    /// Respect `XDG_CONFIG_HOME`.
    pub fn from_env() -> Result<App> {
        let home = env::home_dir().expect("user should have a home directory");
        let db = db_from_env(&home)?;
        Ok(App { home, db })
    }

    /// # Errors
    ///
    /// Returns [`Error::Target`] if the target is not in this app's database.
    fn target(&self, target: &str) -> Result<&String> {
        self.db
            .get(target)
            .ok_or_else(|| Error::Target(target.to_owned()))
    }

    /// Looks up the specified target in this app's database and resolves it
    /// to a [`Target`] value.
    ///
    /// The resolved value depends on the target type:
    /// - URLs (`http://`, `https://`) → `Target::String` (verbatim)
    /// - Paths (`/`, `~`, `$`, `%`) → `Target::Path` (expanded)
    /// - Everything else → `Target::String` (verbatim)
    ///
    /// If the target is not found, but ends with a slash character (`'/'`),
    /// lookup is also attempted without the trailing slash, in case the user's
    /// shell tab-completed a directory that happened to have the same name as
    /// the target.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if the target cannot be found or resolved.
    pub fn resolve(&self, target: &str) -> Result<Target> {
        let value = self.target(target).or_else(|err| {
            target
                .strip_suffix('/')
                .and_then(|target| self.target(target).ok())
                .ok_or(err)
        })?;
        Ok(Expand::with_home(&self.home).target(value)?)
    }
}
