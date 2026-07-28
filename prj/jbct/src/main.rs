//! Create and track, and optionally push, a Jujutsu bookmark.

use std::{os::unix::process::CommandExt, process::Command};

/// Returns true if the executable name is "jbctp" (meaning we should push the
// new bookmark), and false if the name is "jbct" (indicating that we should
// not).
///
/// # Errors
///
/// Will return [`None`] if the executable cannot be determined, or is
// unrecognized.
fn parse_exe() -> Option<bool> {
    let exe = std::env::current_exe().ok()?;
    match exe.as_path().file_name()?.to_str()? {
        "jbctp" => Some(true),
        "jbct" => Some(false),
        _ => None,
    }
}

fn main() {
    let Some(push) = parse_exe() else {
        eprintln!("error: exe name should be jbct or jbctp");
        std::process::exit(1);
    };

    let Some(bookmark) = std::env::args_os().nth(1) else {
        eprintln!("error: expected bookmark name");
        std::process::exit(2);
    };

    // create
    run(Command::new("jj")
        .args(["bookmark", "create"])
        .arg(&bookmark)
        .args(std::env::args_os().skip(2))); // -r CHANGE_ID

    // track
    run(Command::new("jj")
        .args(["bookmark", "track"])
        .arg(&bookmark));

    if push {
        let error = Command::new("jj")
            .args(["git", "push", "--bookmark"])
            .arg(bookmark)
            .exec();
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

/// Run `command` to completion, ending this process if it fails.
fn run(command: &mut Command) {
    let status = command.status().expect("running jj");
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}
