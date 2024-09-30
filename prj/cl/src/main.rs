//! Opens a log project for today (or -`-yesterday``) in VS Code, creating it if necessary.
//!
//! # TODO
//!
//! * Accept dates as arguments.

use std::{
    env::{self, set_current_dir},
    fs, io,
    path::PathBuf,
    process::exit,
};

use chrono::{Local, TimeDelta};

enum Error {
    Arg(String),
    Dir(PathBuf, io::Error),
}

type Result<T> = std::result::Result<T, Error>;

fn main_imp() -> Result<()> {
    let date = match env::args().nth(1) {
        Some(arg) if arg == "-y" || arg == "--yesterday" => Local::now() - TimeDelta::days(1),
        Some(arg) => return Err(Error::Arg(arg)),
        None => Local::now(),
    }
    .date_naive();

    let path = dirs::home_dir()
        .expect("can't find home directory")
        .join("log")
        .join(date.format("%Y/%m/%d").to_string());

    let _ = fs::create_dir_all(&path); // It's OK if the path already exists.

    set_current_dir(&path).map_err(|err| Error::Dir(path, err))?;

    Ok(())
}

fn main() {
    match main_imp() {
        Ok(()) => (),
        Err(Error::Arg(arg)) => {
            eprintln!("Error: bad argument: '{arg}'");
            eprintln!("Usage: cl [-y|--yesterday]");
            exit(2);
        }
        Err(Error::Dir(path, err)) => {
            eprintln!("Error: {}: can't chdir: {err}", path.display());
            exit(1);
        }
    }
}
