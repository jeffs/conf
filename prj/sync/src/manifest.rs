use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::{fmt, fs};

/// Raw TOML representation of the manifest.
#[derive(Deserialize)]
pub struct Manifest {
    pub root: String,
    #[serde(default)]
    pub repos: BTreeMap<String, RawRepo>,
}

/// A single repo entry as written in the TOML file.
#[derive(Deserialize)]
pub struct RawRepo {
    pub clone: String,
    pub upstream: Option<String>,
    pub bookmarks: Option<Vec<String>>,
    pub upstream_ref: Option<String>,
    pub build: Vec<String>,
    #[serde(default)]
    pub post_build: Vec<String>,
    pub path: Option<String>,
}

/// A parsed `upstream_ref` value: ref name + remote qualifier.
///
/// In jj, remote branches are `name@remote` (e.g. `main@upstream`) while tags
/// imported from git are `name@git`.  The TOML value can be:
///   - `"main"` → name="main", remote="upstream" (default)
///   - `"main@upstream"` → explicit
///   - `"v0.43.1@git"` → tag
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpstreamRef {
    pub name: String,
    pub remote: String,
}

impl UpstreamRef {
    /// Parse an `upstream_ref` string, splitting on the last `@`.
    /// If no `@` is present, defaults the remote to `"upstream"`.
    fn parse(s: &str) -> Result<Self, String> {
        if let Some(pos) = s.rfind('@') {
            let name = &s[..pos];
            let remote = &s[pos + 1..];
            if name.is_empty() {
                return Err("upstream_ref name must not be empty".into());
            }
            if remote.is_empty() {
                return Err("upstream_ref remote must not be empty".into());
            }
            Ok(Self {
                name: name.into(),
                remote: remote.into(),
            })
        } else {
            Ok(Self {
                name: s.into(),
                remote: "upstream".into(),
            })
        }
    }

    /// The fully qualified ref for use in jj revsets (e.g. `main@upstream`).
    pub fn qualified(&self) -> String {
        format!("{}@{}", self.name, self.remote)
    }

    /// Whether this ref is a remote branch (can be synced/pushed as a bookmark).
    /// Tags (`@git`) cannot.
    pub fn is_branch(&self) -> bool {
        self.remote != "git"
    }
}

/// Inferred behavior for a repo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepoKind {
    /// Fork with upstream + bookmarks to rebase and push.
    ForkRebase {
        upstream: String,
        upstream_ref: UpstreamRef,
        bookmarks: Vec<String>,
    },
    /// Fork tracking upstream, no custom bookmarks.
    ForkTrack {
        upstream: String,
        upstream_ref: UpstreamRef,
    },
    /// Own repo or upstream-only (no fork relationship).
    Own,
}

/// A fully resolved repo ready for operations.
#[derive(Debug, Clone)]
pub struct Repo {
    pub name: String,
    pub clone_url: String,
    pub path: PathBuf,
    pub kind: RepoKind,
    pub build: Vec<String>,
    pub post_build: Vec<String>,
}

#[derive(Debug)]
pub enum ManifestError {
    Io(std::io::Error),
    Parse(toml::de::Error),
    Validation(String),
}

impl fmt::Display for ManifestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "reading manifest: {e}"),
            Self::Parse(e) => write!(f, "parsing manifest: {e}"),
            Self::Validation(msg) => write!(f, "invalid manifest: {msg}"),
        }
    }
}

impl std::error::Error for ManifestError {}

impl From<std::io::Error> for ManifestError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<toml::de::Error> for ManifestError {
    fn from(e: toml::de::Error) -> Self {
        Self::Parse(e)
    }
}

fn expand_tilde(s: &str) -> PathBuf {
    if let Some(rest) = s.strip_prefix("~/") {
        #[allow(deprecated)]
        let home = std::env::home_dir().expect("HOME not set");
        home.join(rest)
    } else {
        PathBuf::from(s)
    }
}

/// Load and validate the manifest from a TOML file.
pub fn load(path: &Path) -> Result<Vec<Repo>, ManifestError> {
    let text = fs::read_to_string(path)?;
    let manifest: Manifest = toml::from_str(&text)?;
    let root = expand_tilde(&manifest.root);

    let mut repos = Vec::new();
    for (name, raw) in manifest.repos {
        let repo_path = match &raw.path {
            Some(p) => expand_tilde(p),
            None => root.join(&name),
        };

        let kind = match (&raw.upstream, &raw.bookmarks) {
            (Some(upstream), Some(bookmarks)) => {
                let raw_ref = raw.upstream_ref.clone().ok_or_else(|| {
                    ManifestError::Validation(format!(
                        "{name}: `bookmarks` requires `upstream_ref`"
                    ))
                })?;
                let upstream_ref = UpstreamRef::parse(&raw_ref).map_err(|e| {
                    ManifestError::Validation(format!("{name}: {e}"))
                })?;
                if bookmarks.is_empty() {
                    return Err(ManifestError::Validation(format!(
                        "{name}: `bookmarks` must not be empty"
                    )));
                }
                RepoKind::ForkRebase {
                    upstream: upstream.clone(),
                    upstream_ref,
                    bookmarks: bookmarks.clone(),
                }
            }
            (Some(upstream), None) => {
                let raw_ref = raw.upstream_ref.clone().ok_or_else(|| {
                    ManifestError::Validation(format!(
                        "{name}: `upstream` requires `upstream_ref`"
                    ))
                })?;
                let upstream_ref = UpstreamRef::parse(&raw_ref).map_err(|e| {
                    ManifestError::Validation(format!("{name}: {e}"))
                })?;
                RepoKind::ForkTrack {
                    upstream: upstream.clone(),
                    upstream_ref,
                }
            }
            (None, Some(_)) => {
                return Err(ManifestError::Validation(format!(
                    "{name}: `bookmarks` requires `upstream`"
                )));
            }
            (None, None) => {
                if raw.upstream_ref.is_some() {
                    return Err(ManifestError::Validation(format!(
                        "{name}: `upstream_ref` requires `upstream`"
                    )));
                }
                RepoKind::Own
            }
        };

        repos.push(Repo {
            name,
            clone_url: raw.clone,
            path: repo_path,
            kind,
            build: raw.build,
            post_build: raw.post_build,
        });
    }

    Ok(repos)
}
