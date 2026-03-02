# Shared Platform Crate (`platform`)

## Status

**Done:**
- `platform` crate created at `~/conf/prj/platform/`.
- `etc/platform/macos.toml` written with all values from `jeff-login`,
  `jeff-alias`, and `upgrade`.
- `jeff-login` migrated to load all env vars and PATH from the platform
  crate. Hardcoded values removed.

**Next:**
- Migrate `jeff-alias` (tool paths: fzf, glow).
- Migrate `upgrade` (brew upgrade, softwareupdate commands).
- Migrate `sync` (shell invocation).

## Config file layout

```
~/conf/etc/platform/
    macos.toml          # macOS defaults
    windows.toml        # Windows defaults (future)
    linux.toml          # Linux defaults (future)

~/conf/var/
    site.toml           # Host-specific overrides (gitignored)
```

The platform crate:
1. Detects the current platform (`cfg!(target_os)`).
2. Loads `etc/platform/<os>.toml`.
3. Loads `var/site.toml` if it exists, deep-merging it on top.
4. Exposes the merged config via a typed Rust API.

## TOML schema

See `~/conf/etc/platform/macos.toml` for the canonical schema.

Key sections: `[paths]`, `[shell]`, `[package_manager]`, `[system_update]`,
`[env]`, `[tools]`.

Merge semantics: nested tables merge key-by-key; scalars and arrays in
the site file replace the platform value entirely.

## Rust API

```rust
pub struct Platform {
    pub paths: Paths,
    pub shell: Shell,
    pub package_manager: PackageManager,
    pub system_update: SystemUpdate,
    pub env: IndexMap<String, String>,     // resolved (~ expanded), TOML order
    pub tools: BTreeMap<String, PathBuf>,  // resolved
}

pub struct Paths {
    pub home: PathBuf,
    pub config_home: PathBuf,      // resolved absolute
    pub pkg_prefix: PathBuf,
    pub home_paths: Vec<PathBuf>,  // resolved absolute
    pub system_paths: Vec<PathBuf>,
}

impl Platform {
    pub fn load(conf_root: &Path) -> Result<Self, Error>;
    pub fn tool(&self, name: &str) -> Option<&Path>;
    pub fn full_path(&self) -> Vec<&Path>;  // home_paths ++ system_paths
}
```

`conf_root` is `~/conf` (parent of both `etc/` and `var/`).

Dependencies: `serde`, `toml` (with `preserve_order`), `indexmap`.
No `dirs` crate — uses `std::env::home_dir()` with `#[allow(deprecated)]`,
matching existing codebase convention.

## Integration points

### jeff-login — DONE
Loads `Platform`, writes `env.json` and `env.sh` from `platform.env`
and `platform.full_path()`. No hardcoded values remain.

### jeff-alias
Hardcodes `/opt/homebrew/bin/{fzf,glow}`. Replace with
`platform.tool("fzf")` / `platform.tool("glow")`.

### upgrade
Task definitions for `brew` and `softwareupdate` come from
`platform.package_manager` and `platform.system_update`.

### sync
`sh -c` invocation uses `platform.shell.invoke`.

### Xonsh config (rc.d/config.py)
Replaces hardcoded `/opt/homebrew/bin/yazi` with `platform.tool("yazi")`
once PyO3 bindings are available. Not a blocker for anything else.

## Optional PyO3 bindings (future)

Behind a `pyo3` feature flag, expose `Platform` as a Python class for
use from Xonsh. These are convenience bindings, not foundational.

## Open questions

- Should the platform crate also own the list of packages to install
  (a `[packages]` table), or should that live in a separate
  `packages.toml`?
- How to handle tools that aren't installed yet at config-load time?
  `platform.tool()` returns `Option` — callers decide whether to fail
  or skip.
