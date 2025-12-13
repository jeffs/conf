mod error;
mod expansion;

pub mod db;

use std::env;
use std::path::{Path, PathBuf};

pub use db::Database;
pub use error::Error;
pub use expansion::{Expand, Target};

pub type Result<T> = std::result::Result<T, Error>;

fn db_from_env(home: &Path) -> Result<(Database, Vec<PathBuf>)> {
    let mut prefixes = env::var("JUMP_PREFIXES")
        .map(|s| s.split(':').map(str::to_owned).collect::<Vec<_>>())
        .unwrap_or_default()
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .collect::<Vec<_>>();

    if prefixes.is_empty() {
        let config_home =
            env::var_os("XDG_CONFIG_HOME").map_or_else(|| home.join(".config"), PathBuf::from);
        prefixes.push(config_home.join("jump"));
    }

    let mut db = Database::new();
    for prefix in &prefixes {
        db.read_file(prefix.join("targets.csv"))?;
    }
    Ok((db, paths))
}

/// Maps target names to paths from a [`Database`].
pub struct App {
    home: PathBuf,
    db: Database,
    db_paths: Vec<PathBuf>,
}

impl App {
    /// Returns an app that reads from all `PREFIX/targets.csv` files,
    /// where `PREFIX` is each path in the `JUMP_PREFIXES` environment
    /// variable. If `JUMP_PREFIXES` is empty or unset, reads from
    /// `$XDG_CONFIG_HOME/jump/targets.csv` (defaulting to `~/.config/jump`).
    ///
    /// # Panics
    ///
    /// Panics if [`env::home_dir`] returns [`Err`].
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if the target database cannot be read.
    pub fn from_env() -> Result<App> {
        let home = env::home_dir().expect("user should have a home directory");
        let (db, db_paths) = db_from_env(&home)?;
        Ok(App { home, db, db_paths })
    }

    /// # Errors
    ///
    /// Returns [`Error::Target`] if the target is not in this app's database.
    fn target(&self, target: &str) -> Result<&String> {
        self.db.get(target).ok_or_else(|| Error::Target {
            name: target.to_owned(),
            searched: self.db_paths.clone(),
        })
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
