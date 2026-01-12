use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fmt, fs, io};

// Note: Database stores raw target values as String (not PathBuf) to support
// URLs and arbitrary strings. Type detection happens at resolve time.

/// A single key or list of keys in YAML format.
#[derive(Deserialize)]
#[serde(untagged)]
enum Keys {
    One(String),
    Many(Vec<String>),
}

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
    Yaml(serde_saphyr::Error),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::Yaml(e) => e.fmt(f),
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
    pub fn yaml(file: PathBuf, cause: serde_saphyr::Error) -> Self {
        Self::new(file, ErrorKind::Yaml(cause))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.location, self.kind)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Database(
    /// Maps target names to raw target values (paths, URLs, or arbitrary strings).
    HashMap<String, String>,
);

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

impl Database {
    #[must_use]
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// # Errors
    ///
    /// Returns an error if the file cannot be read, or if its syntax is
    /// invalid.
    pub fn read_file(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        let contents = fs::read_to_string(path).map_err(|e| Error::io(path.into(), e))?;

        let yaml: HashMap<String, Keys> =
            serde_saphyr::from_str(&contents).map_err(|e| Error::yaml(path.into(), e))?;

        for (value, keys) in yaml {
            match keys {
                Keys::One(key) => {
                    self.0.insert(key, value);
                }
                Keys::Many(keys) => {
                    for key in keys {
                        self.0.insert(key, value.clone());
                    }
                }
            }
        }

        Ok(())
    }

    #[must_use]
    pub fn get(&self, name: &str) -> Option<&String> {
        self.0.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn parse(yaml: &str) -> Database {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        file.write_all(yaml.as_bytes()).unwrap();
        let mut db = Database::new();
        db.read_file(file.path()).unwrap();
        db
    }

    #[test]
    fn single_key() {
        let db = parse("~/conf: c\n");
        assert_eq!(db.get("c"), Some(&"~/conf".into()));
    }

    #[test]
    fn multiple_keys() {
        let db = parse("~/conf: [c, conf]\n");
        assert_eq!(db.get("c"), Some(&"~/conf".into()));
        assert_eq!(db.get("conf"), Some(&"~/conf".into()));
    }

    #[test]
    fn quoted_value() {
        let db = parse("\"value with spaces\": key\n");
        assert_eq!(db.get("key"), Some(&"value with spaces".into()));
    }

    #[test]
    fn quoted_key() {
        let db = parse("~/path: \"key with spaces\"\n");
        assert_eq!(db.get("key with spaces"), Some(&"~/path".into()));
    }

    #[test]
    fn quoted_key_in_list() {
        let db = parse("~/path: [simple, \"key with spaces\"]\n");
        assert_eq!(db.get("simple"), Some(&"~/path".into()));
        assert_eq!(db.get("key with spaces"), Some(&"~/path".into()));
    }

    #[test]
    fn comments_ignored() {
        let db = parse("# comment\n~/conf: c\n");
        assert_eq!(db.get("c"), Some(&"~/conf".into()));
    }

    #[test]
    fn blank_lines_ignored() {
        let db = parse("\n~/conf: c\n\n");
        assert_eq!(db.get("c"), Some(&"~/conf".into()));
    }

    #[test]
    fn url_value() {
        let db = parse("https://example.com: ex\n");
        assert_eq!(db.get("ex"), Some(&"https://example.com".into()));
    }
}
