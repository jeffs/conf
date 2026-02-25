use std::{
    env, ffi, io,
    os::unix::process::CommandExt,
    path::{Path, PathBuf},
    process::Command,
};

fn cargo_bin() -> PathBuf {
    env::home_dir().expect("home dir").join(".cargo/bin")
}

fn tput_cols() -> u32 {
    let stdout = Command::new("tput")
        .arg("cols")
        .output()
        .expect("tput cols")
        .stdout;
    String::from_utf8(stdout)
        .expect("tput cols should print UTF-8")
        .trim()
        .parse()
        .expect("tput cols should print a small natural number")
}

/// Executable target file
#[derive(Clone, Copy)]
enum Exe {
    // From Rust-Kart
    Edit,
    /// A modern `ls` replacement
    Eza,
    /// Markdown reader written in Go
    Glow,
    /// Jujutsu
    Jj,
}

impl Exe {
    fn exec<S: AsRef<ffi::OsStr>, I: IntoIterator<Item = S>>(self, args: I) -> io::Error {
        Command::new(self.path()).args(args).exec()
    }

    fn exec_with(
        self,
        inserted_args: impl IntoIterator<Item = impl AsRef<ffi::OsStr>>,
        args: impl IntoIterator<Item = impl AsRef<ffi::OsStr>>,
    ) -> io::Error {
        Command::new(self.path())
            .args(inserted_args)
            .args(args)
            .exec()
    }

    fn path(self) -> PathBuf {
        match self {
            Exe::Edit => cargo_bin().join("edit"),
            Exe::Eza => cargo_bin().join("eza"),
            Exe::Glow => "/opt/homebrew/bin/glow".into(),
            Exe::Jj => cargo_bin().join("jj"),
        }
    }
}

fn main() {
    let mut args = env::args_os();
    let Some(arg) = args.next() else {
        eprintln!("error: expected argv[0]");
        std::process::exit(1)
    };

    let Some(name) = Path::new(&arg).file_name().and_then(ffi::OsStr::to_str) else {
        eprintln!("error: expected Unicode file name at argv[0]");
        std::process::exit(1)
    };

    let err = match name {
        "e" => Exe::Edit.exec(args),
        "glow" => Exe::Glow.exec_with(["--pager", "--width", &tput_cols().to_string()], args),
        "jb" => Exe::Jj.exec_with(["bookmark"], args),
        "jbc" => Exe::Jj.exec_with(["bookmark", "create"], args),
        "jbd" => Exe::Jj.exec_with(["bookmark", "describe"], args),
        "jbdm" => Exe::Jj.exec_with(["bookmark", "describe", "--message"], args),
        "jbe" => Exe::Jj.exec_with(["edit"], args),
        "jbm" => Exe::Jj.exec_with(["bookmark", "move"], args),
        "jbs" => Exe::Jj.exec_with(["bookmark", "set"], args),
        "jbt" => Exe::Jj.exec_with(["bookmark", "track"], args),
        "jg" => Exe::Jj.exec_with(["git"], args),
        "jgf" => Exe::Jj.exec_with(["git", "fetch"], args),
        "jgi" => Exe::Jj.exec_with(["git", "init"], args),
        "jgp" => Exe::Jj.exec_with(["git", "push"], args),
        "jl" => Exe::Jj.exec_with(["log"], args),
        "jlr" => Exe::Jj.exec_with(["log", "--revisions"], args),
        "jn" => Exe::Jj.exec_with(["new"], args),
        "jnm" => Exe::Jj.exec_with(["new", "--message"], args),
        "t" | "tree" => Exe::Eza.exec_with(["-T"], args),
        _ => {
            eprintln!("error: {name}: bad alias");
            std::process::exit(2)
        }
    };

    eprintln!("error: {err}");
    std::process::exit(1);
}
