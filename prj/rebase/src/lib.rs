pub mod github;
pub mod jj;
pub mod manifest;
pub mod ops;
pub mod output;
pub mod tui;

use clap::{Parser, Subcommand};

use crate::github::Slug;
use crate::ops::Op;

#[derive(Parser)]
#[command(name = "rebase", about = "Source-installed package manager")]
pub struct Cli {
    /// Manifest path
    #[arg(short, long, default_value = "~/conf/etc/rebase.toml")]
    pub manifest: String,

    /// Operate on specific repo(s) (repeatable)
    #[arg(short, long = "repo")]
    pub repo: Vec<String>,

    /// Print commands without executing
    #[arg(short = 'n', long)]
    pub dry_run: bool,

    /// Max repos to process concurrently (TUI mode)
    #[arg(short, long, default_value_t = 4)]
    pub jobs: usize,

    /// Sequential text output instead of the TUI
    #[arg(long)]
    pub plain: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
}

/// The subcommands that the CLI accepts.
#[derive(Subcommand)]
pub enum Command {
    /// Create the missing GitHub fork of a manifest repo and its fork
    /// bookmarks, then clone it
    Fork {
        /// GitHub URL of the upstream repo, as in the manifest's `upstream`
        #[arg(value_parser = parse_github_url)]
        upstream: Slug,
    },
    #[command(flatten)]
    Op(Op),
}

fn parse_github_url(url: &str) -> Result<Slug, String> {
    Slug::from_url(url).ok_or_else(|| format!("not a GitHub repo URL: {url}"))
}
