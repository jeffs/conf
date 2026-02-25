//! Login shell launcher. See [`main`] for notes.

use std::{
    env, ffi,
    os::unix::process::CommandExt,
    path::{Path, PathBuf},
    process,
};

fn path_join<P: AsRef<Path>>(dirs: &[P]) -> ffi::OsString {
    dirs.iter()
        .enumerate()
        .fold(ffi::OsString::new(), |mut path, (index, dir)| {
            if index > 0 {
                path.push(":");
            }
            path.push(dir.as_ref());
            path
        })
}

fn main() {
    let home = env::home_dir().expect("home dir");

    let path = [
        home.join("usr/bin"),
        home.join("conf/bin"),
        home.join(".local/bin"),
        home.join(".cargo/bin"),
        home.join("go/bin"),
        PathBuf::from("/usr/local/go/bin"),
        PathBuf::from("/opt/homebrew/bin"),
        PathBuf::from("/usr/local/bin"),
        PathBuf::from("/usr/bin"),
        PathBuf::from("/bin"),
        PathBuf::from("/usr/sbin"),
        PathBuf::from("/sbin"),
        PathBuf::from("/Library/Developer/CommandLineTools/usr/bin"),
    ];

    let err = process::Command::new(home.join("usr/bin/nu"))
        .arg("--login")
        .envs([
            ("EDITOR", "hx"),
            ("LESS", "-FRX -j5"),
            ("MANPAGER", "col -b | bat -pl man"),
            ("HOMEBREW_NO_ENV_HINTS", "true"),
            ("RUSTC_WRAPPER", "/opt/homebrew/bin/sccache"),
            ("ENABLE_LSP_TOOL", "1"),
            ("ENABLE_LSP_TOOLS", "1"),
            ("GRIT_TRUNKS", "dev,main,master,trunk"),
        ])
        .envs([
            ("XDG_CONFIG_HOME", home.join(".config")),
            ("FZF_DEFAULT_OPTS_FILE", home.join("conf/etc/fzf")),
            ("RIPGREP_CONFIG_PATH", home.join("conf/etc/ripgreprc")),
            (
                "COPILOT_CUSTOM_INSTRUCTIONS_DIRS",
                home.join("conf/etc/copilot/instructions.md"),
            ),
            ("JUMP_PREFIXES", home.join("conf/etc/jump")),
            ("HELIX_RUNTIME", home.join("usr/src/helix/runtime")),
            ("JUMP_HOME", home),
        ])
        .env("PATH", path_join(path.as_slice()))
        .exec();

    // exec should have replaced the current process.
    eprintln!("executing login shell: {err}");
    process::exit(1);
}
