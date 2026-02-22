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

/// Inferred behavior for a repo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepoKind {
    /// Fork with upstream + bookmarks to rebase and push.
    ForkRebase {
        upstream: String,
        upstream_ref: String,
        bookmarks: Vec<String>,
    },
    /// Fork tracking upstream, no custom bookmarks.
    ForkTrack {
        upstream: String,
        upstream_ref: String,
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
                let upstream_ref = raw.upstream_ref.clone().ok_or_else(|| {
                    ManifestError::Validation(format!(
                        "{name}: `bookmarks` requires `upstream_ref`"
                    ))
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
                let upstream_ref = raw.upstream_ref.clone().ok_or_else(|| {
                    ManifestError::Validation(format!(
                        "{name}: `upstream` requires `upstream_ref`"
                    ))
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
