use std::{
    os::unix::process::{CommandExt, ExitStatusExt},
    process::Command,
};

fn main() {
    let bookmark = std::env::args_os()
        .nth(1)
        .expect("error: expected bookmark name");

    let status = Command::new("jj")
        .args(["bookmark", "create"])
        .arg(&bookmark)
        .args(std::env::args_os().skip(2)) // -r CHANGE_ID
        .status()
        .expect("running jj");

    if !status.success() {
        std::process::exit(status.into_raw())
    }

    let error = Command::new("jj")
        .args(["bookmark", "track"])
        .arg(bookmark)
        .exec();

    eprintln!("error: {error}");
    std::process::exit(1);
}
