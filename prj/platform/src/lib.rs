mod merge;

use std::collections::BTreeMap;
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

#[derive(Debug)]
pub struct Platform {
    pub paths: Paths,
    pub shell: Shell,
    pub package_manager: PackageManager,
    pub system_update: SystemUpdate,
    pub env: IndexMap<String, String>,
    pub tools: BTreeMap<String, PathBuf>,
}

#[derive(Debug)]
pub struct Paths {
    pub home: PathBuf,
    pub config_home: PathBuf,
    pub pkg_prefix: PathBuf,
    pub home_paths: Vec<PathBuf>,
    pub system_paths: Vec<PathBuf>,
    pub jump_prefixes: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct Shell {
    pub invoke: Vec<String>,
    pub env_format: EnvFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvFormat {
    Posix,
    PowerShell,
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
    paths: RawPaths,
    shell: RawShell,
    package_manager: RawPackageManager,
    system_update: RawSystemUpdate,
    #[serde(default)]
    env: IndexMap<String, String>,
    #[serde(default)]
    tools: BTreeMap<String, String>,
}

#[derive(Deserialize)]
struct RawPaths {
    config_home: String,
    pkg_prefix: PathBuf,
    home_paths: Vec<String>,
    system_paths: Vec<PathBuf>,
    #[serde(default)]
    jump_prefixes: Vec<String>,
}

#[derive(Deserialize)]
struct RawShell {
    invoke: Vec<String>,
    env_format: RawEnvFormat,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum RawEnvFormat {
    Posix,
    #[serde(rename = "powershell")]
    PowerShell,
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

fn expand_tilde_path(value: &str, home: &Path) -> PathBuf {
    PathBuf::from(expand_tilde(value, home))
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
        #[allow(deprecated)]
        let home = std::env::home_dir().ok_or(Error::NoHomeDir)?;

        let toml_name = platform_toml_name()?;
        let platform_path = conf_root.join("etc/platform").join(toml_name);
        let site_path = conf_root.join("var/site.toml");

        Self::load_from(&platform_path, &site_path, &home)
    }

    /// Load from explicit file paths (useful for testing).
    fn load_from(
        platform_path: &Path,
        site_path: &Path,
        home: &Path,
    ) -> Result<Self, Error> {
        let platform_str = fs::read_to_string(platform_path)?;
        let mut table: toml::Table = toml::from_str(&platform_str)?;

        if site_path.is_file() {
            let site_str = fs::read_to_string(site_path)?;
            let site_table: toml::Table = toml::from_str(&site_str)?;
            deep_merge(&mut table, site_table);
        }

        let raw: RawPlatform = toml::Value::Table(table).try_into()?;
        Ok(Self::resolve(raw, home))
    }

    fn resolve(raw: RawPlatform, home: &Path) -> Self {
        let config_home = if raw.paths.config_home.starts_with('/') {
            PathBuf::from(&raw.paths.config_home)
        } else {
            home.join(&raw.paths.config_home)
        };

        let home_paths = raw
            .paths
            .home_paths
            .iter()
            .map(|p| home.join(p))
            .collect();

        let jump_prefixes = raw
            .paths
            .jump_prefixes
            .iter()
            .map(|p| home.join(p))
            .collect();

        let env = raw
            .env
            .into_iter()
            .map(|(k, v)| (k, expand_tilde(&v, home)))
            .collect();

        let tools = raw
            .tools
            .into_iter()
            .map(|(k, v)| (k, expand_tilde_path(&v, home)))
            .collect();

        Self {
            paths: Paths {
                home: home.to_owned(),
                config_home,
                pkg_prefix: raw.paths.pkg_prefix,
                home_paths,
                system_paths: raw.paths.system_paths,
                jump_prefixes,
            },
            shell: Shell {
                invoke: raw.shell.invoke,
                env_format: match raw.shell.env_format {
                    RawEnvFormat::Posix => EnvFormat::Posix,
                    RawEnvFormat::PowerShell => EnvFormat::PowerShell,
                },
            },
            package_manager: PackageManager {
                name: raw.package_manager.name,
                install: raw.package_manager.install,
                upgrade: raw.package_manager.upgrade,
            },
            system_update: SystemUpdate {
                command: raw.system_update.command,
            },
            env,
            tools,
        }
    }

    /// Look up an explicit tool path from the `[tools]` table.
    pub fn tool(&self, name: &str) -> Option<&Path> {
        self.tools.get(name).map(PathBuf::as_path)
    }

    /// Return the full `PATH` list: `home_paths` followed by `system_paths`.
    pub fn full_path(&self) -> Vec<&Path> {
        self.paths
            .home_paths
            .iter()
            .chain(&self.paths.system_paths)
            .map(PathBuf::as_path)
            .collect()
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

        assert_eq!(p.paths.config_home, home.join("Library/Application Support"));
        assert_eq!(p.paths.pkg_prefix, PathBuf::from("/opt/homebrew"));
        assert!(!p.paths.home_paths.is_empty());
        assert!(!p.paths.system_paths.is_empty());
        assert_eq!(p.shell.invoke, vec!["sh", "-c"]);
        assert_eq!(p.shell.env_format, EnvFormat::Posix);
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
            &format!("{}/home", home.display())
        );

        // Other env vars should still be present from the base.
        assert!(p.env.contains_key("EDITOR"));

        let _ = fs::remove_file(&tmp);
    }

    #[test]
    fn env_preserves_toml_order() {
        let root = conf_root();
        let platform_path = root.join("etc/platform/macos.toml");
        let site_path = root.join("var/nonexistent-site.toml");
        let home = fake_home();

        let p = Platform::load_from(&platform_path, &site_path, &home).unwrap();

        let keys: Vec<&str> = p.env.keys().map(String::as_str).collect();
        // The first key in our macos.toml [env] section is EDITOR.
        assert_eq!(keys[0], "EDITOR");
    }

    #[test]
    fn full_path_order() {
        let root = conf_root();
        let platform_path = root.join("etc/platform/macos.toml");
        let site_path = root.join("var/nonexistent-site.toml");
        let home = fake_home();

        let p = Platform::load_from(&platform_path, &site_path, &home).unwrap();
        let path = p.full_path();

        // Home paths come first (they are relative, resolved to $HOME).
        assert!(path[0].starts_with(&home));
        // System paths come after (they are absolute).
        let last = path.last().unwrap();
        assert!(last.is_absolute());
    }

    #[test]
    fn tool_lookup() {
        let root = conf_root();
        let platform_path = root.join("etc/platform/macos.toml");
        let site_path = root.join("var/nonexistent-site.toml");
        let home = fake_home();

        let p = Platform::load_from(&platform_path, &site_path, &home).unwrap();
        assert_eq!(
            p.tool("fzf").unwrap(),
            Path::new("/opt/homebrew/bin/fzf")
        );
        assert!(p.tool("nonexistent").is_none());
    }
}
