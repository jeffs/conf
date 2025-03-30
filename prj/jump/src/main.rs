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

use std::io::Write;
use std::process::ExitCode;
use std::{env, io};

fn main_imp() -> jump::Result<()> {
    let app = jump::App::from_env()?;
    let mut stdout = io::stdout();
    for target in env::args().skip(1) {
        stdout
            .write_all(&app.jump(&target)?)
            .expect("stdout should be writable");
    }
    Ok(())
}

fn main() -> ExitCode {
    if let Err(err) = main_imp() {
        eprintln!("jump: {err}");
        return ExitCode::FAILURE;
    };
    ExitCode::SUCCESS
}
