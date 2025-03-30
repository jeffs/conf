mod as_bytes;
mod error;
mod expansion;

pub mod db;

use std::env;
use std::path::PathBuf;

pub use db::Database;
pub use error::Error;
pub use expansion::Expand;

pub type Result<T> = std::result::Result<T, Error>;

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
        // The [`std::env::home_dir`] function is deprecated because it behaved inconsistently on
        // Windows before Rust 1.85, but it does what we want here.
        #[allow(deprecated)]
        let home = env::home_dir().expect("user should have a home directory");
        let db = Database::read_file(home.join(".config/jump/targets.csv"))?;
        Ok(App { home, db })
    }

    /// Returns a shell comamnd for jumping to the specified target.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if the target cannot be mapped to a command.
    pub fn jump(&self, target: &str) -> Result<Vec<u8>> {
        let expand = Expand::with_home(&self.home);
        let path = self
            .db
            .get(target)
            .ok_or_else(|| Error::Target(target.to_owned()))?;
        let path = expand.path(path)?;
        Ok(expand.command(&path)?)
    }
}
