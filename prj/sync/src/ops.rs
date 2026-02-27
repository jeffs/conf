use std::path::Path;

use crate::jj;
use crate::manifest::{self, Repo, RepoKind};
use crate::output::{self, Outcome};

/// Run the status check for a single repo.
fn status_one(repo: &Repo) -> Outcome {
    if !repo.path.exists() {
        output::warn("not cloned");
        return Outcome::Skipped("not cloned".into());
    }

    let st = jj::status(&repo.path);
    if !st.success {
        return Outcome::Failed("jj status failed".into());
    }

    let stdout = st.stdout.trim();
    if stdout.starts_with("The working copy has no changes") {
        output::ok("clean");
    } else {
        // Show just the first line (e.g. "Working copy changes:" or similar).
        let first_line = stdout.lines().next().unwrap_or(stdout);
        output::warn(first_line);
    }

    match &repo.kind {
        RepoKind::ForkRebase {
            upstream_ref,
            bookmarks,
            ..
        } => {
            let qualified = upstream_ref.qualified();
            for bm in bookmarks {
                let log = jj::log_bookmark(&repo.path, bm, &qualified);
                let commits: Vec<&str> = log
                    .stdout
                    .lines()
                    .filter(|l| !l.trim().is_empty())
                    .collect();
                if commits.is_empty() {
                    output::info(&format!("{bm}: up to date with {}", upstream_ref.name));
                } else {
                    output::info(&format!(
                        "{bm}: {} commit(s) ahead of {}",
                        commits.len(),
                        upstream_ref.name,
                    ));
                }
            }
        }
        RepoKind::ForkTrack { .. } | RepoKind::Own => {}
    }

    Outcome::Ok
}

/// Fetch remotes for a single repo.
fn fetch_one(repo: &Repo, dry_run: bool) -> Outcome {
    if !repo.path.exists() {
        output::warn("not cloned — skipping");
        return Outcome::Skipped("not cloned".into());
    }

    match &repo.kind {
        RepoKind::ForkRebase { .. } | RepoKind::ForkTrack { .. } => {
            let r1 = jj::fetch(&repo.path, Some("upstream"), dry_run);
            if !r1.success {
                return Outcome::Failed("fetch upstream failed".into());
            }
            let r2 = jj::fetch(&repo.path, Some("origin"), dry_run);
            if !r2.success {
                return Outcome::Failed("fetch origin failed".into());
            }
        }
        RepoKind::Own => {
            let r = jj::fetch(&repo.path, None, dry_run);
            if !r.success {
                return Outcome::Failed("fetch failed".into());
            }
        }
    }

    Outcome::Ok
}

/// Sync the trunk bookmark for a fork: set local trunk to upstream, push to origin.
/// Only meaningful for branches; tags (`@git`) can't be managed as bookmarks.
fn sync_trunk(repo: &Repo, upstream_ref: &manifest::UpstreamRef, dry_run: bool) -> bool {
    let target = upstream_ref.qualified();
    let r = jj::bookmark_set(&repo.path, &upstream_ref.name, &target, dry_run);
    if !r.success {
        return false;
    }
    let r = jj::push(&repo.path, "origin", &upstream_ref.name, dry_run);
    r.success
}

/// Rebase fork bookmarks onto upstream.
fn rebase_one(repo: &Repo, dry_run: bool) -> Outcome {
    if !repo.path.exists() {
        output::warn("not cloned — skipping");
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
            let r = jj::rebase(&repo.path, bm, &qualified, dry_run);
            if !r.success {
                return Outcome::Failed(format!("rebase {bm} failed"));
            }

            let conflict_check = jj::has_conflicts(&repo.path, bm, dry_run);
            if !conflict_check.success {
                return Outcome::Failed(format!("{bm} has conflicts"));
            }
        }
        Outcome::Ok
    } else {
        output::info("not a fork-rebase repo — nothing to rebase");
        Outcome::Ok
    }
}

/// Run build commands for a single repo.
fn build_one(repo: &Repo, dry_run: bool) -> Outcome {
    if !repo.path.exists() {
        output::warn("not cloned — skipping");
        return Outcome::Skipped("not cloned".into());
    }

    for cmd_str in &repo.build {
        let r = jj::build(&repo.path, cmd_str, dry_run);
        if !r.success {
            return Outcome::Failed(format!("build command failed: {cmd_str}"));
        }
    }
    for cmd_str in &repo.post_build {
        let r = jj::build(&repo.path, cmd_str, dry_run);
        if !r.success {
            return Outcome::Failed(format!("post_build command failed: {cmd_str}"));
        }
    }

    Outcome::Ok
}

/// Push fork bookmarks to origin.
fn push_one(repo: &Repo, dry_run: bool) -> Outcome {
    if !repo.path.exists() {
        output::warn("not cloned — skipping");
        return Outcome::Skipped("not cloned".into());
    }

    if let RepoKind::ForkRebase {
        upstream_ref,
        bookmarks,
        ..
    } = &repo.kind
    {
        for bm in bookmarks {
            let r = jj::push(&repo.path, "origin", bm, dry_run);
            if !r.success {
                return Outcome::Failed(format!("push {bm} failed"));
            }
        }
        if upstream_ref.is_branch() {
            let r = jj::push(&repo.path, "origin", &upstream_ref.name, dry_run);
            if !r.success {
                return Outcome::Failed(format!("push {} failed", upstream_ref.name));
            }
        }
        Outcome::Ok
    } else {
        output::info("not a fork-rebase repo — nothing to push");
        Outcome::Ok
    }
}

/// Full update pipeline: fetch → sync trunk → rebase → build → push.
fn update_one(repo: &Repo, dry_run: bool) -> Outcome {
    if !repo.path.exists() {
        output::warn("not cloned — skipping");
        return Outcome::Skipped("not cloned".into());
    }

    // Fetch
    let outcome = fetch_one(repo, dry_run);
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
            if upstream_ref.is_branch() {
                if !sync_trunk(repo, upstream_ref, dry_run) {
                    return Outcome::Failed("sync trunk failed".into());
                }
            }
            let outcome = rebase_one(repo, dry_run);
            if !matches!(outcome, Outcome::Ok) {
                return outcome;
            }
            if bookmarks.contains(&"custom".to_string()) {
                let r = jj::new_at(&repo.path, "custom", dry_run);
                if !r.success {
                    return Outcome::Failed("jj new failed".into());
                }
            }
        }
        RepoKind::ForkTrack { upstream_ref, .. } => {
            let r = jj::new_at(&repo.path, &upstream_ref.qualified(), dry_run);
            if !r.success {
                return Outcome::Failed("jj new failed".into());
            }
        }
        RepoKind::Own => {
            let r = jj::new_at(&repo.path, "trunk()", dry_run);
            if !r.success {
                return Outcome::Failed("jj new failed".into());
            }
        }
    }

    // Build
    let outcome = build_one(repo, dry_run);
    if !matches!(outcome, Outcome::Ok) {
        return outcome;
    }

    // Push (fork-rebase only)
    push_one(repo, dry_run)
}

/// Clone repos that don't exist locally, then run update.
fn clone_one(repo: &Repo, dry_run: bool) -> Outcome {
    if repo.path.exists() {
        output::info("already cloned");
        return Outcome::Ok;
    }

    // Clone
    let parent = repo.path.parent().unwrap_or(Path::new("."));
    let r = jj::clone(parent, &repo.clone_url, &repo.path, dry_run);
    if !r.success {
        return Outcome::Failed("clone failed".into());
    }

    // Add upstream remote for forks
    match &repo.kind {
        RepoKind::ForkRebase { upstream, .. } | RepoKind::ForkTrack { upstream, .. } => {
            let r = jj::remote_add(&repo.path, "upstream", upstream, dry_run);
            if !r.success {
                return Outcome::Failed("adding upstream remote failed".into());
            }
        }
        RepoKind::Own => {}
    }

    // Run full update
    update_one(repo, dry_run)
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

/// Run an operation across all repos, collecting results.
pub fn run(op: Op, repos: &[Repo], dry_run: bool) -> bool {
    let mut results: Vec<(String, Outcome)> = Vec::new();

    for repo in repos {
        output::header(&repo.name);
        let outcome = match op {
            Op::Status => status_one(repo),
            Op::Fetch => fetch_one(repo, dry_run),
            Op::Rebase => rebase_one(repo, dry_run),
            Op::Build => build_one(repo, dry_run),
            Op::Push => push_one(repo, dry_run),
            Op::Update => update_one(repo, dry_run),
            Op::Clone => clone_one(repo, dry_run),
        };
        results.push((repo.name.clone(), outcome));
    }

    output::summary(&results);

    results
        .iter()
        .all(|(_, o)| matches!(o, Outcome::Ok | Outcome::Skipped(_)))
}
