//! Bring a Jujutsu repository into line with what other tools read: the Git
//! HEAD a workspace is diffed against, and the table of open pull requests
//! `jj log` labels bookmarks from.
//!
//! `worktree` (or `w`) does the first, `prs` (or `p`) the second, and no
//! subcommand at all does both.  `--fetch` runs `jj git fetch` before any of
//! that, forwarding whatever follows `--`; the `jgs` alias is that flag.
//!
//! Whatever the task, a stale working copy is caught up first: jj refuses to
//! fetch until it is, and a HEAD pointed at `@-` describes nothing while the
//! files on disk are from somewhere else.
//!
//! The GitHub query is the only work here that runs no jj command, so it is the
//! only work that overlaps the fetch; everything else waits, and two jj
//! commands never touch the repo at once.

use std::{
    env,
    ffi::OsString,
    io::Write as _,
    process::{Child, Command},
};

use error::Error;
use jjkit::{head, stale};

mod error;
mod prs;

/// Everything the command line may hold.
pub const USAGE: &str = "usage: jj-sync [--fetch] [worktree | prs] [-- <fetch argument>...]";

fn main() {
    if let Err(error) = sync() {
        std::io::stderr().write_all(error.stderr()).ok();
        eprintln!("jj-sync: {error}");
        std::process::exit(error.code());
    }
}

/// Do what the plan asks, starting any fetch first so that the GitHub query
/// runs while it does.  A failed query still waits for the fetch, and outranks
/// any fetch failure in the report.
fn sync() -> Result<(), Error> {
    let plan = Plan::parse()?;

    // Ahead of every other jj command: a stale working copy stops the fetch,
    // and leaves the rest describing a repository the files on disk are no
    // longer from.  Silent, and cheap, where there is nothing to catch up.
    stale::update()?;

    // Resolved before the fetch starts, since asking costs a jj command.
    let github = plan.task.prs().then(prs::github_repo).transpose()?;
    let fetch = plan.fetch.as_deref().map(spawn_fetch).transpose()?;
    let open = github.as_ref().map(prs::open);
    let fetched = fetch.map_or(Ok(()), join);
    let open = open.transpose()?;
    fetched?;

    if let Some(open) = open {
        prs::record(&open)?;
    }
    if plan.task.worktree() {
        // The outcome goes unreported: a workspace jj keeps colocated, a
        // directory in no workspace, and a HEAD already in place are all quiet
        // successes, and `watch-jj` syncs on every refresh.
        let _ = head::sync()?;
    }
    Ok(())
}

/// What one run does.
struct Plan {
    /// Arguments for a `jj git fetch` to run first, where the command line
    /// calls for one.
    fetch: Option<Vec<OsString>>,
    task: Task,
}

/// Which of the two syncs to do.
#[derive(Clone, Copy)]
enum Task {
    /// Point this workspace's Git HEAD at `@-`.
    Worktree,
    /// Rewrite the table of open pull requests.
    Prs,
    Both,
}

impl Task {
    fn worktree(self) -> bool {
        matches!(self, Self::Worktree | Self::Both)
    }

    fn prs(self) -> bool {
        matches!(self, Self::Prs | Self::Both)
    }
}

impl Plan {
    /// Read the command line, in which the subcommand and the flag may come in
    /// either order.  `--` ends this program's own arguments and hands the rest
    /// to the fetch, so it means nothing without `--fetch` before it.
    fn parse() -> Result<Self, Error> {
        let mut fetch = None;
        let mut task = None;
        let mut args = env::args_os().skip(1);
        while let Some(arg) = args.next() {
            let text = arg.to_string_lossy().into_owned();
            match text.as_str() {
                "--fetch" if fetch.is_none() => fetch = Some(Vec::new()),
                "--" if fetch.is_some() => {
                    fetch = Some(args.collect());
                    break;
                }
                "worktree" | "w" if task.is_none() => task = Some(Task::Worktree),
                "prs" | "p" if task.is_none() => task = Some(Task::Prs),
                _ => return Err(Error::Argument(text)),
            }
        }
        Ok(Self {
            fetch,
            task: task.unwrap_or(Task::Both),
        })
    }
}

/// The name a failed fetch is reported under.
const FETCH: &str = "jj git fetch";

/// Start `jj git fetch`, forwarding `args`.
fn spawn_fetch(args: &[OsString]) -> Result<Child, jjkit::Error> {
    Command::new("jj")
        .args(["git", "fetch"])
        .args(args)
        .spawn()
        .map_err(|source| jjkit::Error::Io {
            program: FETCH.to_owned(),
            source,
        })
}

/// Wait for the fetch.
fn join(mut fetch: Child) -> Result<(), jjkit::Error> {
    let status = fetch.wait().map_err(|source| jjkit::Error::Io {
        program: FETCH.to_owned(),
        source,
    })?;
    if status.success() {
        Ok(())
    } else {
        Err(jjkit::Error::Exit {
            program: FETCH.to_owned(),
            status,
            stderr: Vec::new(),
        })
    }
}
