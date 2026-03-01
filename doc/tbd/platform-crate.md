# Shared Platform Crate (`conf-platform`)

## Goal

A Rust crate in the `~/conf/prj/` workspace that reads platform-specific
and site-specific TOML configuration, exposing it to both Rust CLI tools
(as a library) and Python/Xonsh scripts (via PyO3).

## Config file layout

```
~/conf/etc/platform/
    macos.toml          # macOS defaults
    windows.toml        # Windows defaults
    linux.toml          # Linux defaults (future)

~/conf/etc/site/
    <hostname>.toml     # Host-specific overrides
```

The platform crate:
1. Detects the current platform (`cfg!(target_os)`).
2. Loads the matching `platform/<os>.toml`.
3. Loads `site/<hostname>.toml` if it exists, merging it on top.
4. Exposes the merged config via a typed Rust API and a PyO3 module.

Host-specific TOML overrides any key from the platform TOML. This lets a
machine with an unusual Homebrew prefix or a non-standard editor path
override the default without forking the platform file.

## TOML schema

```toml
# ~/conf/etc/platform/macos.toml

[paths]
# Relative paths are resolved against $HOME.
config_home = "Library/Application Support"
data_home = ".local/share"
pkg_prefix = "/opt/homebrew"
system_paths = [
    "/usr/local/go/bin",
    "/opt/homebrew/bin",
    "/opt/homebrew/opt/sqlite/bin",
    "/usr/local/bin",
    "/usr/bin",
    "/bin",
    "/usr/sbin",
    "/sbin",
    "/Library/Developer/CommandLineTools/usr/bin",
]

[shell]
# Command prefix for running shell snippets.
invoke = ["sh", "-c"]
# Format for generated environment files: "posix" or "powershell".
env_format = "posix"

[package_manager]
name = "brew"
install = ["brew", "install"]
upgrade = ["brew", "upgrade", "--quiet"]

[system_update]
command = ["softwareupdate", "--list"]

[env]
# Static environment variables to set. Values that are relative paths
# are resolved against $HOME (same as jeff-login's home_env).
EDITOR = ".cargo/bin/hx"
VISUAL = ".cargo/bin/hx"
XDG_CONFIG_HOME = ".config"
LESS = "-FRX -j5"
MANPAGER = "col -b | bat -pl man"
HOMEBREW_NO_ENV_HINTS = "true"
RUSTC_WRAPPER = "/opt/homebrew/bin/sccache"

[tools]
# Explicit paths for tools that aren't reliably on PATH.
# Omit a key to find the tool via PATH lookup.
fzf = "/opt/homebrew/bin/fzf"
glow = "/opt/homebrew/bin/glow"
yazi = "/opt/homebrew/bin/yazi"
```

```toml
# ~/conf/etc/platform/windows.toml

[paths]
config_home = "AppData/Roaming"
data_home = "AppData/Local"
pkg_prefix = "C:/ProgramData/chocolatey"
system_paths = [
    "C:/Windows/System32",
    "C:/Windows",
]

[shell]
invoke = ["powershell", "-Command"]
env_format = "powershell"

[package_manager]
name = "choco"
install = ["choco", "install", "-y"]
upgrade = ["choco", "upgrade", "all", "-y"]

[system_update]
command = ["winget", "upgrade", "--all"]

[env]
EDITOR = ".cargo/bin/hx.exe"
VISUAL = ".cargo/bin/hx.exe"

[tools]
# On Windows, most tools are expected to be on PATH.
```

```toml
# ~/conf/etc/site/macbook-pro.toml
# Example host-specific overrides.

[paths]
# This machine has Homebrew on Intel prefix for some reason.
pkg_prefix = "/usr/local"

[tools]
sccache = "/usr/local/bin/sccache"
```

## Rust API

```rust
// conf-platform/src/lib.rs

pub struct Platform {
    pub paths: Paths,
    pub shell: Shell,
    pub package_manager: PackageManager,
    pub system_update: SystemUpdate,
    pub env: BTreeMap<String, String>,
    pub tools: BTreeMap<String, PathBuf>,
}

pub struct Paths {
    pub home: PathBuf,
    pub config_home: PathBuf,      // resolved absolute
    pub data_home: PathBuf,        // resolved absolute
    pub pkg_prefix: PathBuf,
    pub system_paths: Vec<PathBuf>,
}

pub struct Shell {
    pub invoke: Vec<String>,
    pub env_format: EnvFormat,
}

pub enum EnvFormat { Posix, PowerShell }

pub struct PackageManager {
    pub name: String,
    pub install: Vec<String>,
    pub upgrade: Vec<String>,
}

pub struct SystemUpdate {
    pub command: Vec<String>,
}

impl Platform {
    /// Load platform config, with optional site override.
    /// `conf_root` is typically `~/conf/etc`.
    pub fn load(conf_root: &Path) -> Result<Self>;

    /// Resolve a possibly-relative path against $HOME.
    pub fn resolve(&self, path: &str) -> PathBuf;

    /// Look up a tool path: explicit config, then PATH fallback.
    pub fn tool(&self, name: &str) -> Option<PathBuf>;
}
```

## PyO3 bindings

Expose `Platform` as a Python class:

```python
from conf_platform import Platform

p = Platform.load("~/conf/etc")
p.paths.config_home       # Path("/Users/jeff/Library/Application Support")
p.paths.pkg_prefix        # Path("/opt/homebrew")
p.package_manager.name    # "brew"
p.package_manager.install # ["brew", "install"]
p.tool("fzf")             # Path("/opt/homebrew/bin/fzf")
p.env["EDITOR"]           # ".cargo/bin/hx"
p.resolve("conf/bin")     # Path("/Users/jeff/conf/bin")
p.shell.env_format        # "posix"
```

This lets Xonsh startup (`rc.d/config.py`) and Python install scripts use
the same config the Rust tools use.

## Integration points

### jeff-login
Currently hardcodes PATH entries, environment variables, and shell script
generation. All of these move to the platform TOML. `jeff-login` becomes:
1. `Platform::load()`
2. Build PATH from `platform.paths.system_paths` + user paths.
3. Write `env.json` from `platform.env` + computed values.
4. Write `env.sh` or `env.ps1` based on `platform.shell.env_format`.

### jeff-alias
Hardcodes `/opt/homebrew/bin/{fzf,glow}`. Replace with
`platform.tool("fzf")` / `platform.tool("glow")`.

### upgrade
Task definitions for `brew` and `softwareupdate` come from
`platform.package_manager` and `platform.system_update`.

### jump
`JUMP_PREFIXES` splits on `:`. Use `std::env::split_paths` instead, or
read prefix dirs from a config key.

### sync
`sh -c` invocation uses `platform.shell.invoke`.

### install-dotfiles (Python)
Reads `platform.paths.config_home` to know where to symlink configs.

### Xonsh config (rc.d/config.py)
Replaces hardcoded `/opt/homebrew/bin/yazi` with `platform.tool("yazi")`.

## Crate metadata

- Name: `conf-platform`
- Location: `~/conf/prj/conf-platform/`
- Dependencies: `serde`, `toml`, `dirs` (for home dir), `pyo3` (optional feature)
- Workspace member in `~/conf/prj/Cargo.toml`

## Open questions

- Should the platform crate also own the list of packages to install
  (a `[packages]` table), or should that live in a separate
  `packages.toml`? The former is simpler; the latter separates "what
  to install" from "how the platform works."
- Should `env_format` support multiple outputs simultaneously (e.g.,
  `jeff-login` generating both `env.sh` and `env.json`)? Probably yes;
  `env_format` might better be called `shell_format` and only govern
  the shell-specific output.
- How to handle tools that aren't installed yet at config-load time?
  `platform.tool()` should probably return `Option` and let the caller
  decide whether to fail or skip.
