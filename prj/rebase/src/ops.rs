use std::path::Path;

use clap::Subcommand;

use crate::jj::{self, Mode, Remote, Workspace};
use crate::manifest::{Repo, RepoKind, UpstreamRef};
use crate::output::{self, Sink};

/// The operations that the CLI can invoke.
#[derive(Debug, Clone, Copy, Subcommand)]
pub enum Op {
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

/// Result of running an operation on one repo.
#[derive(Debug, PartialEq, Eq)]
pub enum Outcome {
    Ok,
    Skipped(String),
    Failed(String),
}

/// Execution context shared by every operation: how commands run, and where
/// progress is reported.
pub struct Runner<'a> {
    pub mode: Mode,
    pub sink: &'a dyn Sink,
}

impl Runner<'_> {
    /// Run an operation on a single repo, reporting progress to the sink.
    #[must_use]
    pub fn run_one(&self, op: Op, repo: &Repo) -> Outcome {
        if !matches!(op, Op::Clone) && !repo.path.exists() {
            self.sink.warn("not cloned — skipping");
            return Outcome::Skipped("not cloned".into());
        }
        let result = match op {
            Op::Status => self.status(repo),
            Op::Fetch => self.fetch(repo),
            Op::Rebase => self.rebase(repo),
            Op::Build => self.build(repo),
            Op::Push => self.push(repo),
            Op::Update => self.update(repo),
            Op::Clone => self.clone(repo),
        };
        match result {
            Ok(()) => Outcome::Ok,
            Err(e) => Outcome::Failed(e.0),
        }
    }

    fn ws<'r>(&'r self, path: &'r Path) -> Workspace<'r> {
        Workspace::new(path, self.mode, self.sink)
    }

    fn status(&self, repo: &Repo) -> jj::Result<()> {
        let ws = self.ws(&repo.path);
        let st = ws.status()?;
        let stdout = st.stdout.trim();
        if stdout.starts_with("The working copy has no changes") {
            self.sink.ok("clean");
        } else {
            // Show just the first line (e.g. "Working copy changes:").
            self.sink.warn(stdout.lines().next().unwrap_or(stdout));
        }

        if let RepoKind::ForkRebase { fork, bookmarks } = &repo.kind {
            let upstream = &fork.upstream_ref;
            let qualified = upstream.qualified();
            for bm in bookmarks {
                let log = ws.log_bookmark(bm, &qualified)?;
                let ahead = log.stdout.lines().filter(|l| !l.trim().is_empty()).count();
                if ahead == 0 {
                    self.sink
                        .info(&format!("{bm}: up to date with {}", upstream.name));
                } else {
                    self.sink.info(&format!(
                        "{bm}: {ahead} commit(s) ahead of {}",
                        upstream.name
                    ));
                }
            }
        }
        Ok(())
    }

    fn fetch(&self, repo: &Repo) -> jj::Result<()> {
        let ws = self.ws(&repo.path);
        if repo.kind.fork().is_some() {
            ws.fetch(Some(Remote::Upstream))?;
            ws.fetch(Some(Remote::Origin))?;
        } else {
            ws.fetch(None)?;
        }
        Ok(())
    }

    fn rebase(&self, repo: &Repo) -> jj::Result<()> {
        let RepoKind::ForkRebase { fork, bookmarks } = &repo.kind else {
            self.sink.info("not a fork-rebase repo — nothing to rebase");
            return Ok(());
        };
        let ws = self.ws(&repo.path);
        let dest = fork.upstream_ref.qualified();
        for bm in bookmarks {
            ws.rebase(bm, &dest)?;
            ws.ensure_no_conflicts(bm)?;
        }
        Ok(())
    }

    fn build(&self, repo: &Repo) -> jj::Result<()> {
        let ws = self.ws(&repo.path);
        for cmd in repo.build.iter().chain(&repo.post_build) {
            ws.build(cmd)?;
        }
        Ok(())
    }

    fn push(&self, repo: &Repo) -> jj::Result<()> {
        let RepoKind::ForkRebase { fork, bookmarks } = &repo.kind else {
            self.sink.info("not a fork-rebase repo — nothing to push");
            return Ok(());
        };
        let ws = self.ws(&repo.path);
        for bm in bookmarks {
            ws.push(Remote::Origin, bm)?;
        }
        if fork.upstream_ref.is_branch() {
            ws.push(Remote::Origin, &fork.upstream_ref.name)?;
        }
        Ok(())
    }

    /// Set the fork's local trunk bookmark to upstream and push it to origin.
    /// Only meaningful for branches; tags (`@git`) can't be managed as
    /// bookmarks.
    fn sync_trunk(&self, repo: &Repo, upstream_ref: &UpstreamRef) -> jj::Result<()> {
        let ws = self.ws(&repo.path);
        ws.bookmark_set(&upstream_ref.name, &upstream_ref.qualified())?;
        ws.push(Remote::Origin, &upstream_ref.name)?;
        Ok(())
    }

    /// Full update pipeline: fetch → sync trunk → rebase → checkout → build
    /// → push.
    fn update(&self, repo: &Repo) -> jj::Result<()> {
        self.fetch(repo)?;

        if let RepoKind::ForkRebase { fork, .. } = &repo.kind {
            if fork.upstream_ref.is_branch() {
                self.sync_trunk(repo, &fork.upstream_ref)?;
            }
            self.rebase(repo)?;
        }
        if let Some(revision) = repo.checkout_target() {
            self.ws(&repo.path).new_at(&revision)?;
        }
        self.build(repo)?;
        self.push(repo)
    }

    /// Clone the repo if absent, then run the full update.
    fn clone(&self, repo: &Repo) -> jj::Result<()> {
        if repo.path.exists() {
            self.sink.info("already cloned");
            return Ok(());
        }

        // The workspace doesn't exist yet; clone from its parent directory.
        let parent = repo.path.parent().unwrap_or(Path::new("."));
        self.ws(parent).clone(&repo.clone_url, &repo.path)?;

        let ws = self.ws(&repo.path);
        if let Some(fork) = repo.kind.fork() {
            ws.remote_add(Remote::Upstream, &fork.upstream)?;
        }
        if let RepoKind::ForkRebase { bookmarks, .. } = &repo.kind {
            // `jj git clone` leaves non-default bookmarks as `name@origin`
            // only; track them so the bare names resolve locally.
            for bm in bookmarks {
                ws.bookmark_track(bm, Remote::Origin)?;
            }
        }

        self.update(repo)
    }
}

/// Run an operation across all repos sequentially, printing to stderr.
#[must_use]
pub fn run(op: Op, repos: &[Repo], mode: Mode) -> bool {
    let runner = Runner {
        mode,
        sink: &output::StderrSink,
    };

    let mut results: Vec<(String, Outcome)> = Vec::new();
    for repo in repos {
        output::header(&repo.name);
        results.push((repo.name.clone(), runner.run_one(op, repo)));
    }
    output::summary(&results);

    results
        .iter()
        .all(|(_, o)| matches!(o, Outcome::Ok | Outcome::Skipped(_)))
}
