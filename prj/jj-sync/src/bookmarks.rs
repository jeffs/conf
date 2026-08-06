//! Hand jj's bookmarks to git.
//!
//! A bookmark lives in jj's own store, and git sees it only once jj has written
//! it out as a branch under `refs/heads`.  jj writes them itself at the end of
//! every command that changes the repository -- but only in the one workspace
//! it colocates.  Work done in a secondary workspace, which is where this
//! program earns its keep, leaves git's branches behind until `jj git export`
//! catches them up.

use std::process::Command;

use crate::{error::Error, jj, process};

/// Write jj's bookmarks out as git branches.
///
/// Silent, and cheap, where git already holds what jj does: jj compares the two
/// and writes neither a ref nor an operation when they agree.  A bookmark git
/// will not take, such as one named as the parent directory of another, it
/// warns about and skips, exporting the rest; that warning survives the silence
/// the rest is asked for.
///
/// The working copy is left unsnapshotted, since a bookmark moves with the
/// commit it names rather than with the files.  A bookmark on the working-copy
/// commit itself therefore reaches git as of the last snapshot, whoever took
/// it.
///
/// Where the current directory is in no workspace, or in one jj keeps its
/// commits outside git for, there is nothing to export and nothing is run.
///
/// # Errors
///
/// Fails if jj cannot be run or refuses.
pub fn export() -> Result<(), Error> {
    let Some(root) = jj::workspace_root()? else {
        return Ok(());
    };
    if jj::git_dir(&root)?.is_none() {
        return Ok(());
    }
    process::run(Command::new("jj").args(["--quiet", "--ignore-working-copy", "git", "export"]))
}
