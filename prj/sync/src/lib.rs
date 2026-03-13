pub mod jj;
pub mod manifest;
pub mod ops;
pub mod output;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "sync", about = "Source-installed package manager")]
pub struct Cli {
    /// Manifest path
    #[arg(short, long, default_value = "~/conf/etc/sync.toml")]
    pub manifest: String,

    /// Operate on specific repo(s) (repeatable)
    #[arg(short, long = "repo")]
    pub repo: Vec<String>,

    /// Print commands without executing
    #[arg(short = 'n', long)]
    pub dry_run: bool,

    #[command(subcommand)]
    pub command: Option<Cmd>,
}

#[derive(Subcommand)]
pub enum Cmd {
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
