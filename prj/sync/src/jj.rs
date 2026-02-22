use std::path::Path;
use std::process::Command;

use crate::output;

/// Result of running a jj (or shell) command.
pub struct RunResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

/// Run a command, printing it if `dry_run` or executing it otherwise.
fn run(cmd: &[&str], cwd: &Path, dry_run: bool) -> RunResult {
    let display = cmd.join(" ");
    if dry_run {
        output::dry_run(&display);
        return RunResult {
            success: true,
            stdout: String::new(),
            stderr: String::new(),
        };
    }

    output::cmd(&display);
    exec(cmd, cwd)
}

/// Run a command silently (no echo). Used for read-only queries.
fn run_quiet(cmd: &[&str], cwd: &Path) -> RunResult {
    exec(cmd, cwd)
}

fn exec(cmd: &[&str], cwd: &Path) -> RunResult {
    let display = cmd.join(" ");
    let result = Command::new(cmd[0])
        .args(&cmd[1..])
        .current_dir(cwd)
        .output();

    match result {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
            let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
            if !out.status.success() {
                let msg = if stderr.is_empty() { &stdout } else { &stderr };
                output::error(&format!("  {}", msg.trim()));
            }
            RunResult {
                success: out.status.success(),
                stdout,
                stderr,
            }
        }
        Err(e) => {
            output::error(&format!("  failed to run `{display}`: {e}"));
            RunResult {
                success: false,
                stdout: String::new(),
                stderr: e.to_string(),
            }
        }
    }
}

/// Run a shell command via `sh -c`.
fn run_shell(cmd_str: &str, cwd: &Path, dry_run: bool) -> RunResult {
    run(&["sh", "-c", cmd_str], cwd, dry_run)
}

pub fn fetch(cwd: &Path, remote: Option<&str>, dry_run: bool) -> RunResult {
    match remote {
        Some(r) => run(&["jj", "git", "fetch", "--remote", r], cwd, dry_run),
        None => run(&["jj", "git", "fetch"], cwd, dry_run),
    }
}

pub fn bookmark_set(cwd: &Path, bookmark: &str, revision: &str, dry_run: bool) -> RunResult {
    run(
        &["jj", "bookmark", "set", bookmark, "-r", revision],
        cwd,
        dry_run,
    )
}

pub fn rebase(cwd: &Path, bookmark: &str, dest: &str, dry_run: bool) -> RunResult {
    run(
        &[
            "jj",
            "rebase",
            "-b",
            bookmark,
            "-d",
            dest,
            "--skip-emptied",
        ],
        cwd,
        dry_run,
    )
}

pub fn has_conflicts(cwd: &Path, bookmark: &str, dry_run: bool) -> RunResult {
    if dry_run {
        let display =
            format!("jj log -r '{bookmark}' --no-graph -T 'if(conflict, \"CONFLICT\\n\")'");
        output::dry_run(&display);
        return RunResult {
            success: true,
            stdout: String::new(),
            stderr: String::new(),
        };
    }
    let result = run(
        &[
            "jj",
            "log",
            "-r",
            bookmark,
            "--no-graph",
            "-T",
            "if(conflict, \"CONFLICT\\n\")",
        ],
        cwd,
        false,
    );
    RunResult {
        success: !result.stdout.contains("CONFLICT"),
        stdout: result.stdout,
        stderr: result.stderr,
    }
}

pub fn push(cwd: &Path, remote: &str, bookmark: &str, dry_run: bool) -> RunResult {
    run(
        &[
            "jj",
            "git",
            "push",
            "--remote",
            remote,
            "--bookmark",
            bookmark,
        ],
        cwd,
        dry_run,
    )
}

pub fn clone(cwd: &Path, url: &str, dest: &Path, dry_run: bool) -> RunResult {
    let dest_str = dest.to_string_lossy();
    run(
        &["jj", "git", "clone", "--colocate", url, &dest_str],
        cwd,
        dry_run,
    )
}

pub fn remote_add(cwd: &Path, name: &str, url: &str, dry_run: bool) -> RunResult {
    run(
        &["jj", "git", "remote", "add", name, url],
        cwd,
        dry_run,
    )
}

pub fn build(cwd: &Path, cmd_str: &str, dry_run: bool) -> RunResult {
    run_shell(cmd_str, cwd, dry_run)
}

/// Move the working copy to a new empty commit on top of the given revision.
pub fn new_at(cwd: &Path, revision: &str, dry_run: bool) -> RunResult {
    run(&["jj", "new", revision], cwd, dry_run)
}

/// Get a summary of repo status (working copy clean, bookmarks, etc.).
pub fn status(cwd: &Path) -> RunResult {
    run_quiet(&["jj", "status"], cwd)
}

/// Get log of bookmarks relative to upstream.
pub fn log_bookmark(cwd: &Path, bookmark: &str, upstream_qualified: &str) -> RunResult {
    let revset = format!("{bookmark}:: ~ {upstream_qualified}::");
    run_quiet(
        &[
            "jj",
            "log",
            "-r",
            &revset,
            "--no-graph",
            "-T",
            "description.first_line() ++ \"\\n\"",
        ],
        cwd,
    )
}
