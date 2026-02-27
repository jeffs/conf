//! Spawns a hard-coded editor program, removing the a/ and b/ prefixes from
//! paths (that appear to have been) produced by git diff.

use std::{
    env, ffi, fs, io,
    os::unix::{ffi::OsStrExt, process::CommandExt},
    process::{Command, ExitCode},
};

fn main() -> io::Result<ExitCode> {
    let args = env::args_os().skip(1).map(|s| {
        let bytes = s.as_encoded_bytes();
        if bytes.starts_with(b"a/") || bytes.starts_with(b"b/") && fs::exists(&s).is_ok_and(|b| !b)
        {
            ffi::OsStr::from_bytes(&bytes[2..]).to_owned()
        } else {
            s
        }
    });
    let err = Command::new("hx").args(args).exec();
    panic!("{err}");
}
