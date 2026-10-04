//! Persists the output of failed tasks under `~/var/log/upgrade/`.

use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::PathBuf;

use crate::task::Task;

/// Writes the task's output and error to a timestamped file, returning its path.
pub fn save_failure(task: &Task, error: &str) -> io::Result<PathBuf> {
    let home = std::env::var_os("HOME").ok_or_else(|| io::Error::other("HOME is not set"))?;
    let dir = PathBuf::from(home).join("var/log/upgrade");
    fs::create_dir_all(&dir)?;
    let stamp = chrono::Local::now().format("%Y-%m-%dT%H%M%S");
    let path = dir.join(format!("{stamp}-{}.log", task.id));
    let mut contents = String::new();
    for line in &task.output {
        let _ = writeln!(contents, "{line}");
    }
    let _ = writeln!(contents, "error: {error}");
    fs::write(&path, contents)?;
    Ok(path)
}
