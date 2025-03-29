use std::path::{Component, Path, PathBuf};
use std::{env, fmt};

use jump::{command, db};

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

enum Expansion<'a, 'b> {
    Path(&'a Path),
    Component(Component<'b>),
    String(String),
}

impl AsRef<Path> for Expansion<'_, '_> {
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
