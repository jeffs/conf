//! Maps semantic command names (such as `cd`) to their implementation in the calling shell.
//!
//! TODO: Read shell commands from config, rather than hard-coding them here.

use std::fmt;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::as_bytes::AsBytes;

#[derive(Debug)]
pub enum Error {
    /// An expanded path was empty.
    Empty,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "Empty target"),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/// Change directory.
const CD: &str = "mc";

/// Use the OS native file association.
///
/// TODO: Compare macOS `open`, Windows `start`, and Linux `xdg-open`.
const OPEN: &str = "open";

macro_rules! write_command {
    ($command:ident, $($arg:expr),*) => {{
        let mut buffer = Vec::new();
        buffer.write_all(crate::cmd::$command.as_bytes()).unwrap();
        buffer.write_all(b" ").unwrap();
        $(
            buffer.write_all($arg.as_bytes()).unwrap();
        )*
        buffer.write_all(b"\n").unwrap();
    }};
}

pub fn command(path: &Path) -> Result<Vec<u8>> {
    let mut parts = path.components();
    let first = parts.next().ok_or(Error::Empty)?;
    if let Some("http:" | "https:") = first.as_os_str().to_str() {
        write_command!(OPEN, first, b"//", parts.collect::<PathBuf>());
    } else {
        write_command!(CD, path);
    }

    todo!()
}
