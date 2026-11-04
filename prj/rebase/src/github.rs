//! GitHub repositories, as named by `gh`.

use std::fmt;

/// A GitHub repository's `owner/name`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Slug {
    pub owner: String,
    pub name: String,
}

impl Slug {
    /// Parse an SSH, scp-style, or HTTPS GitHub URL; `None` if `url` names
    /// some other host or isn't a repository URL.
    #[must_use]
    pub fn from_url(url: &str) -> Option<Self> {
        let path = [
            "git@github.com:",
            "ssh://git@github.com/",
            "https://github.com/",
        ]
        .iter()
        .find_map(|prefix| url.strip_prefix(prefix))?;
        let path = path.trim_end_matches('/');
        let path = path.strip_suffix(".git").unwrap_or(path);
        let (owner, name) = path.split_once('/')?;
        if owner.is_empty() || name.is_empty() || name.contains('/') {
            return None;
        }
        Some(Self {
            owner: owner.into(),
            name: name.into(),
        })
    }
}

impl fmt::Display for Slug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.owner, self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::Slug;

    fn slug(url: &str) -> Option<String> {
        Slug::from_url(url).map(|s| s.to_string())
    }

    #[test]
    fn parses_github_urls() {
        for url in [
            "git@github.com:jeffs/helix.git",
            "git@github.com:jeffs/helix",
            "ssh://git@github.com/jeffs/helix.git",
            "https://github.com/jeffs/helix",
            "https://github.com/jeffs/helix.git",
            "https://github.com/jeffs/helix/",
        ] {
            assert_eq!(slug(url).as_deref(), Some("jeffs/helix"), "{url}");
        }
    }

    #[test]
    fn rejects_other_urls() {
        for url in [
            "git@example.com:jeffs/helix.git",
            "https://github.com/jeffs",
            "https://github.com/jeffs/helix/tree/main",
            "https://github.com//helix",
        ] {
            assert_eq!(slug(url), None, "{url}");
        }
    }
}
