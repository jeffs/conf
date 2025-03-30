//! # Notes
//!
//! Reads config from `~/.config/jump/targets.csv`. The `targets.csv` file supports blank lines,
//! comment lines (beginning with `#`), and jagged lines.  The first column in each row is a
//! directory path, and all subsequent columns are short names for that path.
//!
//! # TODO
//!
//! * [ ] Support database file specification at runtime, via args or env.
//! * [ ] Support complex expansions like "yesterday's date."
//! * [ ] Add DB path list to error messages about missing or empty targets.

#![allow(dead_code, unused_imports)]

use jump::db;
use std::env;
use std::process::ExitCode;

enum Error {
    Arg(String),
    Jump(jump::Error),
}

type Result<T> = std::result::Result<T, Error>;

#[cfg(any())]
fn main_imp() -> Result<()> {
    // The [`std::env::home_dir`] function is deprecated because it behaved inconsistently on
    // Windows before Rust 1.85, but it does what we want here.
    #[allow(deprecated)]
    let home = env::home_dir().expect("user should have a home directory");

    let db = db::Database::read_file(home.join(".config/jump/targets.csv"))?;
    let expand = jump::Expand::with_home(&home);

    for arg in env::args().skip(1) {
        let path = db.get(&arg).ok_or_else(|| Error::target_not_found(&arg))?;
        let path = expand.path(path);
        let mut parts = path.components();
        let first = parts.next().ok_or(Error::target_empty(arg))?;
        if let Some("http:" | "https:") = first.as_os_str().to_str() {
            write_command_line!(OPEN, first, b"//", parts.collect::<PathBuf>());
        } else {
            write_command_line!(CD, path);
        }
    }

    Ok(())
}

#[cfg(any())]
fn main() -> ExitCode {
    if let Err(err) = main_imp() {
        eprintln!("jump: {err}");
        return ExitCode::FAILURE;
    };
    ExitCode::SUCCESS
}

fn main() {}
