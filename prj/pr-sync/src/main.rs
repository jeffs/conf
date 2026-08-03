//! Record open pull request numbers in repo-local Jujutsu config, so that
//! `jj log` can label each bookmark with the pull request it belongs to.
//!
//! A jj template cannot reach the network, so the log reads a cached table
//! rather than GitHub itself: `format_pr_refs()` looks up `pr."<bookmark>"` for
//! every local bookmark on a commit.  This program rewrites that table from
//! `gh pr list`, and is the only thing that keeps it current -- a pull request
//! merged since the last run keeps its label until the next one.
//!
//! Every open pull request is recorded, not only those whose head branch has a
//! local bookmark today.  The template ignores entries no bookmark names, so
//! the extra rows cost a few lines of config and save a sync when a teammate's
//! branch is later checked out.

use std::{collections::BTreeMap, process::Command};

use key::Key;
use pull::Pull;
use serde::Deserialize;

/// One entry of `gh pr list --json number,headRefName`.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PullRequest {
    number: Pull,
    head_ref_name: String,
}

fn main() {
    let open = open_pull_requests();

    for key in recorded_keys() {
        if !open.contains_key(&key) {
            run(Command::new("jj").args(["config", "unset", "--repo", key.as_str()]));
        }
    }
    for (key, pull) in &open {
        run(Command::new("jj").args(["config", "set", "--repo", key.as_str(), &pull.to_string()]));
    }

    eprintln!("pr-sync: {} open pull requests recorded", open.len());
}

/// Every open pull request, keyed by the config entry naming its head branch.
///
/// A branch whose name cannot appear in a TOML basic string is skipped with a
/// warning; git permits such names, and no bookmark here has ever used one.
fn open_pull_requests() -> BTreeMap<Key, Pull> {
    let listing = capture(Command::new("gh").args([
        "pr",
        "list",
        "--state",
        "open",
        "--json",
        "number,headRefName",
        "--limit",
        "1000",
    ]));
    let pulls: Vec<PullRequest> = serde_json::from_str(&listing)
        .unwrap_or_else(|error| fail(&format!("reading pull requests: {error}")));

    pulls
        .into_iter()
        .filter_map(|pull| {
            let Some(key) = Key::for_branch(&pull.head_ref_name) else {
                eprintln!("pr-sync: skipping unquotable branch {}", pull.head_ref_name);
                return None;
            };
            Some((key, pull.number))
        })
        .collect()
}

/// The config entries a previous run left behind.
fn recorded_keys() -> Vec<Key> {
    let listing = capture(Command::new("jj").args(["config", "list", "--repo", "pr"]));
    listing.lines().filter_map(Key::from_listing).collect()
}

/// Run `command` to completion, ending this process if it fails.
fn run(command: &mut Command) {
    let status = command
        .status()
        .unwrap_or_else(|error| fail(&format!("running {}: {error}", program(command))));
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

/// Run `command` and return its standard output, ending this process if it fails.
fn capture(command: &mut Command) -> String {
    let output = command
        .output()
        .unwrap_or_else(|error| fail(&format!("running {}: {error}", program(command))));
    if !output.status.success() {
        std::io::Write::write_all(&mut std::io::stderr(), &output.stderr).ok();
        std::process::exit(output.status.code().unwrap_or(1));
    }
    String::from_utf8(output.stdout)
        .unwrap_or_else(|error| fail(&format!("{} wrote invalid UTF-8: {error}", program(command))))
}

fn program(command: &Command) -> String {
    command.get_program().to_string_lossy().into_owned()
}

fn fail(message: &str) -> ! {
    eprintln!("pr-sync: {message}");
    std::process::exit(1)
}

mod pull {
    use std::fmt;

    use serde::Deserialize;

    /// A pull request number, as it appears in the GitHub UI.
    #[derive(Clone, Copy, Deserialize)]
    pub struct Pull(u32);

    impl fmt::Display for Pull {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }
}

mod key {
    /// The repo-config entry recording which pull request a bookmark belongs
    /// to, as `jj config set` and `jj config list` spell it: `pr."<branch>"`.
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    pub struct Key(String);

    impl Key {
        /// `None` when the branch name would not survive TOML quoting.
        pub fn for_branch(branch: &str) -> Option<Self> {
            let quotable = !branch.contains(['"', '\\']) && !branch.contains(char::is_control);
            quotable.then(|| Self(format!("pr.\"{branch}\"")))
        }

        /// The key from one `jj config list` line, `pr."<branch>" = <number>`.
        pub fn from_listing(line: &str) -> Option<Self> {
            let key = line.split_once(" = ")?.0;
            key.starts_with("pr.\"").then(|| Self(key.to_owned()))
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }
}
