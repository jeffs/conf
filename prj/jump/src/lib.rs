mod as_bytes;
mod error;
mod expansion;

pub mod db;

use std::env;
use std::path::{Path, PathBuf};

pub use db::Database;
pub use error::Error;
pub use expansion::Expand;

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

pub struct App {
    home: PathBuf,
    db: Database,
}

impl App {
    /// # Panics
    ///
    /// Panics if [`env::home_dir`] returns [`Err`].
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if the target database cannot be read.
    pub fn from_env() -> Result<App> {
        let home = env::home_dir().expect("user should have a home directory");
        let db = db_from_env(&home)?;
        Ok(App { home, db })
    }

    fn target(&self, target: &str) -> Result<&PathBuf> {
        self.db
            .get(target)
            .ok_or_else(|| Error::Target(target.to_owned()))
    }

    /// Expands the specified target.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if the target cannot be mapped to a path.
    pub fn path(&self, target: &str) -> Result<PathBuf> {
        let expand = Expand::with_home(&self.home);
        Ok(expand.path(self.target(target)?)?)
    }

    /// Returns a shell comamnd for jumping to the specified target.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if the target cannot be mapped to a command.
    pub fn command(&self, target: &str) -> Result<Vec<u8>> {
        let expand = Expand::with_home(&self.home);
        Ok(expand.command(self.target(target)?)?)
    }
}
