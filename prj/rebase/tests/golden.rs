//! Golden tests: run operations in dry-run mode against fixture manifests
//! and assert the exact command sequence each one would execute.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use rebase::jj::Mode;
use rebase::manifest::{self, Repo};
use rebase::ops::{Op, Outcome, Runner};
use rebase::output::{Kind, Sink};

const FORK_REBASE: &str = r#"
clone = "git@example.com:me/app.git"
upstream = "https://example.com/up/app"
bookmarks = ["custom"]
checkout = "custom"
upstream_ref = "main"
build = ["cargo install --path ."]
"#;

const FORK_REBASE_TAG: &str = r#"
clone = "git@example.com:me/app.git"
upstream = "https://example.com/up/app"
bookmarks = ["custom"]
checkout = "custom"
upstream_ref = "v1.2.3@git"
build = ["cargo install --path ."]
"#;

const FORK_TRACK: &str = r#"
clone = "git@example.com:me/app.git"
upstream = "https://example.com/up/app"
upstream_ref = "main"
build = ["cargo install --path ."]
"#;

const OWN: &str = r#"
clone = "git@example.com:me/app.git"
build = ["cargo install --path ."]
"#;

const CONFLICT_CHECK: &str = "jj log -r custom --no-graph -T if(conflict, \"CONFLICT\\n\")";

/// A scratch directory serving as the manifest root, removed on drop.
struct TempRoot(PathBuf);

impl TempRoot {
    fn new() -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!("rebase-golden-{}-{n}", std::process::id()));
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Write a manifest containing a single repo `app` and load it back.
fn load_repo(root: &TempRoot, entry: &str) -> Repo {
    let manifest_path = root.path().join("rebase.toml");
    let text = format!(
        "root = {:?}\n\n[repos.app]{entry}",
        root.path().to_str().unwrap()
    );
    fs::write(&manifest_path, text).unwrap();
    let mut repos = manifest::load(&manifest_path).unwrap();
    assert_eq!(repos.len(), 1);
    repos.remove(0)
}

/// Collects progress lines for assertions.
#[derive(Default)]
struct TestSink(Mutex<Vec<(Kind, String)>>);

impl Sink for TestSink {
    fn line(&self, kind: Kind, text: String) {
        self.0.lock().unwrap().push((kind, text));
    }
}

impl TestSink {
    fn lines_of(&self, kind: Kind) -> Vec<String> {
        self.0
            .lock()
            .unwrap()
            .iter()
            .filter(|(k, _)| *k == kind)
            .map(|(_, text)| text.clone())
            .collect()
    }
}

/// Run `op` against `repo` in dry-run mode.
fn dry_run(op: Op, repo: &Repo) -> (TestSink, Outcome) {
    let sink = TestSink::default();
    let outcome = Runner {
        mode: Mode::DryRun,
        sink: &sink,
    }
    .run_one(op, repo);
    (sink, outcome)
}

#[test]
fn update_fork_rebase_runs_full_pipeline() {
    let root = TempRoot::new();
    let repo = load_repo(&root, FORK_REBASE);
    fs::create_dir(&repo.path).unwrap();

    let (sink, outcome) = dry_run(Op::Update, &repo);

    assert_eq!(outcome, Outcome::Ok);
    assert_eq!(
        sink.lines_of(Kind::DryRun),
        [
            "jj git fetch --remote upstream",
            "jj git fetch --remote origin",
            "jj bookmark set main -r main@upstream",
            "jj git push --remote origin --bookmark main",
            "jj rebase -b custom -d main@upstream --skip-emptied",
            CONFLICT_CHECK,
            "jj new custom",
            "sh -c cargo install --path .",
            "jj git push --remote origin --bookmark custom",
            "jj git push --remote origin --bookmark main",
        ]
    );
}

#[test]
fn update_tag_upstream_skips_trunk_sync_and_push() {
    let root = TempRoot::new();
    let repo = load_repo(&root, FORK_REBASE_TAG);
    fs::create_dir(&repo.path).unwrap();

    let (sink, outcome) = dry_run(Op::Update, &repo);

    assert_eq!(outcome, Outcome::Ok);
    assert_eq!(
        sink.lines_of(Kind::DryRun),
        [
            "jj git fetch --remote upstream",
            "jj git fetch --remote origin",
            "jj rebase -b custom -d v1.2.3@git --skip-emptied",
            CONFLICT_CHECK,
            "jj new custom",
            "sh -c cargo install --path .",
            "jj git push --remote origin --bookmark custom",
        ]
    );
}

#[test]
fn update_fork_track_checks_out_upstream_ref() {
    let root = TempRoot::new();
    let repo = load_repo(&root, FORK_TRACK);
    fs::create_dir(&repo.path).unwrap();

    let (sink, outcome) = dry_run(Op::Update, &repo);

    assert_eq!(outcome, Outcome::Ok);
    assert_eq!(
        sink.lines_of(Kind::DryRun),
        [
            "jj git fetch --remote upstream",
            "jj git fetch --remote origin",
            "jj new main@upstream",
            "sh -c cargo install --path .",
        ]
    );
    assert_eq!(
        sink.lines_of(Kind::Info),
        ["not a fork-rebase repo — nothing to push"]
    );
}

#[test]
fn update_own_checks_out_trunk() {
    let root = TempRoot::new();
    let repo = load_repo(&root, OWN);
    fs::create_dir(&repo.path).unwrap();

    let (sink, outcome) = dry_run(Op::Update, &repo);

    assert_eq!(outcome, Outcome::Ok);
    assert_eq!(
        sink.lines_of(Kind::DryRun),
        [
            "jj git fetch",
            "jj new trunk()",
            "sh -c cargo install --path .",
        ]
    );
}

#[test]
fn rebase_own_repo_is_a_noop() {
    let root = TempRoot::new();
    let repo = load_repo(&root, OWN);
    fs::create_dir(&repo.path).unwrap();

    let (sink, outcome) = dry_run(Op::Rebase, &repo);

    assert_eq!(outcome, Outcome::Ok);
    assert!(sink.lines_of(Kind::DryRun).is_empty());
    assert_eq!(
        sink.lines_of(Kind::Info),
        ["not a fork-rebase repo — nothing to rebase"]
    );
}

#[test]
fn missing_repo_is_skipped() {
    let root = TempRoot::new();
    let repo = load_repo(&root, FORK_REBASE);

    let (sink, outcome) = dry_run(Op::Update, &repo);

    assert_eq!(outcome, Outcome::Skipped("not cloned".into()));
    assert!(sink.lines_of(Kind::DryRun).is_empty());
    assert_eq!(sink.lines_of(Kind::Warn), ["not cloned — skipping"]);
}

#[test]
fn clone_missing_repo_previews_full_pipeline() {
    let root = TempRoot::new();
    let repo = load_repo(&root, FORK_REBASE);

    let (sink, outcome) = dry_run(Op::Clone, &repo);

    assert_eq!(outcome, Outcome::Ok);
    let clone_cmd = format!(
        "jj git clone --colocate git@example.com:me/app.git {}",
        repo.path.display()
    );
    assert_eq!(
        sink.lines_of(Kind::DryRun),
        [
            clone_cmd.as_str(),
            "jj git remote add upstream https://example.com/up/app",
            "jj bookmark track custom@origin",
            "jj git fetch --remote upstream",
            "jj git fetch --remote origin",
            "jj bookmark set main -r main@upstream",
            "jj git push --remote origin --bookmark main",
            "jj rebase -b custom -d main@upstream --skip-emptied",
            CONFLICT_CHECK,
            "jj new custom",
            "sh -c cargo install --path .",
            "jj git push --remote origin --bookmark custom",
            "jj git push --remote origin --bookmark main",
        ]
    );
}

#[test]
fn clone_existing_repo_is_a_noop() {
    let root = TempRoot::new();
    let repo = load_repo(&root, FORK_REBASE);
    fs::create_dir(&repo.path).unwrap();

    let (sink, outcome) = dry_run(Op::Clone, &repo);

    assert_eq!(outcome, Outcome::Ok);
    assert!(sink.lines_of(Kind::DryRun).is_empty());
    assert_eq!(sink.lines_of(Kind::Info), ["already cloned"]);
}

#[test]
fn manifest_rejects_bookmarks_without_upstream() {
    let root = TempRoot::new();
    let manifest_path = root.path().join("rebase.toml");
    fs::write(
        &manifest_path,
        "root = \"/tmp\"\n\n[repos.app]\nclone = \"x\"\nbookmarks = [\"custom\"]\nbuild = []\n",
    )
    .unwrap();

    let err = manifest::load(&manifest_path).unwrap_err();
    assert_eq!(
        err.to_string(),
        "invalid manifest: app: `bookmarks` requires `upstream`"
    );
}
