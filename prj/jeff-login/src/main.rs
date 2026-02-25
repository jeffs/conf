//! Login shell launcher. See [`main`] for notes.

use std::{
    env, ffi,
    os::unix::process::CommandExt,
    path::{Path, PathBuf},
    process,
};

fn path_join<P: AsRef<Path>, I: IntoIterator<Item = P>>(dirs: I) -> ffi::OsString {
    dirs.into_iter()
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
    let shell = env::var_os("JEFF_LOGIN_SHELL");

    let home_path =
        ["usr/bin", "conf/bin", ".local/bin", ".cargo/bin", "go/bin"].map(|dir| home.join(dir));
    let sys_path = [
        "/usr/local/go/bin",
        "/opt/homebrew/bin",
        "/usr/local/bin",
        "/usr/bin",
        "/bin",
        "/usr/sbin",
        "/sbin",
        "/Library/Developer/CommandLineTools/usr/bin",
    ];
    let path = home_path
        .iter()
        .map(PathBuf::as_path)
        .chain(sys_path.map(Path::new));

    let err = process::Command::new(shell.as_deref().unwrap_or(ffi::OsStr::new("/bin/sh")))
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
        .env("PATH", path_join(path))
        .exec();

    // exec should have replaced the current process.
    eprintln!("executing login shell: {err}");
    process::exit(1);
}
