//! Fetch a GitHub pull request into a local Jujutsu bookmark.
//!
//! jj imports bookmarks only from `refs/heads/`, and refuses to fetch a refspec
//! sourced anywhere else.  So git fetches the pull request head into
//! `refs/heads/`, and jj imports it from there as an ordinary bookmark.

use std::{os::unix::process::CommandExt, process::Command};

use bookmark::Bookmark;
use clap::Parser;
use git_dir::GitDir;
use pull::Pull;
use remote::Remote;

#[derive(Parser)]
#[command(about = "Fetch a GitHub pull request into a local Jujutsu bookmark")]
struct Cli {
    /// Pull request number, as shown in the GitHub UI
    pull: Pull,

    /// Remote whose repository the pull request was opened against
    #[arg(short, long, default_value = "origin")]
    remote: Remote,

    /// Bookmark to point at the pull request head [default: pr-<PULL>]
    #[arg(short, long)]
    bookmark: Option<Bookmark>,

    /// Leave the working copy alone instead of `jj new`-ing onto the head
    #[arg(short = 'n', long)]
    no_new: bool,
}

fn main() {
    let cli = Cli::parse();
    let bookmark = cli.bookmark.unwrap_or_else(|| Bookmark::for_pull(cli.pull));
    let git_dir = GitDir::locate().unwrap_or_else(|error| fail(&error));

    // `+` so that a force-pushed pull request still moves the bookmark; the
    // `pr-*` namespace belongs to this tool, and the commit it displaces stays
    // reachable through `jj op log`.
    let refspec = format!("+{}:{}", cli.pull.head_ref(), bookmark.git_ref());
    run(Command::new("git")
        .arg("--git-dir")
        .arg(git_dir.as_path())
        .arg("fetch")
        .arg(cli.remote.as_str())
        .arg(&refspec));

    // Imports the new ref in the hidden layout; a colocated repo has already
    // done so at load time.  Quiet, because git has just reported the ref.
    run(Command::new("jj").args(["git", "import", "--quiet"]));

    if cli.no_new {
        return;
    }
    let error = Command::new("jj").arg("new").arg(bookmark.as_str()).exec();
    fail(&format!("running jj: {error}"));
}

/// Run `command` to completion, ending this process if it fails.
fn run(command: &mut Command) {
    let status = match command.status() {
        Ok(status) => status,
        Err(error) => {
            let program = command.get_program().display();
            fail(&format!("running {program}: {error}"));
        }
    };
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

fn fail(message: &str) -> ! {
    eprintln!("jjpr: {message}");
    std::process::exit(1)
}

mod pull {
    use std::{fmt, str::FromStr};

    /// A pull request number, as it appears in the GitHub UI.
    #[derive(Clone, Copy)]
    pub struct Pull(u32);

    impl Pull {
        /// The ref GitHub publishes on the *base* repository for this pull
        /// request's head commit.  Forks publish here too, so one remote and no
        /// GitHub API call suffice.
        pub fn head_ref(self) -> String {
            format!("refs/pull/{}/head", self.0)
        }
    }

    impl fmt::Display for Pull {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    impl FromStr for Pull {
        type Err = String;

        /// Accepts `2388` or `#2388`, the latter being what one copies out of a
        /// commit message or a chat window.
        fn from_str(text: &str) -> Result<Self, Self::Err> {
            match text.trim_start_matches('#').parse::<u32>() {
                Ok(0) | Err(_) => Err(format!("not a pull request number: {text}")),
                Ok(number) => Ok(Self(number)),
            }
        }
    }
}

mod bookmark {
    use std::{convert::Infallible, str::FromStr};

    use crate::pull::Pull;

    /// A Jujutsu bookmark name.
    #[derive(Clone)]
    pub struct Bookmark(String);

    impl Bookmark {
        pub fn for_pull(pull: Pull) -> Self {
            Self(format!("pr-{pull}"))
        }

        /// The git ref backing this bookmark, and so the destination a fetch
        /// must name for jj to import it.
        pub fn git_ref(&self) -> String {
            format!("refs/heads/{}", self.0)
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    impl FromStr for Bookmark {
        type Err = Infallible;

        fn from_str(text: &str) -> Result<Self, Self::Err> {
            Ok(Self(text.to_owned()))
        }
    }
}

mod remote {
    use std::{convert::Infallible, str::FromStr};

    /// A git remote name.
    #[derive(Clone)]
    pub struct Remote(String);

    impl Remote {
        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    impl FromStr for Remote {
        type Err = Infallible;

        fn from_str(text: &str) -> Result<Self, Self::Err> {
            Ok(Self(text.to_owned()))
        }
    }
}

mod git_dir {
    use std::{
        fs,
        path::{Path, PathBuf},
        process::Command,
    };

    /// The git directory holding a Jujutsu repository's commits.
    pub struct GitDir(PathBuf);

    impl GitDir {
        /// jj keeps its git repository either beside the workspace (`.git`,
        /// which `git.colocate` makes the default) or hidden inside `.jj`.
        /// Both layouts record its location in `store/git_target`, which is
        /// therefore the one place worth asking.
        pub fn locate() -> Result<Self, String> {
            let repo = repo_dir(&workspace_root()?)?;
            let store = repo.join("store");
            let kind = read_trimmed(&store.join("type"))?;
            if kind != "git" {
                return Err(format!("repository is backed by {kind}, not git"));
            }
            let target = read_trimmed(&store.join("git_target"))?;
            canonicalize(&store.join(target)).map(Self)
        }

        pub fn as_path(&self) -> &Path {
            &self.0
        }
    }

    fn workspace_root() -> Result<PathBuf, String> {
        let output = Command::new("jj")
            .args(["--ignore-working-copy", "root"])
            .output()
            .map_err(|error| format!("running jj: {error}"))?;
        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).trim().to_owned());
        }
        Ok(PathBuf::from(
            String::from_utf8_lossy(&output.stdout).trim(),
        ))
    }

    /// `.jj/repo` is the repository directory, except in a secondary workspace,
    /// where it is a file naming the workspace that owns the repository.
    fn repo_dir(workspace: &Path) -> Result<PathBuf, String> {
        let jj = workspace.join(".jj");
        let repo = jj.join("repo");
        if repo.is_file() {
            canonicalize(&jj.join(read_trimmed(&repo)?))
        } else {
            Ok(repo)
        }
    }

    fn read_trimmed(path: &Path) -> Result<String, String> {
        fs::read_to_string(path)
            .map(|text| text.trim().to_owned())
            .map_err(|error| format!("reading {}: {error}", path.display()))
    }

    fn canonicalize(path: &Path) -> Result<PathBuf, String> {
        fs::canonicalize(path).map_err(|error| format!("resolving {}: {error}", path.display()))
    }
}
