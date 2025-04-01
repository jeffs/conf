use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fmt, fs, io};

#[derive(Debug)]
pub struct Location {
    pub file: PathBuf,
    pub line: Option<usize>,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.file.display().fmt(f)?;
        if let Some(line) = self.line {
            write!(f, ":{line}")?;
        }
        Ok(())
    }
}

trait IntoLocation {
    fn into_location(self) -> Location;
}

impl IntoLocation for Location {
    fn into_location(self) -> Location {
        self
    }
}

impl IntoLocation for PathBuf {
    fn into_location(self) -> Location {
        Location {
            file: self,
            line: None,
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Io(io::Error),
    Syntax,
    Duplicate(String),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::Syntax => write!(f, "Syntax error"),
            // TODO: XXX Overide instead of returning an error
            Self::Duplicate(s) => write!(f, "Duplicate entry for {s}"),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    location: Location,
    kind: ErrorKind,
}

impl Error {
    fn new(location: impl IntoLocation, kind: ErrorKind) -> Self {
        let location = location.into_location();
        Self { location, kind }
    }

    #[must_use]
    pub fn io(file: PathBuf, cause: io::Error) -> Self {
        Self::new(file, ErrorKind::Io(cause))
    }

    #[must_use]
    pub fn syntax(location: Location) -> Self {
        Self::new(location, ErrorKind::Syntax)
    }

    #[must_use]
    pub fn duplicate(location: Location, name: String) -> Self {
        Self::new(location, ErrorKind::Duplicate(name))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.location, self.kind)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Database(
    /// Maps jump target names to directory paths.
    HashMap<String, PathBuf>,
);

impl Database {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// # Errors
    ///
    /// Returns an error if the file cannot be read, or if its syntax is invalid.
    pub fn read_file(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        let file = fs::read_to_string(path).map_err(|e| Error::io(path.into(), e))?;

        for (index, line) in file.lines().enumerate() {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let location = || Location {
                file: path.to_path_buf(),
                line: Some(index + 1),
            };

            let (dir, names) = line
                .split_once(',')
                .ok_or_else(|| Error::syntax(location()))?;

            for name in names.split(',') {
                // Overwrite any existing entry.
                self.0.insert(name.into(), dir.into());
            }
        }

        Ok(())
    }

    #[must_use]
    pub fn get(&self, name: &str) -> Option<&PathBuf> {
        self.0.get(name)
    }
}
