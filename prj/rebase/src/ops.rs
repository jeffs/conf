use std::path::Path;

use crate::jj;
use crate::manifest::{self, Repo, RepoKind};
use crate::output::{self, Kind, Outcome, Sink};

/// Run the status check for a single repo.
fn status_one(repo: &Repo, sink: &dyn Sink) -> Outcome {
    if !repo.path.exists() {
        sink.line(Kind::Warn, "not cloned".into());
        return Outcome::Skipped("not cloned".into());
    }

    let st = jj::status(&repo.path, sink);
    if !st.success {
        return Outcome::Failed("jj status failed".into());
    }

    let stdout = st.stdout.trim();
    if stdout.starts_with("The working copy has no changes") {
        sink.line(Kind::Ok, "clean".into());
    } else {
        // Show just the first line (e.g. "Working copy changes:" or similar).
        let first_line = stdout.lines().next().unwrap_or(stdout);
        sink.line(Kind::Warn, first_line.into());
    }

    match &repo.kind {
        RepoKind::ForkRebase {
            upstream_ref,
            bookmarks,
            ..
        } => {
            let qualified = upstream_ref.qualified();
            for bm in bookmarks {
                let log = jj::log_bookmark(&repo.path, bm, &qualified, sink);
                let commits: Vec<&str> = log
                    .stdout
                    .lines()
                    .filter(|l| !l.trim().is_empty())
                    .collect();
                if commits.is_empty() {
                    sink.line(
                        Kind::Info,
                        format!("{bm}: up to date with {}", upstream_ref.name),
                    );
                } else {
                    sink.line(
                        Kind::Info,
                        format!(
                            "{bm}: {} commit(s) ahead of {}",
                            commits.len(),
                            upstream_ref.name,
                        ),
                    );
                }
            }
        }
        RepoKind::ForkTrack { .. } | RepoKind::Own => {}
    }

    Outcome::Ok
}

/// Fetch remotes for a single repo.
fn fetch_one(repo: &Repo, dry_run: bool, sink: &dyn Sink) -> Outcome {
    if !repo.path.exists() {
        sink.line(Kind::Warn, "not cloned — skipping".into());
        return Outcome::Skipped("not cloned".into());
    }

    match &repo.kind {
        RepoKind::ForkRebase { .. } | RepoKind::ForkTrack { .. } => {
            let r1 = jj::fetch(&repo.path, Some("upstream"), dry_run, sink);
            if !r1.success {
                return Outcome::Failed("fetch upstream failed".into());
            }
            let r2 = jj::fetch(&repo.path, Some("origin"), dry_run, sink);
            if !r2.success {
                return Outcome::Failed("fetch origin failed".into());
            }
        }
        RepoKind::Own => {
            let r = jj::fetch(&repo.path, None, dry_run, sink);
            if !r.success {
                return Outcome::Failed("fetch failed".into());
            }
        }
    }

    Outcome::Ok
}

/// Sync the trunk bookmark for a fork: set local trunk to upstream, push to origin.
/// Only meaningful for branches; tags (`@git`) can't be managed as bookmarks.
fn sync_trunk(
    repo: &Repo,
    upstream_ref: &manifest::UpstreamRef,
    dry_run: bool,
    sink: &dyn Sink,
) -> bool {
    let target = upstream_ref.qualified();
    let r = jj::bookmark_set(&repo.path, &upstream_ref.name, &target, dry_run, sink);
    if !r.success {
        return false;
    }
    let r = jj::push(&repo.path, "origin", &upstream_ref.name, dry_run, sink);
    r.success
}

/// Rebase fork bookmarks onto upstream.
fn rebase_one(repo: &Repo, dry_run: bool, sink: &dyn Sink) -> Outcome {
    if !repo.path.exists() {
        sink.line(Kind::Warn, "not cloned — skipping".into());
        return Outcome::Skipped("not cloned".into());
    }

    if let RepoKind::ForkRebase {
        upstream_ref,
        bookmarks,
        ..
    } = &repo.kind
    {
        let qualified = upstream_ref.qualified();
        for bm in bookmarks {
            let r = jj::rebase(&repo.path, bm, &qualified, dry_run, sink);
            if !r.success {
                return Outcome::Failed(format!("rebase {bm} failed"));
            }

            let conflict_check = jj::has_conflicts(&repo.path, bm, dry_run, sink);
            if !conflict_check.success {
                return Outcome::Failed(format!("{bm} has conflicts"));
            }
        }
        Outcome::Ok
    } else {
        sink.line(
            Kind::Info,
            "not a fork-rebase repo — nothing to rebase".into(),
        );
        Outcome::Ok
    }
}

/// Run build commands for a single repo.
fn build_one(repo: &Repo, dry_run: bool, sink: &dyn Sink) -> Outcome {
    if !repo.path.exists() {
        sink.line(Kind::Warn, "not cloned — skipping".into());
        return Outcome::Skipped("not cloned".into());
    }

    for cmd_str in &repo.build {
        let r = jj::build(&repo.path, cmd_str, dry_run, sink);
        if !r.success {
            return Outcome::Failed(format!("build command failed: {cmd_str}"));
        }
    }
    for cmd_str in &repo.post_build {
        let r = jj::build(&repo.path, cmd_str, dry_run, sink);
        if !r.success {
            return Outcome::Failed(format!("post_build command failed: {cmd_str}"));
        }
    }

    Outcome::Ok
}

/// Push fork bookmarks to origin.
fn push_one(repo: &Repo, dry_run: bool, sink: &dyn Sink) -> Outcome {
    if !repo.path.exists() {
        sink.line(Kind::Warn, "not cloned — skipping".into());
        return Outcome::Skipped("not cloned".into());
    }

    if let RepoKind::ForkRebase {
        upstream_ref,
        bookmarks,
        ..
    } = &repo.kind
    {
        for bm in bookmarks {
            let r = jj::push(&repo.path, "origin", bm, dry_run, sink);
            if !r.success {
                return Outcome::Failed(format!("push {bm} failed"));
            }
        }
        if upstream_ref.is_branch() {
            let r = jj::push(&repo.path, "origin", &upstream_ref.name, dry_run, sink);
            if !r.success {
                return Outcome::Failed(format!("push {} failed", upstream_ref.name));
            }
        }
        Outcome::Ok
    } else {
        sink.line(
            Kind::Info,
            "not a fork-rebase repo — nothing to push".into(),
        );
        Outcome::Ok
    }
}

/// Full update pipeline: fetch → sync trunk → rebase → build → push.
fn update_one(repo: &Repo, dry_run: bool, sink: &dyn Sink) -> Outcome {
    if !repo.path.exists() {
        sink.line(Kind::Warn, "not cloned — skipping".into());
        return Outcome::Skipped("not cloned".into());
    }

    // Fetch
    let outcome = fetch_one(repo, dry_run, sink);
    if !matches!(outcome, Outcome::Ok) {
        return outcome;
    }

    // Sync trunk + rebase (fork-rebase), or advance working copy (others)
    match &repo.kind {
        RepoKind::ForkRebase {
            upstream_ref,
            bookmarks,
            ..
        } => {
            if upstream_ref.is_branch() && !sync_trunk(repo, upstream_ref, dry_run, sink) {
                return Outcome::Failed("sync trunk failed".into());
            }
            let outcome = rebase_one(repo, dry_run, sink);
            if !matches!(outcome, Outcome::Ok) {
                return outcome;
            }
            if bookmarks.contains(&"custom".to_string()) {
                let r = jj::new_at(&repo.path, "custom", dry_run, sink);
                if !r.success {
                    return Outcome::Failed("jj new failed".into());
                }
            }
        }
        RepoKind::ForkTrack { upstream_ref, .. } => {
            let r = jj::new_at(&repo.path, &upstream_ref.qualified(), dry_run, sink);
            if !r.success {
                return Outcome::Failed("jj new failed".into());
            }
        }
        RepoKind::Own => {
            let r = jj::new_at(&repo.path, "trunk()", dry_run, sink);
            if !r.success {
                return Outcome::Failed("jj new failed".into());
            }
        }
    }

    // Build
    let outcome = build_one(repo, dry_run, sink);
    if !matches!(outcome, Outcome::Ok) {
        return outcome;
    }

    // Push (fork-rebase only)
    push_one(repo, dry_run, sink)
}

/// Clone repos that don't exist locally, then run update.
fn clone_one(repo: &Repo, dry_run: bool, sink: &dyn Sink) -> Outcome {
    if repo.path.exists() {
        sink.line(Kind::Info, "already cloned".into());
        return Outcome::Ok;
    }

    // Clone
    let parent = repo.path.parent().unwrap_or(Path::new("."));
    let r = jj::clone(parent, &repo.clone_url, &repo.path, dry_run, sink);
    if !r.success {
        return Outcome::Failed("clone failed".into());
    }

    // Add upstream remote for forks
    match &repo.kind {
        RepoKind::ForkRebase { upstream, .. } | RepoKind::ForkTrack { upstream, .. } => {
            let r = jj::remote_add(&repo.path, "upstream", upstream, dry_run, sink);
            if !r.success {
                return Outcome::Failed("adding upstream remote failed".into());
            }
        }
        RepoKind::Own => {}
    }

    // `jj git clone` only creates a local bookmark for the fork's default
    // branch, so custom bookmarks (e.g. `custom`) start out as `name@origin`
    // only.  Track them locally so rebase/checkout/push can resolve the bare
    // name; a bookmark absent on origin is silently skipped.
    if let RepoKind::ForkRebase { bookmarks, .. } = &repo.kind {
        for bm in bookmarks {
            let r = jj::bookmark_track(&repo.path, bm, "origin", dry_run, sink);
            if !r.success {
                return Outcome::Failed(format!("tracking {bm}@origin failed"));
            }
        }
    }

    // Run full update
    update_one(repo, dry_run, sink)
}

/// The operations that the CLI can invoke.
#[derive(Debug, Clone, Copy)]
pub enum Op {
    Status,
    Fetch,
    Rebase,
    Build,
    Push,
    Update,
    Clone,
}

/// Run an operation on a single repo, reporting progress to `sink`.
#[must_use]
pub fn run_one(op: Op, repo: &Repo, dry_run: bool, sink: &dyn Sink) -> Outcome {
    match op {
        Op::Status => status_one(repo, sink),
        Op::Fetch => fetch_one(repo, dry_run, sink),
        Op::Rebase => rebase_one(repo, dry_run, sink),
        Op::Build => build_one(repo, dry_run, sink),
        Op::Push => push_one(repo, dry_run, sink),
        Op::Update => update_one(repo, dry_run, sink),
        Op::Clone => clone_one(repo, dry_run, sink),
    }
}

/// Run an operation across all repos sequentially, printing to stderr.
#[must_use]
pub fn run(op: Op, repos: &[Repo], dry_run: bool) -> bool {
    let mut results: Vec<(String, Outcome)> = Vec::new();

    for repo in repos {
        output::header(&repo.name);
        let outcome = run_one(op, repo, dry_run, &output::StderrSink);
        results.push((repo.name.clone(), outcome));
    }

    output::summary(&results);

    results
        .iter()
        .all(|(_, o)| matches!(o, Outcome::Ok | Outcome::Skipped(_)))
}
