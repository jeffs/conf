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
//!
//! The GitHub repository is resolved through jj's remote list rather than from
//! the working directory, so this program always agrees with jj about which
//! repo it is in -- even in a secondary workspace, which has no `.git` for
//! `gh` to find.
//!
//! Invoked as `jgs`, the program also runs `jj git fetch`, forwarding its
//! arguments.  The fetch and the GitHub query are independent, so they overlap;
//! the table rewrite waits for the fetch, so two jj commands never touch the
//! repo at once.

use std::{
    collections::BTreeMap,
    env, io,
    path::Path,
    process::{Child, Command},
};

use error::Error;
use key::Key;
use pull::Pull;
use repo::Repo;
use serde::Deserialize;

/// One entry of `gh pr list --json number,headRefName`.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PullRequest {
    number: Pull,
    head_ref_name: String,
}

/// The behavior selected by the name this program was invoked under:
/// `pr-sync` rewrites the table, and `jgs` also runs `jj git fetch`.
enum Invocation {
    Sync,
    FetchAndSync,
}

impl Invocation {
    fn parse() -> Result<Self, Error> {
        let arg = env::args_os().next();
        let name = arg
            .as_deref()
            .map(Path::new)
            .and_then(Path::file_name)
            .and_then(|name| name.to_str());
        match name {
            Some("pr-sync") => Ok(Self::Sync),
            Some("jgs") => Ok(Self::FetchAndSync),
            _ => Err(Error::Name),
        }
    }
}

fn main() {
    if let Err(error) = sync() {
        io::Write::write_all(&mut io::stderr(), error.stderr()).ok();
        eprintln!("pr-sync: {error}");
        std::process::exit(error.code());
    }
}

/// Rewrite the table, first starting a fetch if the executable name asks for
/// one.  A failed GitHub query still waits for the fetch, and outranks any
/// fetch failure in the report.
fn sync() -> Result<(), Error> {
    let invocation = Invocation::parse()?;
    let github = github_repo()?;

    let fetch = match invocation {
        Invocation::Sync => None,
        Invocation::FetchAndSync => Some(spawn_fetch()?),
    };

    let open = open_pull_requests(&github);
    let fetched = fetch.map_or(Ok(()), join);
    let open = open?;
    fetched?;

    for key in recorded_keys()? {
        if !open.contains_key(&key) {
            run(Command::new("jj").args(["config", "unset", "--repo", key.as_str()]))?;
        }
    }
    for (key, pull) in &open {
        run(Command::new("jj").args(["config", "set", "--repo", key.as_str(), &pull.to_string()]))?;
    }

    eprintln!("pr-sync: {} open pull requests recorded", open.len());
    Ok(())
}

/// Start `jj git fetch`, forwarding this program's arguments.
fn spawn_fetch() -> Result<Child, Error> {
    Command::new("jj")
        .args(["git", "fetch"])
        .args(env::args_os().skip(1))
        .spawn()
        .map_err(|source| Error::Io {
            program: "jj git fetch".to_owned(),
            source,
        })
}

/// Wait for the fetch.
fn join(mut fetch: Child) -> Result<(), Error> {
    let status = fetch.wait().map_err(|source| Error::Io {
        program: "jj git fetch".to_owned(),
        source,
    })?;
    if status.success() {
        Ok(())
    } else {
        Err(Error::Child {
            program: "jj git fetch".to_owned(),
            status,
            stderr: Vec::new(),
        })
    }
}

/// The GitHub repository named by the `origin` remote, or by the sole remote
/// when none is named `origin`.
///
/// Asking jj, rather than letting `gh` inspect the working directory, keeps
/// the answer aligned with the repo whose config this program rewrites: jj
/// reads the shared repo store, so every workspace agrees, colocated or not.
fn github_repo() -> Result<Repo, Error> {
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

/// Every open pull request, keyed by the config entry naming its head branch.
///
/// A branch whose name cannot appear in a TOML basic string is skipped with a
/// warning; git permits such names, and no bookmark here has ever used one.
fn open_pull_requests(github: &Repo) -> Result<BTreeMap<Key, Pull>, Error> {
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
                eprintln!("pr-sync: skipping unquotable branch {}", pull.head_ref_name);
                return None;
            };
            Some((key, pull.number))
        })
        .collect())
}

/// The config entries a previous run left behind.
fn recorded_keys() -> Result<Vec<Key>, Error> {
    let listing = capture(Command::new("jj").args(["config", "list", "--repo", "pr"]))?;
    Ok(listing.lines().filter_map(Key::from_listing).collect())
}

/// Run `command` to completion.
fn run(command: &mut Command) -> Result<(), Error> {
    let status = command.status().map_err(|source| Error::Io {
        program: program(command),
        source,
    })?;
    if status.success() {
        Ok(())
    } else {
        Err(Error::Child {
            program: program(command),
            status,
            stderr: Vec::new(),
        })
    }
}

/// Run `command` and return its standard output.
fn capture(command: &mut Command) -> Result<String, Error> {
    let output = command.output().map_err(|source| Error::Io {
        program: program(command),
        source,
    })?;
    if !output.status.success() {
        return Err(Error::Child {
            program: program(command),
            status: output.status,
            stderr: output.stderr,
        });
    }
    String::from_utf8(output.stdout).map_err(|source| Error::Utf8 {
        program: program(command),
        source,
    })
}

fn program(command: &Command) -> String {
    command.get_program().to_string_lossy().into_owned()
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

mod error {
    use std::{fmt, io, process::ExitStatus, string::FromUtf8Error};

    /// Everything that can end a sync early, one variant per kind.
    pub enum Error {
        /// This program was invoked under an unrecognized name.
        Name,
        /// A child process could not be started or awaited.
        Io { program: String, source: io::Error },
        /// A child process ran and failed.  Its standard error rides along
        /// when it was captured rather than inherited.
        Child {
            program: String,
            status: ExitStatus,
            stderr: Vec<u8>,
        },
        /// A child process wrote something other than UTF-8.
        Utf8 {
            program: String,
            source: FromUtf8Error,
        },
        /// `gh pr list` printed unreadable JSON.
        Json(serde_json::Error),
        /// No remote is named `origin`, and there is not exactly one.
        NoOrigin,
        /// The remote URL fits no recognized git URL form.
        RemoteUrl(String),
    }

    impl Error {
        /// The code for main to end the process with: a failed child's own
        /// exit code where there is one, otherwise 1.
        pub fn code(&self) -> i32 {
            match self {
                Self::Child { status, .. } => status.code().unwrap_or(1),
                _ => 1,
            }
        }

        /// Captured standard error awaiting forwarding; empty for every other
        /// kind of error.
        pub fn stderr(&self) -> &[u8] {
            match self {
                Self::Child { stderr, .. } => stderr,
                _ => &[],
            }
        }
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Name => write!(f, "executable name should be pr-sync or jgs"),
                Self::Io { program, source } => write!(f, "running {program}: {source}"),
                Self::Child {
                    program, status, ..
                } => write!(f, "{program}: {status}"),
                Self::Utf8 { program, source } => {
                    write!(f, "{program} wrote invalid UTF-8: {source}")
                }
                Self::Json(source) => write!(f, "reading pull requests: {source}"),
                Self::NoOrigin => write!(f, "expected an origin remote"),
                Self::RemoteUrl(url) => write!(f, "{url}: unsupported remote URL"),
            }
        }
    }
}
