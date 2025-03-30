use std::fmt;

use crate::{db, expansion};

#[derive(Debug)]
pub enum Error {
    /// An error ocurred loading a database.
    Database(db::Error),
    /// An error occurred expanding a path.
    Expansion(expansion::Error),
    /// No target was found for the argument.
    Target(String),
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
            Self::Database(e) => e.fmt(f),
            Self::Expansion(e) => e.fmt(f),
            Self::Target(arg) => write!(f, "{arg}: Target not found"),
        }
    }
}
