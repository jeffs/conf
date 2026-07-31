use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;

use crate::output::{Kind, Sink};

/// Result of running a jj (or shell) command.
#[must_use]
pub struct RunResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

impl RunResult {
    fn noop() -> Self {
        Self {
            success: true,
            stdout: String::new(),
            stderr: String::new(),
        }
    }

    fn error(e: &std::io::Error) -> Self {
        Self {
            success: false,
            stdout: String::new(),
            stderr: e.to_string(),
        }
    }
}

/// Run a command, echoing it and streaming its output to `sink`, or printing
/// it without executing if `dry_run`.
fn run(cmd: &[&str], cwd: &Path, dry_run: bool, sink: &dyn Sink) -> RunResult {
    let display = cmd.join(" ");
    if dry_run {
        sink.line(Kind::DryRun, display);
        return RunResult::noop();
    }

    sink.line(Kind::Cmd, display);
    exec_stream(cmd, cwd, sink)
}

/// Run a command silently, collecting output. Used for read-only queries;
/// failures are still reported to `sink`.
fn run_quiet(cmd: &[&str], cwd: &Path, sink: &dyn Sink) -> RunResult {
    let result = Command::new(cmd[0])
        .args(&cmd[1..])
        .current_dir(cwd)
        .env_remove("CARGO_MANIFEST_DIR")
        .output();

    match result {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
            let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
            if !out.status.success() {
                let msg = if stderr.is_empty() { &stdout } else { &stderr };
                sink.line(Kind::Error, msg.trim().to_string());
            }
            RunResult {
                success: out.status.success(),
                stdout,
                stderr,
            }
        }
        Err(e) => {
            sink.line(
                Kind::Error,
                format!("failed to run `{}`: {e}", cmd.join(" ")),
            );
            RunResult::error(&e)
        }
    }
}

/// Spawn a command and forward each output line to `sink` as it arrives,
/// while also collecting the full text for the caller.
fn exec_stream(cmd: &[&str], cwd: &Path, sink: &dyn Sink) -> RunResult {
    let display = cmd.join(" ");
    let spawned = Command::new(cmd[0])
        .args(&cmd[1..])
        .current_dir(cwd)
        .env_remove("CARGO_MANIFEST_DIR")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    let mut child = match spawned {
        Ok(child) => child,
        Err(e) => {
            sink.line(Kind::Error, format!("failed to run `{display}`: {e}"));
            return RunResult::error(&e);
        }
    };

    let out_pipe = child.stdout.take().expect("stdout is piped");
    let err_pipe = child.stderr.take().expect("stderr is piped");
    let (stdout, stderr) = thread::scope(|scope| {
        let out = scope.spawn(move || drain(out_pipe, sink));
        let err = scope.spawn(move || drain(err_pipe, sink));
        (
            out.join().expect("stdout reader"),
            err.join().expect("stderr reader"),
        )
    });

    match child.wait() {
        Ok(status) => {
            if !status.success() {
                sink.line(Kind::Error, format!("`{display}` failed ({status})"));
            }
            RunResult {
                success: status.success(),
                stdout,
                stderr,
            }
        }
        Err(e) => {
            sink.line(Kind::Error, format!("failed to wait for `{display}`: {e}"));
            RunResult::error(&e)
        }
    }
}

/// Forward each line to `sink` while collecting the full text.
fn drain(pipe: impl Read, sink: &dyn Sink) -> String {
    let mut text = String::new();
    for line in BufReader::new(pipe).lines() {
        let Ok(line) = line else { break };
        sink.line(Kind::Out, line.clone());
        text.push_str(&line);
        text.push('\n');
    }
    text
}

/// Run a shell command via `sh -c`.
fn run_shell(cmd_str: &str, cwd: &Path, dry_run: bool, sink: &dyn Sink) -> RunResult {
    run(&["sh", "-c", cmd_str], cwd, dry_run, sink)
}

pub fn fetch(cwd: &Path, remote: Option<&str>, dry_run: bool, sink: &dyn Sink) -> RunResult {
    match remote {
        Some(r) => run(&["jj", "git", "fetch", "--remote", r], cwd, dry_run, sink),
        None => run(&["jj", "git", "fetch"], cwd, dry_run, sink),
    }
}

pub fn bookmark_set(
    cwd: &Path,
    bookmark: &str,
    revision: &str,
    dry_run: bool,
    sink: &dyn Sink,
) -> RunResult {
    run(
        &["jj", "bookmark", "set", bookmark, "-r", revision],
        cwd,
        dry_run,
        sink,
    )
}

pub fn rebase(cwd: &Path, bookmark: &str, dest: &str, dry_run: bool, sink: &dyn Sink) -> RunResult {
    run(
        &["jj", "rebase", "-b", bookmark, "-d", dest, "--skip-emptied"],
        cwd,
        dry_run,
        sink,
    )
}

pub fn has_conflicts(cwd: &Path, bookmark: &str, dry_run: bool, sink: &dyn Sink) -> RunResult {
    let cmd = [
        "jj",
        "log",
        "-r",
        bookmark,
        "--no-graph",
        "-T",
        "if(conflict, \"CONFLICT\\n\")",
    ];
    if dry_run {
        sink.line(Kind::DryRun, cmd.join(" "));
        return RunResult::noop();
    }
    let result = run_quiet(&cmd, cwd, sink);
    RunResult {
        success: result.success && !result.stdout.contains("CONFLICT"),
        ..result
    }
}

pub fn push(cwd: &Path, remote: &str, bookmark: &str, dry_run: bool, sink: &dyn Sink) -> RunResult {
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
        sink,
    )
}

pub fn clone(cwd: &Path, url: &str, dest: &Path, dry_run: bool, sink: &dyn Sink) -> RunResult {
    let dest_str = dest.to_string_lossy();
    run(
        &["jj", "git", "clone", "--colocate", url, &dest_str],
        cwd,
        dry_run,
        sink,
    )
}

pub fn remote_add(cwd: &Path, name: &str, url: &str, dry_run: bool, sink: &dyn Sink) -> RunResult {
    run(
        &["jj", "git", "remote", "add", name, url],
        cwd,
        dry_run,
        sink,
    )
}

/// Create a local bookmark tracking `bookmark@remote`.
///
/// `jj git clone` only creates a local bookmark for the remote's default
/// branch; others exist solely as `name@remote`.  Tracking makes the bare
/// bookmark name resolve locally.  A no-op (still succeeds) when the remote
/// bookmark is absent, giving "track it if present" semantics.
pub fn bookmark_track(
    cwd: &Path,
    bookmark: &str,
    remote: &str,
    dry_run: bool,
    sink: &dyn Sink,
) -> RunResult {
    let qualified = format!("{bookmark}@{remote}");
    run(&["jj", "bookmark", "track", &qualified], cwd, dry_run, sink)
}

pub fn build(cwd: &Path, cmd_str: &str, dry_run: bool, sink: &dyn Sink) -> RunResult {
    run_shell(cmd_str, cwd, dry_run, sink)
}

/// Move the working copy to a new empty commit on top of the given revision.
pub fn new_at(cwd: &Path, revision: &str, dry_run: bool, sink: &dyn Sink) -> RunResult {
    run(&["jj", "new", revision], cwd, dry_run, sink)
}

/// Get a summary of repo status (working copy clean, bookmarks, etc.).
pub fn status(cwd: &Path, sink: &dyn Sink) -> RunResult {
    run_quiet(&["jj", "status"], cwd, sink)
}

/// Get log of bookmarks relative to upstream.
pub fn log_bookmark(
    cwd: &Path,
    bookmark: &str,
    upstream_qualified: &str,
    sink: &dyn Sink,
) -> RunResult {
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
        sink,
    )
}
