pub mod jj;
pub mod manifest;
pub mod ops;
pub mod output;
pub mod tui;

use clap::Parser;

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
    pub command: Option<Op>,
}
