//! Thin wrapper around `jj` (and shell) subprocesses.
//!
//! Every command method returns [`Result`]: `Ok` with captured output, or an
//! [`Error`] carrying a short failure reason (detail has already gone to the
//! sink) — so per-method `# Errors` sections would all say the same thing.
#![allow(clippy::missing_errors_doc)]

use std::fmt;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;

use crate::output::{Kind, Sink};

/// Whether commands actually execute or are only printed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Execute,
    DryRun,
}

/// A named git remote this tool manages.
#[derive(Debug, Clone, Copy)]
pub enum Remote {
    Origin,
    Upstream,
}

impl Remote {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Origin => "origin",
            Self::Upstream => "upstream",
        }
    }
}

impl fmt::Display for Remote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Captured stdout of a successful command.
pub struct Output {
    pub stdout: String,
}

/// A short failure reason, fit for a summary line; full detail has already
/// been reported to the sink.
pub struct Error(pub String);

impl Error {
    fn new(label: &str) -> Self {
        Self(format!("{label} failed"))
    }
}

pub type Result<T = Output> = std::result::Result<T, Error>;

/// A jj working directory bound to an execution mode and a progress sink —
/// the context every command needs, captured once.
pub struct Workspace<'a> {
    cwd: &'a Path,
    mode: Mode,
    sink: &'a dyn Sink,
}

impl<'a> Workspace<'a> {
    #[must_use]
    pub fn new(cwd: &'a Path, mode: Mode, sink: &'a dyn Sink) -> Self {
        Self { cwd, mode, sink }
    }

    pub fn fetch(&self, remote: Option<Remote>) -> Result {
        match remote {
            Some(r) => self.run(
                &format!("fetch {r}"),
                &["jj", "git", "fetch", "--remote", r.as_str()],
            ),
            None => self.run("fetch", &["jj", "git", "fetch"]),
        }
    }

    pub fn bookmark_set(&self, bookmark: &str, revision: &str) -> Result {
        self.run(
            &format!("bookmark set {bookmark}"),
            &["jj", "bookmark", "set", bookmark, "-r", revision],
        )
    }

    pub fn rebase(&self, bookmark: &str, dest: &str) -> Result {
        self.run(
            &format!("rebase {bookmark}"),
            &["jj", "rebase", "-b", bookmark, "-d", dest, "--skip-emptied"],
        )
    }

    /// Fail if `bookmark` reaches a commit with unresolved conflicts.
    pub fn ensure_no_conflicts(&self, bookmark: &str) -> Result<()> {
        let cmd = [
            "jj",
            "log",
            "-r",
            bookmark,
            "--no-graph",
            "-T",
            "if(conflict, \"CONFLICT\\n\")",
        ];
        if self.mode == Mode::DryRun {
            self.sink.line(Kind::DryRun, cmd.join(" "));
            return Ok(());
        }
        let out = self.run_quiet(&format!("conflict check on {bookmark}"), &cmd)?;
        if out.stdout.contains("CONFLICT") {
            return Err(Error(format!("{bookmark} has conflicts")));
        }
        Ok(())
    }

    pub fn push(&self, remote: Remote, bookmark: &str) -> Result {
        self.run(
            &format!("push {bookmark}"),
            &[
                "jj",
                "git",
                "push",
                "--remote",
                remote.as_str(),
                "--bookmark",
                bookmark,
            ],
        )
    }

    pub fn clone(&self, url: &str, dest: &Path) -> Result {
        let dest = dest.to_string_lossy();
        self.run("clone", &["jj", "git", "clone", "--colocate", url, &dest])
    }

    pub fn remote_add(&self, remote: Remote, url: &str) -> Result {
        self.run(
            &format!("add remote {remote}"),
            &["jj", "git", "remote", "add", remote.as_str(), url],
        )
    }

    /// Create a local bookmark tracking `bookmark@remote`.
    ///
    /// `jj git clone` only creates a local bookmark for the remote's default
    /// branch; others exist solely as `name@remote`.  Tracking makes the bare
    /// bookmark name resolve locally.  A no-op (still succeeds) when the
    /// remote bookmark is absent, giving "track it if present" semantics.
    pub fn bookmark_track(&self, bookmark: &str, remote: Remote) -> Result {
        let qualified = format!("{bookmark}@{remote}");
        self.run(
            &format!("track {qualified}"),
            &["jj", "bookmark", "track", &qualified],
        )
    }

    /// Run a build command via `sh -c`.
    pub fn build(&self, cmd_str: &str) -> Result {
        self.run(&format!("`{cmd_str}`"), &["sh", "-c", cmd_str])
    }

    /// Move the working copy to a new empty commit on top of `revision`.
    pub fn new_at(&self, revision: &str) -> Result {
        self.run(&format!("jj new {revision}"), &["jj", "new", revision])
    }

    /// Summary of repo status (working copy clean, bookmarks, etc.).
    pub fn status(&self) -> Result {
        self.run_quiet("jj status", &["jj", "status"])
    }

    /// First lines of commits reachable from `bookmark` but not from upstream.
    pub fn log_bookmark(&self, bookmark: &str, upstream_qualified: &str) -> Result {
        let revset = format!("{bookmark}:: ~ {upstream_qualified}::");
        self.run_quiet(
            &format!("log {bookmark}"),
            &[
                "jj",
                "log",
                "-r",
                &revset,
                "--no-graph",
                "-T",
                "description.first_line() ++ \"\\n\"",
            ],
        )
    }

    /// Run a command, echoing it and streaming its output to the sink — or,
    /// in dry-run mode, print it without executing.
    fn run(&self, label: &str, cmd: &[&str]) -> Result {
        let display = cmd.join(" ");
        match self.mode {
            Mode::DryRun => {
                self.sink.line(Kind::DryRun, display);
                Ok(Output {
                    stdout: String::new(),
                })
            }
            Mode::Execute => {
                self.sink.line(Kind::Cmd, display);
                self.exec_stream(label, cmd)
            }
        }
    }

    /// Run a read-only query silently, collecting output; executes even in
    /// dry-run mode.  Failures are still reported to the sink.
    fn run_quiet(&self, label: &str, cmd: &[&str]) -> Result {
        let spawned = Command::new(cmd[0])
            .args(&cmd[1..])
            .current_dir(self.cwd)
            .output();

        let out = match spawned {
            Ok(out) => out,
            Err(e) => {
                self.sink
                    .error(&format!("failed to run `{}`: {e}", cmd.join(" ")));
                return Err(Error::new(label));
            }
        };

        let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
        if out.status.success() {
            Ok(Output { stdout })
        } else {
            let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
            let msg = if stderr.is_empty() { &stdout } else { &stderr };
            self.sink.error(msg.trim());
            Err(Error::new(label))
        }
    }

    /// Spawn a command and forward each output line to the sink as it
    /// arrives, while also collecting stdout for the caller.
    fn exec_stream(&self, label: &str, cmd: &[&str]) -> Result {
        let display = cmd.join(" ");
        let spawned = Command::new(cmd[0])
            .args(&cmd[1..])
            .current_dir(self.cwd)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        let mut child = match spawned {
            Ok(child) => child,
            Err(e) => {
                self.sink.error(&format!("failed to run `{display}`: {e}"));
                return Err(Error::new(label));
            }
        };

        let out_pipe = child.stdout.take().expect("stdout is piped");
        let err_pipe = child.stderr.take().expect("stderr is piped");
        let sink = self.sink;
        let stdout = thread::scope(|scope| {
            let out = scope.spawn(move || drain(out_pipe, sink));
            drain(err_pipe, sink);
            out.join().expect("stdout reader")
        });

        match child.wait() {
            Ok(status) if status.success() => Ok(Output { stdout }),
            Ok(status) => {
                self.sink.error(&format!("`{display}` failed ({status})"));
                Err(Error::new(label))
            }
            Err(e) => {
                self.sink
                    .error(&format!("failed to wait for `{display}`: {e}"));
                Err(Error::new(label))
            }
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
