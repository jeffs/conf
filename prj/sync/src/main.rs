mod jj;
mod manifest;
mod ops;
mod output;

use std::path::PathBuf;
use std::process;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "sync", about = "Source-installed package manager")]
struct Cli {
    /// Manifest path
    #[arg(short, long, default_value = "~/conf/etc/sync.toml")]
    manifest: String,

    /// Operate on specific repo(s) (repeatable)
    #[arg(short, long = "repo")]
    repo: Vec<String>,

    /// Print commands without executing
    #[arg(short = 'n', long)]
    dry_run: bool,

    #[command(subcommand)]
    command: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    /// Show state of all repos
    Status,
    /// Fetch from remotes
    Fetch,
    /// Rebase fork bookmarks onto upstream
    Rebase,
    /// Build and install
    Build,
    /// Push fork bookmarks to origin
    Push,
    /// fetch → rebase → build → push (default)
    Update,
    /// Clone repos that don't exist locally
    Clone,
}

fn expand_tilde(s: &str) -> PathBuf {
    if let Some(rest) = s.strip_prefix("~/") {
        #[allow(deprecated)]
        let home = std::env::home_dir().expect("HOME not set");
        home.join(rest)
    } else {
        PathBuf::from(s)
    }
}

fn main() {
    let cli = Cli::parse();

    let manifest_path = expand_tilde(&cli.manifest);
    let repos = match manifest::load(&manifest_path) {
        Ok(repos) => repos,
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(1);
        }
    };

    // Filter to requested repos if --repo given.
    let repos: Vec<_> = if cli.repo.is_empty() {
        repos
    } else {
        let filtered: Vec<_> = repos
            .into_iter()
            .filter(|r| cli.repo.contains(&r.name))
            .collect();
        let found: Vec<&str> = filtered.iter().map(|r| r.name.as_str()).collect();
        for name in &cli.repo {
            if !found.contains(&name.as_str()) {
                eprintln!("warning: repo '{name}' not found in manifest");
            }
        }
        filtered
    };

    let op = match cli.command {
        None | Some(Cmd::Update) => ops::Op::Update,
        Some(Cmd::Status) => ops::Op::Status,
        Some(Cmd::Fetch) => ops::Op::Fetch,
        Some(Cmd::Rebase) => ops::Op::Rebase,
        Some(Cmd::Build) => ops::Op::Build,
        Some(Cmd::Push) => ops::Op::Push,
        Some(Cmd::Clone) => ops::Op::Clone,
    };

    let all_ok = ops::run(op, &repos, cli.dry_run);
    if !all_ok {
        process::exit(1);
    }
}
