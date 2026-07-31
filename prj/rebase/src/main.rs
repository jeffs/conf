use std::io::IsTerminal;
use std::process;

use clap::Parser;
use rebase::{Cli, Cmd, manifest, ops, tui};

fn main() {
    let cli = Cli::parse();

    let manifest_path = manifest::expand_tilde(&cli.manifest);
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

    let use_tui = !cli.plain && !cli.dry_run && std::io::stdout().is_terminal();
    let all_ok = if use_tui {
        match tui::run(op, &repos, cli.jobs) {
            Ok(ok) => ok,
            Err(e) => {
                eprintln!("error: {e}");
                false
            }
        }
    } else {
        ops::run(op, &repos, cli.dry_run)
    };
    if !all_ok {
        process::exit(1);
    }
}
