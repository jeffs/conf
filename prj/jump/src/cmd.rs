//! Maps semantic command names (such as `cd`) to their implementation in the calling shell.
//!
//! TODO: Read shell commands from config, rather than hard-coding them here.

/// Change directory.
pub const CD: &str = "mc";

/// Use the OS native file association.
///
/// TODO: Compare macOS `open`, Windows `start`, and Linux `xdg-open`.
pub const OPEN: &str = "open";
