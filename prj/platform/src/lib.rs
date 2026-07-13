mod merge;

use std::path::{Path, PathBuf};
use std::{fmt, fs, io};

use indexmap::IndexMap;
use serde::Deserialize;

use merge::deep_merge;

// ---------------------------------------------------------------------------
// Error
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Toml(toml::de::Error),
    UnsupportedOs,
    NoHomeDir,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{e}"),
            Self::Toml(e) => write!(f, "{e}"),
            Self::UnsupportedOs => write!(f, "unsupported operating system"),
            Self::NoHomeDir => write!(f, "could not determine home directory"),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Self::Toml(e)
    }
}

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnvValue {
    String(String),
    Bool(bool),
}

/// A single value or a list of values under `[paths]`. Scalars stay scalars,
/// arrays stay arrays — the distinction is preserved through to the output.
#[derive(Debug, Clone)]
pub enum PathEntry {
    Single(PathBuf),
    Multi(Vec<PathBuf>),
}

#[derive(Debug)]
pub struct Platform {
    pub path_env: IndexMap<String, PathEntry>,
    pub package_manager: PackageManager,
    pub system_update: SystemUpdate,
    pub env: IndexMap<String, EnvValue>,
}

#[derive(Debug)]
pub struct PackageManager {
    pub name: String,
    pub install: Vec<String>,
    pub upgrade: Vec<String>,
}

#[derive(Debug)]
pub struct SystemUpdate {
    pub command: Vec<String>,
}

// ---------------------------------------------------------------------------
// Raw TOML shapes (deserialization)
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct RawPlatform {
    #[serde(default)]
    paths: IndexMap<String, toml::Value>,
    package_manager: RawPackageManager,
    system_update: RawSystemUpdate,
    #[serde(default)]
    env: IndexMap<String, toml::Value>,
}

#[derive(Deserialize)]
struct RawPackageManager {
    name: String,
    install: Vec<String>,
    upgrade: Vec<String>,
}

#[derive(Deserialize)]
struct RawSystemUpdate {
    command: Vec<String>,
}

// ---------------------------------------------------------------------------
// Path / tilde helpers
// ---------------------------------------------------------------------------

/// Expand a leading `~/` or bare `~` against `home`. Other strings pass
/// through unchanged.
fn expand_tilde(value: &str, home: &Path) -> String {
    if value == "~" {
        return home.display().to_string();
    }
    if let Some(rest) = value.strip_prefix("~/") {
        return home.join(rest).display().to_string();
    }
    value.to_owned()
}

/// Top-level keys recognized in platform and site TOML files.
const KNOWN_KEYS: &[&str] = &["paths", "package_manager", "system_update", "env"];

fn unknown_keys(table: &toml::Table) -> impl Iterator<Item = &str> {
    table
        .keys()
        .map(String::as_str)
        .filter(|k| !KNOWN_KEYS.contains(k))
}

fn warn_unknown_keys(table: &toml::Table, source: &Path) {
    for key in unknown_keys(table) {
        eprintln!(
            "warning: {}: unrecognized top-level key `{key}` (known keys: {})",
            source.display(),
            KNOWN_KEYS.join(", "),
        );
    }
}

fn platform_toml_name() -> Result<&'static str, Error> {
    if cfg!(target_os = "macos") {
        Ok("macos.toml")
    } else if cfg!(target_os = "windows") {
        Ok("windows.toml")
    } else if cfg!(target_os = "linux") {
        Ok("linux.toml")
    } else {
        Err(Error::UnsupportedOs)
    }
}

// ---------------------------------------------------------------------------
// Platform impl
// ---------------------------------------------------------------------------

impl Platform {
    /// Load platform config.
    ///
    /// `conf_root` is the `~/conf` directory (parent of both `etc/` and `var/`).
    ///
    /// # Errors
    ///
    /// Returns an error if the platform TOML cannot be read or parsed, or if
    /// the home directory cannot be determined.
    pub fn load(conf_root: &Path) -> Result<Self, Error> {
        let home = std::env::home_dir().ok_or(Error::NoHomeDir)?;

        let toml_name = platform_toml_name()?;
        let platform_path = conf_root.join("etc/platform").join(toml_name);
        let site_path = conf_root.join("var/site.toml");

        Self::load_from(&platform_path, &site_path, &home)
    }

    /// Load from explicit file paths (useful for testing).
    fn load_from(platform_path: &Path, site_path: &Path, home: &Path) -> Result<Self, Error> {
        let platform_str = fs::read_to_string(platform_path)?;
        let mut table: toml::Table = toml::from_str(&platform_str)?;
        warn_unknown_keys(&table, platform_path);

        if site_path.is_file() {
            let site_str = fs::read_to_string(site_path)?;
            let site_table: toml::Table = toml::from_str(&site_str)?;
            warn_unknown_keys(&site_table, site_path);
            deep_merge(&mut table, site_table);
        }

        let raw: RawPlatform = toml::Value::Table(table).try_into()?;
        Ok(Self::resolve(raw, home))
    }

    fn resolve(raw: RawPlatform, home: &Path) -> Self {
        let path_env = raw
            .paths
            .into_iter()
            .filter_map(|(key, value)| {
                match value {
                    toml::Value::Array(arr) => {
                        let dirs: Vec<PathBuf> = arr
                            .into_iter()
                            .filter_map(|v| match v {
                                toml::Value::String(s) => Some(home.join(&s)),
                                _ => None,
                            })
                            .collect();
                        if dirs.is_empty() {
                            None
                        } else {
                            Some((key, PathEntry::Multi(dirs)))
                        }
                    }
                    toml::Value::String(s) => Some((key, PathEntry::Single(home.join(&s)))),
                    _ => None,
                }
            })
            .collect();

        let env = raw
            .env
            .into_iter()
            .map(|(k, v)| match v {
                toml::Value::String(s) => (k, EnvValue::String(expand_tilde(&s, home))),
                toml::Value::Boolean(b) => (k, EnvValue::Bool(b)),
                other => (k, EnvValue::String(other.to_string())),
            })
            .collect();

        Self {
            path_env,
            package_manager: PackageManager {
                name: raw.package_manager.name,
                install: raw.package_manager.install,
                upgrade: raw.package_manager.upgrade,
            },
            system_update: SystemUpdate {
                command: raw.system_update.command,
            },
            env,
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn conf_root() -> PathBuf {
        // prj/platform/src/lib.rs → prj/platform → prj → (conf root)
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("could not find conf root")
            .to_owned()
    }

    fn fake_home() -> PathBuf {
        PathBuf::from("/Users/testuser")
    }

    #[test]
    fn load_macos_toml() {
        let root = conf_root();
        let platform_path = root.join("etc/platform/macos.toml");
        let site_path = root.join("var/nonexistent-site.toml");
        let home = fake_home();

        let p = Platform::load_from(&platform_path, &site_path, &home).unwrap();

        assert!(p.path_env.contains_key("PATH"));
        assert!(p.path_env.contains_key("JUMP_DIRS"));
        assert!(p.path_env.contains_key("CAML_LD_LIBRARY_PATH"));
        assert_eq!(p.package_manager.name, "brew");
        assert!(!p.package_manager.install.is_empty());
        assert!(!p.package_manager.upgrade.is_empty());
        assert!(!p.system_update.command.is_empty());
        assert!(!p.env.is_empty());
    }

    #[test]
    fn tilde_expansion_path() {
        let home = fake_home();
        assert_eq!(
            expand_tilde("~/.cargo/bin/hx", &home),
            "/Users/testuser/.cargo/bin/hx"
        );
    }

    #[test]
    fn tilde_expansion_bare() {
        let home = fake_home();
        assert_eq!(expand_tilde("~", &home), "/Users/testuser");
    }

    #[test]
    fn tilde_expansion_literal() {
        let home = fake_home();
        assert_eq!(expand_tilde("-FRX -j5", &home), "-FRX -j5");
    }

    #[test]
    fn tilde_expansion_absolute() {
        let home = fake_home();
        assert_eq!(
            expand_tilde("/opt/homebrew/bin/sccache", &home),
            "/opt/homebrew/bin/sccache"
        );
    }

    #[test]
    fn site_override_merges() {
        let root = conf_root();
        let platform_path = root.join("etc/platform/macos.toml");
        let home = fake_home();

        // Write a temporary site file.
        let tmp = std::env::temp_dir().join("platform-test-site.toml");
        fs::write(&tmp, "[env]\nJUMP_HOME = \"~/home\"\n").unwrap();

        let p = Platform::load_from(&platform_path, &tmp, &home).unwrap();
        assert_eq!(
            p.env.get("JUMP_HOME").unwrap(),
            &EnvValue::String(format!("{}/home", home.display()))
        );

        // Other env vars should still be present from the base.
        assert!(p.env.contains_key("LESS"));

        let _ = fs::remove_file(&tmp);
    }

    #[test]
    fn unknown_keys_flags_top_level_strays() {
        let table: toml::Table =
            toml::from_str("JUMP_DIRS = [\"conf/var\"]\n[env]\nFOO = \"bar\"\n").unwrap();
        let strays: Vec<&str> = unknown_keys(&table).collect();
        assert_eq!(strays, ["JUMP_DIRS"]);
    }

    #[test]
    fn unknown_keys_accepts_known_sections() {
        let table: toml::Table = toml::from_str("[paths]\nJUMP_DIRS = [\"conf/var\"]\n").unwrap();
        assert_eq!(unknown_keys(&table).count(), 0);
    }

    #[test]
    fn env_preserves_toml_order() {
        let root = conf_root();
        let platform_path = root.join("etc/platform/macos.toml");
        let site_path = root.join("var/nonexistent-site.toml");
        let home = fake_home();

        let p = Platform::load_from(&platform_path, &site_path, &home).unwrap();

        let keys: Vec<&str> = p.env.keys().map(String::as_str).collect();
        // The first key in our macos.toml [env] section is LESS.
        assert_eq!(keys[0], "LESS");
    }

}
