use std::fmt::Write as _;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";

pub fn header(repo_name: &str) {
    eprintln!("{BOLD}{CYAN}==> {repo_name}{RESET}");
}

pub fn cmd(display: &str) {
    eprintln!("{DIM}  $ {display}{RESET}");
}

pub fn dry_run(display: &str) {
    eprintln!("{DIM}  [dry-run] {display}{RESET}");
}

pub fn ok(msg: &str) {
    eprintln!("{GREEN}  {msg}{RESET}");
}

pub fn warn(msg: &str) {
    eprintln!("{YELLOW}  {msg}{RESET}");
}

pub fn error(msg: &str) {
    eprintln!("{RED}  {msg}{RESET}");
}

pub fn info(msg: &str) {
    eprintln!("{DIM}  {msg}{RESET}");
}

/// Outcome for a single repo.
pub enum Outcome {
    Ok,
    Skipped(String),
    Failed(String),
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
