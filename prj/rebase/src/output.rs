use std::fmt::Write as _;

use crate::ops::Outcome;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";

/// The role of a single line of progress output.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// A command about to run.
    Cmd,
    /// A command printed instead of run.
    DryRun,
    Ok,
    Warn,
    Error,
    Info,
    /// A line streamed from a child process.
    Out,
}

/// Destination for one repo's progress lines.
///
/// `Sync` so a single sink can serve the stdout and stderr reader threads of
/// a streaming child process.
pub trait Sink: Sync {
    fn line(&self, kind: Kind, text: String);

    fn ok(&self, text: &str) {
        self.line(Kind::Ok, text.to_string());
    }

    fn info(&self, text: &str) {
        self.line(Kind::Info, text.to_string());
    }

    fn warn(&self, text: &str) {
        self.line(Kind::Warn, text.to_string());
    }

    fn error(&self, text: &str) {
        self.line(Kind::Error, text.to_string());
    }
}

/// Prints lines to stderr, colored, for sequential (non-TUI) runs.
pub struct StderrSink;

impl Sink for StderrSink {
    fn line(&self, kind: Kind, text: String) {
        match kind {
            Kind::Cmd => eprintln!("{DIM}  $ {text}{RESET}"),
            Kind::DryRun => eprintln!("{DIM}  [dry-run] {text}{RESET}"),
            Kind::Ok => eprintln!("{GREEN}  {text}{RESET}"),
            Kind::Warn => eprintln!("{YELLOW}  {text}{RESET}"),
            Kind::Error => eprintln!("{RED}  {text}{RESET}"),
            Kind::Info | Kind::Out => eprintln!("{DIM}  {text}{RESET}"),
        }
    }
}

pub fn header(repo_name: &str) {
    eprintln!("{BOLD}{CYAN}==> {repo_name}{RESET}");
}

/// Print a summary table after all repos have been processed.
pub fn summary(results: &[(String, Outcome)]) {
    eprintln!();
    eprintln!("{BOLD}Summary:{RESET}");
    let max_name = results.iter().map(|(n, _)| n.len()).max().unwrap_or(0);
    let mut buf = String::new();
    for (name, outcome) in results {
        buf.clear();
        let _ = write!(buf, "  {name:<max_name$}  ");
        match outcome {
            Outcome::Ok => {
                let _ = write!(buf, "{GREEN}ok{RESET}");
            }
            Outcome::Skipped(reason) => {
                let _ = write!(buf, "{YELLOW}skipped{RESET} ({reason})");
            }
            Outcome::Failed(reason) => {
                let _ = write!(buf, "{RED}FAILED{RESET} ({reason})");
            }
        }
        eprintln!("{buf}");
    }
}
