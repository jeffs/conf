use std::fmt;
use std::path::PathBuf;

use crate::{db, expansion};

#[derive(Debug)]
pub enum Error {
    /// A config file could not be parsed.
    Config(PathBuf, Box<serde_saphyr::Error>),
    /// An error ocurred loading a database.
    Database(db::Error),
    /// An error occurred expanding a path.
    Expansion(expansion::Error),
    /// No target was specified, and no default found.
    Missing,
    /// No target was found for the argument.
    Target {
        name: String,
        searched: Vec<PathBuf>,
    },
}

impl std::error::Error for Error {}

impl From<db::Error> for Error {
    fn from(value: db::Error) -> Self {
        Self::Database(value)
    }
}

impl From<expansion::Error> for Error {
    fn from(value: expansion::Error) -> Self {
        Self::Expansion(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Config(p, e) => write!(f, "{}: {e}", p.display()),
            Self::Database(e) => e.fmt(f),
            Self::Expansion(e) => e.fmt(f),
            Self::Missing => "no default target is configured".fmt(f),
            Self::Target { name, searched } => {
                write!(f, "{name}: target not found; searched:")?;
                for path in searched {
                    write!(f, "\n  {}", path.display())?;
                }
                Ok(())
            }
        }
    }
}
