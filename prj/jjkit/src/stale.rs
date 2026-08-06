//! Catch up a working copy the repository has left behind.
//!
//! A workspace goes stale when the repository moves somewhere its working copy
//! cannot follow -- a `jj op undo` in a sibling workspace, say, rewinding past
//! the operation this one was last updated at.  jj then refuses every command
//! that snapshots the files, `jj git fetch` among them, until
//! `jj workspace update-stale` has caught the working copy up.
//!
//! Only jj can tell a stale working copy from one merely out of date, which
//! every command updates on its own, so [`update`] asks jj rather than
//! guessing: `jj workspace update-stale` does nothing where nothing is stale,
//! and `--quiet` keeps it from saying so.

use std::process::Command;

use crate::{error::Error, jj, process};

/// Catch the current workspace's working copy up with the repository, where jj
/// finds it stale.
///
/// Silent on success, and cheap where there is nothing to catch up: one jj
/// command, which records an operation only for the snapshot it takes.  It does
/// take one, as every jj command but an `--ignore-working-copy` one does.  So a
/// caller may run this before every sync, rather than wait to be told a working
/// copy is stale.
///
/// # Errors
///
/// Fails if jj cannot be run or refuses.  It refuses a workspace the repository
/// holds no working-copy commit for -- what `jj workspace forget` and an undone
/// `jj workspace add` leave behind -- that being a break past what
/// update-stale repairs.
pub fn update() -> Result<(), Error> {
    if jj::workspace_root()?.is_none() {
        return Ok(());
    }
    process::run(Command::new("jj").args(["--quiet", "workspace", "update-stale"]))
}
