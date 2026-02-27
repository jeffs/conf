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

fn t_depth(depth: u8) -> &'static [&'static str] {
    match depth {
        0 => &["-T", "--group-directories-first", "--git-ignore", "-L0"],
        1 => &["-T", "--group-directories-first", "--git-ignore", "-L1"],
        2 => &["-T", "--group-directories-first", "--git-ignore", "-L2"],
        3 => &["-T", "--group-directories-first", "--git-ignore", "-L3"],
        4 => &["-T", "--group-directories-first", "--git-ignore", "-L4"],
        5 => &["-T", "--group-directories-first", "--git-ignore", "-L5"],
        6 => &["-T", "--group-directories-first", "--git-ignore", "-L6"],
        7 => &["-T", "--group-directories-first", "--git-ignore", "-L7"],
        8 => &["-T", "--group-directories-first", "--git-ignore", "-L8"],
        9 => &["-T", "--group-directories-first", "--git-ignore", "-L9"],
        n => panic!("{n}: unsupported tree depth"),
    }
}

/// Executable target file
#[derive(Clone, Copy)]
enum Exe {
    /// Cat with wings
    Bat,
    /// Rust package manager and build tool
    Cargo,
    /// From Rust-Kart
    Edit,
    /// Rust REPL (`EValuation ConteXt for Rust`)
    Evcxr,
    /// A modern `ls` replacement
    Eza,
    /// Fuzzy finder
    Fzf,
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
            Exe::Bat => cargo_bin().join("bat"),
            Exe::Cargo => cargo_bin().join("cargo"), // Whoa, meta.
            Exe::Edit => cargo_bin().join("edit"),
            Exe::Evcxr => cargo_bin().join("evcxr"),
            Exe::Eza => cargo_bin().join("eza"),
            Exe::Fzf => "/opt/homebrew/bin/fzf".into(),
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
        "ef" => Exe::Fzf.exec_with(["--bind=enter:become(hx {})"], args),
        "fz" => Exe::Fzf.exec_with(["--preview=bat -p --color=always {}"], args),

        "glow" => Exe::Glow.exec_with(["--pager", "--width", &tput_cols().to_string()], args),
        "mat" => Exe::Bat.exec_with(["-pl", "man"], args),

        // TODO: Default to pedantic, but allow override in `Cargo.toml`.
        "clippy" => Exe::Cargo.exec_with(["clippy", "--all-targets", "--workspace"], args),
        "rust" => Exe::Evcxr.exec(args),

        "di" => Exe::Jj.exec_with(["diff"], args),
        "st" => Exe::Jj.exec_with(["status"], args),

        "jb" => Exe::Jj.exec_with(["bookmark"], args),
        "jbc" => Exe::Jj.exec_with(["bookmark", "create"], args),
        "jbd" => Exe::Jj.exec_with(["bookmark", "delete"], args),
        "jbm" => Exe::Jj.exec_with(["bookmark", "move"], args),
        "jbs" => Exe::Jj.exec_with(["bookmark", "set"], args),
        "jbt" => Exe::Jj.exec_with(["bookmark", "track"], args),
        "jd" => Exe::Jj.exec_with(["describe"], args),
        "jdm" => Exe::Jj.exec_with(["describe", "--message"], args),
        "je" => Exe::Jj.exec_with(["edit"], args),
        "jg" => Exe::Jj.exec_with(["git"], args),
        "jgf" => Exe::Jj.exec_with(["git", "fetch"], args),
        "jgi" => Exe::Jj.exec_with(["git", "init"], args),
        "jgp" => Exe::Jj.exec_with(["git", "push"], args),
        "jl" => Exe::Jj.exec_with(["log"], args),
        "jlr" => Exe::Jj.exec_with(["log", "--revisions"], args),
        "jn" => Exe::Jj.exec_with(["new"], args),
        "jnm" => Exe::Jj.exec_with(["new", "--message"], args),

        "tree" => Exe::Eza.exec_with(["-T", "--group-directories-first"], args),
        "t" => Exe::Eza.exec_with(["-T", "--group-directories-first", "--git-ignore"], args),
        "t1" => Exe::Eza.exec_with(t_depth(1), args),
        "t2" => Exe::Eza.exec_with(t_depth(2), args),
        "t3" => Exe::Eza.exec_with(t_depth(3), args),
        "t4" => Exe::Eza.exec_with(t_depth(4), args),
        "t5" => Exe::Eza.exec_with(t_depth(5), args),
        "t6" => Exe::Eza.exec_with(t_depth(6), args),
        "t7" => Exe::Eza.exec_with(t_depth(7), args),
        "t8" => Exe::Eza.exec_with(t_depth(8), args),
        "t9" => Exe::Eza.exec_with(t_depth(9), args),

        _ => {
            eprintln!("error: {name}: bad alias");
            std::process::exit(2)
        }
    };

    eprintln!("error: {err}");
    std::process::exit(1);
}
