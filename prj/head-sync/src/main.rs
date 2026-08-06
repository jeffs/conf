//! Point this Jujutsu workspace's Git HEAD at the parent of its working copy,
//! so that tools reading the repo through git see the change jj is holding.
//!
//! The work, and the reasoning behind it, is [`jjkit::head`].  This program is
//! the way to run one pass of it from a shell -- `watch-jj` does so on every
//! refresh -- and so says nothing at all when there is nothing to report: a
//! workspace jj keeps colocated, a directory in no workspace, and a HEAD
//! already in place are all quiet successes.

use std::io::Write as _;

use jjkit::head;

fn main() {
    if let Err(error) = head::sync() {
        std::io::stderr().write_all(error.stderr()).ok();
        eprintln!("head-sync: {error}");
        std::process::exit(error.code());
    }
}
