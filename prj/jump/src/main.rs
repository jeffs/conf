//! # Notes
//!
//! Reads config from `~/.config/jump/targets.csv`, where `~` is returned by [`std::env::home_dir`].
//! That function is deprecated because it behaved inconsistently on Windows before Rust 1.85, but
//! it does what we want here.
//!
//! The `targets.csv` file supports blank lines, comment lines (beginning with `#`), and jagged
//! lines.  The first column in each row is a directory path, and all subsequent columns are short
//! names for that path.
//!
//! # TODO
//!
//! * [ ] Support database file specification at runtime, via args or env.
//! * [ ] Support complex expansions like "yesterday's date."

use std::{env, fmt};

use jump::{cmd, db};

#[derive(Debug)]
enum Error {
    Db(db::Error),
}

impl From<db::Error> for Error {
    fn from(value: db::Error) -> Self {
        Self::Db(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Db(e) => e.fmt(f),
        }
    }
}

fn main_imp() -> Result<(), db::Error> {
    #[allow(deprecated)]
    let home = env::home_dir().expect("user should have a home directory");
    let db_path = home.join(".config/jump/targets.csv");
    let db = db::Database::read_file(&db_path)?;

    let expander = jump::Expander::with_home(&home);
    for arg in env::args().skip(1) {
        let Some(path) = db.get(&arg) else {
            return Err(db::Error::arg(db_path, arg));
        };

        let buf = expander.expand(path);
        let command = if buf.starts_with("http://") || buf.starts_with("https://") {
            cmd::OPEN
        } else {
            cmd::CD
        };

        println!("{command} {}", buf.display());
    }

    Ok(())
}

fn main() {
    if let Err(e) = main_imp() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
