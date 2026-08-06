//! Plumbing shared by the Jujutsu wrappers in this workspace.
//!
//! [`process`] runs child processes and [`jj`] asks jj about the repository it
//! is standing in.  The rest keep a secondary workspace current for whoever
//! reads it through git: [`head`] its Git HEAD, [`bookmarks`] its branches, and
//! [`stale`] the working copy both describe.  Every module reports failure as
//! the one [`Error`], so a wrapper can hand any of them the same treatment on
//! the way out.

pub mod bookmarks;
pub mod error;
pub mod head;
pub mod jj;
pub mod process;
pub mod stale;

pub use error::Error;
