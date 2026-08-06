//! Keep a Jujutsu workspace's Git HEAD at the parent of its working copy.
//!
//! jj colocates at most one workspace of a repository: a `.git` sits beside
//! it, and jj moves that HEAD along with the workspace's `@-`.  Every other
//! workspace has no `.git` at all, so anything reading the repo through git --
//! Helix's diff gutter, say -- has nothing to compare the files against, and
//! shows a workspace full of changes as unchanged.
//!
//! A linked git worktree supplies what is missing.  It is git's one HEAD that
//! belongs to a directory rather than to the repository: a `.git` *file* in
//! the workspace names a directory under the shared git directory, and the
//! HEAD in there is that workspace's alone.  Pointing it at `@-` makes git
//! agree with jj about what is being changed, and moves nothing in the working
//! copy, which remains jj's.
//!
//! jj neither maintains this nor objects to it: it does not snapshot `.git`,
//! and counts a workspace as colocated only when the git directory sits beside
//! it, which the shared one never does.  So HEAD is ours to keep current, and
//! [`sync`] is one pass of doing so -- cheap enough to call on a timer, since a
//! HEAD already at `@-` costs one `jj log` and a handful of file reads.

use std::{
    ffi::OsStr,
    fs, io,
    os::unix::ffi::OsStrExt as _,
    path::{Path, PathBuf},
    process::Command,
};

use crate::{
    error::Error,
    jj::{self, CommitId},
    process,
};

/// What one pass of [`sync`] found to do.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[must_use]
pub enum Outcome {
    /// The current directory is in no workspace.
    NoWorkspace,
    /// The repository stores its commits somewhere other than git.
    NotGit,
    /// The workspace is colocated, and jj moves its HEAD itself.
    Colocated,
    /// The working copy has no parent to point at: it sits on the root
    /// commit, which is jj's alone and has no git commit behind it.
    NoParent,
    /// The parent is conflicted, and the git commit behind it holds jj's
    /// conflict scaffolding rather than the files.  HEAD is left where it is,
    /// since pointing at that commit would diff every file against a
    /// three-sided stand-in.
    Conflicted,
    /// HEAD already names the parent of the working copy.
    Unchanged,
    /// HEAD now names the parent of the working copy.
    Updated,
}

/// Point the current workspace's Git HEAD at `@-`, building the worktree that
/// holds that HEAD if this is the first pass.
///
/// The index moves with HEAD, so `git status` in the workspace reports the
/// same change `jj status` does.
///
/// # Errors
///
/// Fails if jj or git cannot be run or refuses, if the repository's files
/// cannot be read, or if either the workspace's `.git` or the worktree name it
/// wants belongs to something else.
pub fn sync() -> Result<Outcome, Error> {
    let Some(root) = jj::workspace_root()? else {
        return Ok(Outcome::NoWorkspace);
    };
    let Some(git_dir) = jj::git_dir(&root)? else {
        return Ok(Outcome::NotGit);
    };
    if jj::is_colocated(&root, &git_dir) {
        return Ok(Outcome::Colocated);
    }

    let worktree = Worktree::new(&git_dir, root)?;
    let Some(parent) = jj::commit("@-")? else {
        return Ok(Outcome::NoParent);
    };
    if parent.conflict {
        return Ok(Outcome::Conflicted);
    }

    let link = worktree.link()?;
    if link == Link::Ours && matches!(worktree.head()?, Head::Detached(id) if id == parent.id) {
        worktree.ignore_state()?;
        return Ok(Outcome::Unchanged);
    }
    worktree.register(&parent.id, link)?;
    worktree.point(&parent.id)?;
    Ok(Outcome::Updated)
}

/// The linked git worktree that gives one workspace a HEAD of its own.
struct Worktree {
    /// `<git dir>/worktrees/<name>`, holding this worktree's HEAD.
    admin: PathBuf,
    /// The workspace root the worktree describes.
    root: PathBuf,
    /// `<root>/.git`, the file naming `admin`.
    link: PathBuf,
}

/// What `<root>/.git` holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Link {
    /// Nothing: the worktree has yet to be built.
    Absent,
    /// This worktree's own link.
    Ours,
    /// A link to another worktree of the same repository, which is what a
    /// rename of the workspace directory leaves behind.
    Stale,
    /// Something belonging to someone else.
    Stranger,
}

/// What a worktree's HEAD names.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Head {
    /// Nothing: the worktree has yet to be built.
    Absent,
    Detached(CommitId),
    /// A branch, which a git command run in the workspace put there.
    Branch,
}

impl Worktree {
    /// The worktree for `root`, named after its directory.
    fn new(git_dir: &Path, root: PathBuf) -> Result<Self, Error> {
        let name = root
            .file_name()
            .ok_or_else(|| Error::Unnamed(root.clone()))?;
        Ok(Self {
            admin: git_dir.join("worktrees").join(name),
            link: root.join(".git"),
            root,
        })
    }

    fn link(&self) -> Result<Link, Error> {
        if self.link.is_dir() {
            return Ok(Link::Stranger);
        }
        let Some(found) = read(&self.link)? else {
            return Ok(Link::Absent);
        };
        if found == line("gitdir: ", &self.admin) {
            return Ok(Link::Ours);
        }
        let sibling =
            admin_named(&found).is_some_and(|admin| admin.parent() == self.admin.parent());
        Ok(if sibling { Link::Stale } else { Link::Stranger })
    }

    fn head(&self) -> Result<Head, Error> {
        let Some(found) = read(&self.admin.join("HEAD"))? else {
            return Ok(Head::Absent);
        };
        let text = String::from_utf8_lossy(&found);
        Ok(CommitId::parse(&text).map_or(Head::Branch, Head::Detached))
    }

    /// The workspace already registered under this worktree's name, where it
    /// is not this one.  Two workspaces of a repository share a name when
    /// their directories do, which `jj workspace add --name` permits.
    fn registered_to(&self) -> Result<Option<PathBuf>, Error> {
        let Some(found) = read(&self.admin.join("gitdir"))? else {
            return Ok(None);
        };
        let found = path_of(&found);
        Ok((found != self.link).then_some(found))
    }

    /// Register the worktree with git, leaving HEAD wherever it already
    /// points.  Both ways this can be refused are settled before anything is
    /// written, so a workspace this crate may not claim leaves no trace in the
    /// shared git directory.
    fn register(&self, commit: &CommitId, link: Link) -> Result<(), Error> {
        if link == Link::Stranger {
            return Err(Error::Occupied(self.link.clone()));
        }
        if let Some(other) = self.registered_to()? {
            return Err(Error::Registered {
                admin: self.admin.clone(),
                other,
            });
        }

        fs::create_dir_all(&self.admin).map_err(|source| Error::file(&self.admin, source))?;
        write(&self.admin.join("gitdir"), &line("", &self.link))?;
        write(&self.admin.join("commondir"), b"../..\n")?;
        seed(&self.admin.join("HEAD"), format!("{commit}\n").as_bytes())?;
        write(&self.link, &line("gitdir: ", &self.admin))?;
        self.ignore_state()
    }

    /// Keep git's hands off the workspace's own state, as jj does for a
    /// colocated workspace by writing the same file.  Without it `.jj` is
    /// merely untracked, and a `git clean` in the workspace takes it.
    fn ignore_state(&self) -> Result<(), Error> {
        seed(&self.root.join(".jj").join(".gitignore"), b"/*\n")
    }

    /// Move HEAD and the index to `commit`, leaving every file alone.
    ///
    /// HEAD is written without following it, so that a branch a git command
    /// left the workspace on is detached from rather than dragged along.
    fn point(&self, commit: &CommitId) -> Result<(), Error> {
        process::run(
            self.git()
                .args(["update-ref", "--no-deref", "HEAD"])
                .arg(commit.as_str()),
        )?;
        process::run(self.git().args(["reset", "--quiet"]))
    }

    fn git(&self) -> Command {
        let mut command = Command::new("git");
        command.arg("-C").arg(&self.root);
        command
    }
}

/// One line of a git pointer file: `prefix`, then `path`.
fn line(prefix: &str, path: &Path) -> Vec<u8> {
    let mut bytes = prefix.as_bytes().to_vec();
    bytes.extend_from_slice(path.as_os_str().as_bytes());
    bytes.push(b'\n');
    bytes
}

/// The path a git pointer file holds, without its trailing newline.
fn path_of(contents: &[u8]) -> PathBuf {
    let bytes = contents.strip_suffix(b"\n").unwrap_or(contents);
    PathBuf::from(OsStr::from_bytes(bytes))
}

/// The worktree directory a `.git` file names, where it names one.
fn admin_named(link: &[u8]) -> Option<PathBuf> {
    link.strip_prefix(b"gitdir: ").map(path_of)
}

/// The contents of `path`, or `None` where there is no such file.
fn read(path: &Path) -> Result<Option<Vec<u8>>, Error> {
    match fs::read(path) {
        Ok(found) => Ok(Some(found)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(source) => Err(Error::file(path, source)),
    }
}

/// Write `contents` where there is no file yet.
fn seed(path: &Path, contents: &[u8]) -> Result<(), Error> {
    if path.exists() {
        Ok(())
    } else {
        write(path, contents)
    }
}

fn write(path: &Path, contents: &[u8]) -> Result<(), Error> {
    fs::write(path, contents).map_err(|source| Error::file(path, source))
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use tempfile::TempDir;

    use super::{Head, Link, Worktree, line, seed};
    use crate::jj::CommitId;

    const ID: &str = "071f4edd73c0eef6508c4ba2445cb2af1bd5ebd4";

    /// A worktree named `ws`, in a repository and a workspace that exist.
    fn worktree(dir: &Path) -> Worktree {
        let git_dir = dir.join("repo.git");
        let root = dir.join("ws");
        fs::create_dir_all(git_dir.join("worktrees")).unwrap();
        fs::create_dir_all(root.join(".jj")).unwrap();
        Worktree::new(&git_dir, root).unwrap()
    }

    #[test]
    fn writes_a_pointer_line_ending_in_a_newline() {
        assert_eq!(line("gitdir: ", Path::new("/x/y")), b"gitdir: /x/y\n");
        assert_eq!(line("", Path::new("/x/y")), b"/x/y\n");
    }

    #[test]
    fn names_the_worktree_after_the_workspace_directory() {
        let worktree = Worktree::new(Path::new("/r/.git"), "/w/ws".into()).unwrap();
        assert_eq!(worktree.admin, Path::new("/r/.git/worktrees/ws"));
        assert_eq!(worktree.link, Path::new("/w/ws/.git"));
    }

    #[test]
    fn refuses_a_workspace_whose_path_names_no_directory() {
        assert!(Worktree::new(Path::new("/r/.git"), "/".into()).is_err());
    }

    #[test]
    fn tells_its_own_link_from_a_stale_one_and_a_stranger() {
        let dir = TempDir::new().unwrap();
        let worktree = worktree(dir.path());
        assert_eq!(worktree.link().unwrap(), Link::Absent);

        for (contents, expected) in [
            (line("gitdir: ", &worktree.admin), Link::Ours),
            (
                line("gitdir: ", &worktree.admin.with_file_name("renamed")),
                Link::Stale,
            ),
            (line("gitdir: ", Path::new("/elsewhere/.git")), Link::Stranger),
            (b"not a link at all\n".to_vec(), Link::Stranger),
        ] {
            fs::write(&worktree.link, &contents).unwrap();
            assert_eq!(worktree.link().unwrap(), expected, "{contents:?}");
        }
    }

    #[test]
    fn treats_a_git_directory_as_a_stranger() {
        let dir = TempDir::new().unwrap();
        let worktree = worktree(dir.path());
        fs::create_dir(&worktree.link).unwrap();
        assert_eq!(worktree.link().unwrap(), Link::Stranger);
    }

    #[test]
    fn tells_a_detached_head_from_a_branch() {
        let dir = TempDir::new().unwrap();
        let worktree = worktree(dir.path());
        assert_eq!(worktree.head().unwrap(), Head::Absent);

        fs::create_dir_all(&worktree.admin).unwrap();
        let head = worktree.admin.join("HEAD");
        fs::write(&head, format!("{ID}\n")).unwrap();
        assert_eq!(
            worktree.head().unwrap(),
            Head::Detached(CommitId::parse(ID).unwrap())
        );

        fs::write(&head, "ref: refs/heads/mine\n").unwrap();
        assert_eq!(worktree.head().unwrap(), Head::Branch);
    }

    #[test]
    fn registers_a_workspace_and_leaves_a_second_pass_alone() {
        let dir = TempDir::new().unwrap();
        let worktree = worktree(dir.path());
        let commit = CommitId::parse(ID).unwrap();

        worktree.register(&commit, Link::Absent).unwrap();
        assert_eq!(worktree.link().unwrap(), Link::Ours);
        assert_eq!(worktree.head().unwrap(), Head::Detached(commit.clone()));
        assert_eq!(
            fs::read(worktree.admin.join("gitdir")).unwrap(),
            line("", &worktree.link)
        );
        assert_eq!(
            fs::read(worktree.admin.join("commondir")).unwrap(),
            b"../..\n"
        );
        assert_eq!(
            fs::read(worktree.root.join(".jj").join(".gitignore")).unwrap(),
            b"/*\n"
        );

        // A HEAD git has moved since is not seeded back to where it began.
        fs::write(worktree.admin.join("HEAD"), "ref: refs/heads/mine\n").unwrap();
        worktree.register(&commit, Link::Ours).unwrap();
        assert_eq!(worktree.head().unwrap(), Head::Branch);
    }

    #[test]
    fn writes_nothing_for_a_workspace_it_may_not_claim() {
        let dir = TempDir::new().unwrap();
        let worktree = worktree(dir.path());
        let commit = CommitId::parse(ID).unwrap();

        fs::write(&worktree.link, "gitdir: /elsewhere\n").unwrap();
        assert!(worktree.register(&commit, Link::Stranger).is_err());
        assert!(!worktree.admin.exists());

        // A name another workspace holds is refused just as completely.
        fs::remove_file(&worktree.link).unwrap();
        fs::create_dir_all(&worktree.admin).unwrap();
        fs::write(worktree.admin.join("gitdir"), "/another/ws/.git\n").unwrap();
        assert!(worktree.register(&commit, Link::Absent).is_err());
        assert!(!worktree.link.exists());
    }

    #[test]
    fn seeds_only_where_there_is_no_file() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("f");
        seed(&path, b"first").unwrap();
        seed(&path, b"second").unwrap();
        assert_eq!(fs::read(&path).unwrap(), b"first");
    }
}
