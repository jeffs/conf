//! Record open pull request numbers in repo-local Jujutsu config, so that
//! `jj log` can label each bookmark with the pull request it belongs to.
//!
//! A jj template cannot reach the network, so the log reads a cached table
//! rather than GitHub itself: `format_pr_refs()` looks up `pr."<bookmark>"` for
//! every local bookmark on a commit.  [`record`] rewrites that table, and is
//! the only thing that keeps it current -- a pull request merged since the last
//! run keeps its label until the next one.
//!
//! Every open pull request is recorded, not only those whose head branch has a
//! local bookmark today.  The template ignores entries no bookmark names, so
//! the extra rows cost a few lines of config and save a sync when a teammate's
//! branch is later checked out.
//!
//! The GitHub repository is resolved through jj's remote list rather than from
//! the working directory, so this program always agrees with jj about which
//! repo it is in -- even in a secondary workspace, which has no `.git` for
//! `gh` to find.

use std::{collections::BTreeMap, process::Command};

use jjkit::process::{capture, run};
use serde::Deserialize;

use crate::error::Error;

pub use key::Key;
pub use pull::Pull;
pub use repo::Repo;

/// One entry of `gh pr list --json number,headRefName`.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PullRequest {
    number: Pull,
    head_ref_name: String,
}

/// The GitHub repository named by the `origin` remote, or by the sole remote
/// when none is named `origin`.
///
/// Asking jj, rather than letting `gh` inspect the working directory, keeps
/// the answer aligned with the repo whose config this program rewrites: jj
/// reads the shared repo store, so every workspace agrees, colocated or not.
///
/// # Errors
///
/// Fails if jj cannot be run or refuses, or if the remote it names fits no
/// recognized git URL form.
pub fn github_repo() -> Result<Repo, Error> {
    let listing =
        capture(Command::new("jj").args(["--ignore-working-copy", "git", "remote", "list"]))?;
    let remotes: Vec<(&str, &str)> = listing
        .lines()
        .filter_map(|line| line.split_once(' '))
        .collect();
    let url = match remotes[..] {
        [(_, url)] => url,
        _ => remotes
            .iter()
            .find_map(|(name, url)| (*name == "origin").then_some(*url))
            .ok_or(Error::NoOrigin)?,
    };
    Repo::from_remote_url(url).ok_or_else(|| Error::RemoteUrl(url.to_owned()))
}

/// Every open pull request of `github`, keyed by the config entry naming its
/// head branch.
///
/// A branch whose name cannot appear in a TOML basic string is skipped with a
/// warning; git permits such names, and no bookmark here has ever used one.
///
/// # Errors
///
/// Fails if `gh` cannot be run, refuses, or prints unreadable JSON.
pub fn open(github: &Repo) -> Result<BTreeMap<Key, Pull>, Error> {
    let listing = capture(Command::new("gh").args([
        "pr",
        "list",
        "--repo",
        github.as_str(),
        "--state",
        "open",
        "--json",
        "number,headRefName",
        "--limit",
        "1000",
    ]))?;
    let pulls: Vec<PullRequest> = serde_json::from_str(&listing).map_err(Error::Json)?;

    Ok(pulls
        .into_iter()
        .filter_map(|pull| {
            let Some(key) = Key::for_branch(&pull.head_ref_name) else {
                eprintln!("jj-sync: skipping unquotable branch {}", pull.head_ref_name);
                return None;
            };
            Some((key, pull.number))
        })
        .collect())
}

/// Rewrite the table to hold exactly the pull requests in `open`.
///
/// # Errors
///
/// Fails if jj cannot be run or refuses.
pub fn record(open: &BTreeMap<Key, Pull>) -> Result<(), Error> {
    for key in recorded_keys()? {
        if !open.contains_key(&key) {
            run(Command::new("jj").args(["config", "unset", "--repo", key.as_str()]))?;
        }
    }
    for (key, pull) in open {
        run(Command::new("jj").args(["config", "set", "--repo", key.as_str(), &pull.to_string()]))?;
    }

    eprintln!("jj-sync: {} open pull requests recorded", open.len());
    Ok(())
}

/// The config entries a previous run left behind.
fn recorded_keys() -> Result<Vec<Key>, Error> {
    let listing = capture(Command::new("jj").args(["config", "list", "--repo", "pr"]))?;
    Ok(listing.lines().filter_map(Key::from_listing).collect())
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

mod repo {
    /// A GitHub repository, as `gh --repo` spells it: `HOST/OWNER/REPO`.
    pub struct Repo(String);

    impl Repo {
        /// Parses a git remote URL: `git@host:owner/repo.git`,
        /// `ssh://git@host/owner/repo.git`, or `https://host/owner/repo.git`.
        pub fn from_remote_url(url: &str) -> Option<Self> {
            let url = url.strip_suffix(".git").unwrap_or(url);
            let (host, path) = if let Some((_scheme, rest)) = url.split_once("://") {
                let rest = rest.split_once('@').map_or(rest, |(_user, rest)| rest);
                rest.split_once('/')?
            } else {
                let rest = url.split_once('@').map_or(url, |(_user, rest)| rest);
                rest.split_once(':')?
            };
            let (owner, name) = path.split_once('/')?;
            let plausible =
                !host.is_empty() && !owner.is_empty() && !name.is_empty() && !name.contains('/');
            plausible.then(|| Self(format!("{host}/{owner}/{name}")))
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Repo;

        #[test]
        fn parses_common_remote_urls() {
            for url in [
                "git@github.com:jeffs/conf.git",
                "ssh://git@github.com/jeffs/conf.git",
                "https://github.com/jeffs/conf",
            ] {
                let repo = Repo::from_remote_url(url).expect(url);
                assert_eq!(repo.as_str(), "github.com/jeffs/conf");
            }
        }

        #[test]
        fn rejects_urls_naming_no_repository() {
            for url in [
                "github.com/jeffs/conf",
                "git@github.com:conf.git",
                "https://github.com",
            ] {
                assert!(Repo::from_remote_url(url).is_none(), "{url}");
            }
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

        /// The key from one `jj config list` line, `pr.<branch> = <number>`.
        /// The listing quotes the branch only when TOML demands it, so the
        /// name is unwrapped and requoted to compare equal to a key built by
        /// [`Key::for_branch`].
        pub fn from_listing(line: &str) -> Option<Self> {
            let key = line.split_once(" = ")?.0;
            let branch = key.strip_prefix("pr.")?;
            let branch = branch
                .strip_prefix('"')
                .and_then(|rest| rest.strip_suffix('"'))
                .unwrap_or(branch);
            Self::for_branch(branch)
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Key;

        #[test]
        fn reads_bare_and_quoted_listing_keys() {
            for (line, key) in [
                ("pr.jgfs = 7", "pr.\"jgfs\""),
                ("pr.\"ar/jgfs\" = 9", "pr.\"ar/jgfs\""),
            ] {
                assert_eq!(Key::from_listing(line).expect(line).as_str(), key);
            }
        }

        #[test]
        fn ignores_lines_recording_no_pull_request() {
            for line in ["ui.pager = \"delta\"", "pr.jgfs"] {
                assert!(Key::from_listing(line).is_none(), "{line}");
            }
        }
    }
}
