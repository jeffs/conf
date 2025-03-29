use std::collections::HashMap;
use std::path::{Component, Path, PathBuf};
use std::{env, fmt, fs, io};

/// The name of a command to be executed by the calling shell.
///
/// TODO: Read shell commands from config, rather than hard-coding them here.
mod command {
    /// Change directory.
    pub const CD: &str = "mc";

    /// Use the OS native file association.
    ///
    /// TODO: Compare macOS `open`, Windows `start`, and Linux `xdg-open`.
    pub const OPEN: &str = "open";
}

mod db {
    use super::*;

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
        Arg(String),
    }

    impl fmt::Display for ErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::Io(e) => e.fmt(f),
                Self::Syntax => write!(f, "syntax error"),
                Self::Duplicate(s) => write!(f, "duplicate entry for {s}"),
                Self::Arg(s) => write!(f, "no such target: {s}"),
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

        pub fn io(file: PathBuf, cause: io::Error) -> Self {
            Self::new(file, ErrorKind::Io(cause))
        }

        pub fn syntax(location: Location) -> Self {
            Self::new(location, ErrorKind::Syntax)
        }

        pub fn duplicate(location: Location, name: String) -> Self {
            Self::new(location, ErrorKind::Duplicate(name))
        }

        pub fn arg(file: PathBuf, arg: String) -> Self {
            Self::new(file, ErrorKind::Arg(arg))
        }
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}: {}", self.location, self.kind)
        }
    }

    pub struct Database(
        /// Maps jump target names to directory paths.
        HashMap<String, PathBuf>,
    );

    impl Database {
        pub fn read_file(path: impl AsRef<Path>) -> Result<Self, Error> {
            let path = path.as_ref();
            let file = fs::read_to_string(path).map_err(|e| Error::io(path.into(), e))?;

            let mut jumps = HashMap::new();
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
                    if jumps.insert(name.into(), dir.into()).is_some() {
                        return Err(Error::duplicate(location(), name.into()));
                    }
                }
            }

            Ok(Database(jumps))
        }

        pub fn get(&self, name: &str) -> Option<&PathBuf> {
            self.0.get(name)
        }
    }
}

#[derive(Debug)]
enum Error {
    Db(db::Error),
}

impl From<db::Error> for Error {
    fn from(e: db::Error) -> Self {
        Self::Db(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Db(e) => e.fmt(f),
        }
    }
}

enum Expansion<'a, 'b> {
    Path(&'a Path),
    Component(Component<'b>),
    String(String),
}

impl<'a, 'b> AsRef<Path> for Expansion<'a, 'b> {
    fn as_ref(&self) -> &Path {
        match self {
            Self::Path(p) => p,
            Self::Component(c) => c.as_ref(),
            Self::String(s) => Path::new(s),
        }
    }
}

fn expand_component<'a, 'b>(home: &'a Path, part: Component<'b>) -> Expansion<'a, 'b> {
    let Component::Normal(s) = part else {
        return Expansion::Component(part);
    };

    let Some(s) = s.to_str() else {
        return Expansion::Component(part);
    };

    if s.starts_with('%') {
        let today = chrono::Local::now().date_naive();
        Expansion::String(today.format(s).to_string())
    } else if s == "~" {
        Expansion::Path(home)
    } else {
        Expansion::Component(part)
    }
}

/// # Notes
///
/// Reads config from `~/.config/jump/targets.csv`, where `~` is returned by [`std::env::home_dir`].
/// That function is deprecated because it behaved inconsistently on Windows before Rust 1.85, but
/// it does what we want here.
///
/// The `targets.csv` file supports blank lines, comment lines (beginning with `#`), and jagged
/// lines.  The first column in each row is a directory path, and all subsequent columns are short
/// names for that path.
///
/// # TODO
///
/// Support database file path specfication via environment variables.
fn main_imp() -> Result<(), db::Error> {
    let mut is_verbose = false;

    #[allow(deprecated)]
    let home = env::home_dir().expect("user should have a home directory");

    let db_path = home.join(".config/jump/targets.csv");
    let db = db::Database::read_file(&db_path)?;

    for arg in env::args().skip(1) {
        if arg == "-v" {
            is_verbose = true;
            continue;
        }

        let Some(path) = db.get(&arg) else {
            return Err(db::Error::arg(db_path, arg));
        };

        let buf = path
            .components()
            .map(|c| expand_component(&home, c))
            .collect::<PathBuf>();

        if is_verbose {
            if buf == *path {
                eprintln!("{}", buf.display());
            } else {
                eprintln!("{} -> {}", path.display(), buf.display());
            }
        }

        let command = if buf.starts_with("http://") || buf.starts_with("https://") {
            command::OPEN
        } else {
            command::CD
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
