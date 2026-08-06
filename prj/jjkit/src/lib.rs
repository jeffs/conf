//! Plumbing shared by the Jujutsu wrappers in this workspace.
//!
//! [`process`] runs child processes, [`jj`] asks jj about the repository it is
//! standing in, [`head`] keeps a secondary workspace's Git HEAD current, and
//! [`stale`] keeps its working copy current.  Every module reports failure as
//! the one [`Error`], so a wrapper can hand any of them the same treatment on
//! the way out.

pub mod error;
pub mod head;
pub mod jj;
pub mod process;
pub mod stale;

pub use error::Error;
