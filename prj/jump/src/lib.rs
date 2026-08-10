mod error;
mod expansion;

pub mod db;

use std::path::{Path, PathBuf};
use std::{env, fs};

pub use db::Database;
pub use error::Error;
pub use expansion::{Expand, Target};

pub type Result<T> = std::result::Result<T, Error>;

/// Returns `$XDG_CONFIG_HOME` if set, and `~/.config` otherwise.
fn config_home(home: &Path) -> PathBuf {
    env::var_os("XDG_CONFIG_HOME").map_or_else(|| home.join(".config"), PathBuf::from)
}

fn dirs_from_env(home: &Path) -> Vec<PathBuf> {
    let string = env::var_os("JUMP_DIRS").unwrap_or_default();
    let paths = env::split_paths(&string)
        .filter(|p| p != Path::new(""))
        .collect::<Vec<_>>();
    if paths.is_empty() {
        return vec![config_home(home)];
    }
    paths
}

/// Returns the accumulated database, and a list of paths loaded (for use in
/// error messages).
fn db_from_env(home: &Path) -> Result<(Database, Vec<PathBuf>)> {
    let dirs = dirs_from_env(home);
    let paths = dirs.iter().map(|p| p.join("jump.yaml")).collect::<Vec<_>>();
    let mut db = Database::new();
    for path in &paths {
        db.read_file(path)?;
    }
    Ok((db, paths))
}

/// Splits a partially typed suffix into the leading components, which are
/// complete, and the final component, which is not.
fn split_suffix(suffix: &str) -> (&str, &str) {
    match suffix.rfind('/') {
        Some(i) => suffix.split_at(i + 1),
        None => ("", suffix),
    }
}

/// Returns the names in `dir` that begin with `prefix`, each preceded by
/// `parent` and followed by `/` if it names a directory. Hidden names are
/// omitted unless `prefix` itself begins with `.`.
fn entries(dir: &Path, parent: &str, prefix: &str) -> Vec<String> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let name = entry.file_name().into_string().ok()?;
            if !name.starts_with(prefix) || (name.starts_with('.') && !prefix.starts_with('.')) {
                return None;
            }
            let separator = if entry.file_type().is_ok_and(|t| t.is_dir()) {
                "/"
            } else {
                ""
            };
            Some(format!("{parent}{name}{separator}"))
        })
        .collect()
}

/// Maps target names to paths from a [`Database`].
pub struct App {
    home: PathBuf,
    db: Database,
    db_paths: Vec<PathBuf>,
}

impl App {
    /// Returns an app that reads from all `DIR/jump.yaml` files, where `DIR`
    /// is each path in the `JUMP_DIRS` environment variable. If `JUMP_DIRS` is
    /// empty or unset, reads from `$XDG_CONFIG_HOME/jump.yaml` (defaulting to
    /// `~/.config/jump.yaml`).
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

    /// Returns the target names that begin with `prefix`. The default target,
    /// whose name is empty, is not a candidate.
    fn targets(&self, prefix: &str) -> Vec<String> {
        self.db
            .names()
            .filter(|name| !name.is_empty() && name.starts_with(prefix))
            .map(str::to_owned)
            .collect()
    }

    /// Returns the paths under `target` that begin with `suffix`. A target
    /// that is not a path, or cannot be resolved, has no candidates.
    fn suffixes(&self, target: &str, suffix: &str) -> Vec<String> {
        let Ok(Target::Path(path)) = self.resolve(target) else {
            return Vec::new();
        };
        let (parent, prefix) = split_suffix(suffix);
        entries(&path.join(parent), parent, prefix)
    }

    /// Returns the completion candidates for the last of `words`, the
    /// arguments typed so far. The first word completes to a target name and
    /// the second to a path under that target; a later word has no
    /// candidates.
    ///
    /// # Panics
    ///
    /// Panics if the first word names a target whose value contains an
    /// invalid `strftime` format string; see [`Expand::path`].
    #[must_use]
    pub fn complete(&self, words: &[String]) -> Vec<String> {
        let mut candidates = match words {
            [] => self.targets(""),
            [target] => self.targets(target),
            [target, suffix] => self.suffixes(target, suffix),
            _ => Vec::new(),
        };
        candidates.sort_unstable();
        candidates
    }
}
