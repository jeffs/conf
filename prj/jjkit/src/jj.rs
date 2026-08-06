//! What this crate asks jj, and its repository on disk, about where it is.
//!
//! Where the repository lives is read from the files under `.jj`, which cost
//! no subprocess -- worth having for a caller that runs once a second.  What a
//! revset names is jj's own question to answer, and the one `jj` this crate
//! runs.

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use crate::{error::Error, process};

pub use commit::{Commit, CommitId};

/// The root of the workspace containing the current directory, or `None`
/// where no ancestor of it holds a `.jj` directory -- which is jj's own rule
/// for finding a workspace.
///
/// The path is canonical, so that it compares equal to paths this crate
/// resolves through the repository.
///
/// # Errors
///
/// Fails if the current directory cannot be read or resolved.
pub fn workspace_root() -> Result<Option<PathBuf>, Error> {
    let cwd = env::current_dir().map_err(|source| Error::file(Path::new("."), source))?;
    let cwd = canonicalize(&cwd)?;
    Ok(cwd
        .ancestors()
        .find(|dir| dir.join(".jj").is_dir())
        .map(Path::to_path_buf))
}

/// The git directory holding `workspace`'s commits, or `None` where the
/// repository is backed by something other than git.
///
/// jj keeps that directory either beside the workspace (`.git`, which
/// `git.colocate` makes the default) or hidden inside `.jj`.  Both layouts
/// record its location in `store/git_target`, which is therefore the one
/// place worth asking.
///
/// # Errors
///
/// Fails if the repository's own files cannot be read or resolved.
pub fn git_dir(workspace: &Path) -> Result<Option<PathBuf>, Error> {
    let store = repo_dir(workspace)?.join("store");
    if read_trimmed(&store.join("type"))? != "git" {
        return Ok(None);
    }
    let target = read_trimmed(&store.join("git_target"))?;
    canonicalize(&store.join(target)).map(Some)
}

/// Whether jj keeps `git_dir` beside `root`, and so moves its HEAD along with
/// the working copy itself.
///
/// `<root>/.git` is resolved rather than compared as written, since a
/// colocated checkout may reach its git directory through a symlink.
#[must_use]
pub fn is_colocated(root: &Path, git_dir: &Path) -> bool {
    fs::canonicalize(root.join(".git")).is_ok_and(|path| path == git_dir)
}

/// The commit `revset` resolves to, or the first of them where it resolves to
/// several -- as `@-` does at a merge.
///
/// `None` where the revset names nothing a Git HEAD can point at: no commit
/// at all, or jj's virtual root commit, which has no git object behind it.
///
/// # Errors
///
/// Fails if jj cannot be run, refuses the revset, or describes the commit in
/// terms this crate cannot read.
pub fn commit(revset: &str) -> Result<Option<Commit>, Error> {
    let mut command = Command::new("jj");
    command.args([
        "--ignore-working-copy",
        "log",
        "--no-graph",
        "-r",
        revset,
        "-T",
        "commit_id ++ \" \" ++ if(conflict, \"conflict\", \"-\") ++ \"\\n\"",
    ]);
    let listing = process::capture(&mut command)?;
    let unreadable = || Error::Parse {
        program: process::program(&command),
        text: listing.clone(),
    };

    let Some(line) = listing.lines().next() else {
        return Ok(None);
    };
    let (id, state) = line.split_once(' ').ok_or_else(unreadable)?;
    if id.bytes().all(|byte| byte == b'0') {
        return Ok(None);
    }
    let id = CommitId::parse(id).ok_or_else(unreadable)?;
    Ok(Some(Commit {
        id,
        conflict: state == "conflict",
    }))
}

/// `.jj/repo` is the repository directory, except in a secondary workspace,
/// where it is a file naming the workspace that owns the repository.
fn repo_dir(workspace: &Path) -> Result<PathBuf, Error> {
    let jj = workspace.join(".jj");
    let repo = jj.join("repo");
    if repo.is_file() {
        canonicalize(&jj.join(read_trimmed(&repo)?))
    } else {
        Ok(repo)
    }
}

fn read_trimmed(path: &Path) -> Result<String, Error> {
    fs::read_to_string(path)
        .map(|text| text.trim().to_owned())
        .map_err(|source| Error::file(path, source))
}

fn canonicalize(path: &Path) -> Result<PathBuf, Error> {
    fs::canonicalize(path).map_err(|source| Error::file(path, source))
}

mod commit {
    use std::fmt;

    /// A commit, as much of one as a Git HEAD needs.
    pub struct Commit {
        pub id: CommitId,
        /// Whether jj holds the commit as a conflict, in which case the git
        /// commit behind it carries jj's conflict scaffolding -- several
        /// sides of each file, under `.jjconflict-*` -- in place of the files
        /// themselves.
        pub conflict: bool,
    }

    /// A git commit ID, as jj and git both spell it: forty hex digits.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct CommitId(String);

    impl CommitId {
        /// `None` for anything that is not a full hash naming an object: a
        /// symbolic ref, a blank line, or the all-zero ID, which stands for
        /// the absence of a commit.
        #[must_use]
        pub fn parse(text: &str) -> Option<Self> {
            let text = text.trim();
            let hash = text.len() == 40 && text.bytes().all(|b| b.is_ascii_hexdigit());
            let null = text.bytes().all(|b| b == b'0');
            (hash && !null).then(|| Self(text.to_owned()))
        }

        #[must_use]
        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    impl fmt::Display for CommitId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(&self.0)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::CommitId;

        #[test]
        fn reads_a_full_hash() {
            let text = "071f4edd73c0eef6508c4ba2445cb2af1bd5ebd4";
            assert_eq!(CommitId::parse(text).expect(text).as_str(), text);
        }

        #[test]
        fn rejects_anything_shorter_or_stranger() {
            for text in [
                "",
                "071f4edd",
                "ref: refs/heads/main",
                "071f4edd73c0eef6508c4ba2445cb2af1bd5ebdz",
            ] {
                assert!(CommitId::parse(text).is_none(), "{text}");
            }
        }

        #[test]
        fn rejects_the_null_id_that_names_no_object() {
            assert!(CommitId::parse(&"0".repeat(40)).is_none());
        }
    }
}
