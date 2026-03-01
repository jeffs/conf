# Cross-Platform Support (Windows)

## Goal

Make the tools in `~/conf/prj/` and the config/install infrastructure in
`~/conf/` work on Windows, assuming the platform crate and script migration
from the other two specs are complete.

This spec covers the remaining Rust-level and config-level changes that
aren't addressed by the platform TOML or the script rewrite.

## Prerequisites

- `conf-platform` crate exists and is integrated (see `platform-crate.md`).
- Install scripts are Python, not zsh/nu (see `script-migration.md`).
- Obsolete tools purged (Cursor, Poetry, Nushell configs).

## Rust API changes in `~/conf/prj/`

### 1. Replace `std::os::unix::process::CommandExt::exec()`

**Affected:** `edit/src/main.rs`, `jeff-alias/src/main.rs`

On Unix, `exec()` replaces the process image (no parent lingers). On
Windows, spawn the child and exit with its status.

```rust
#[cfg(unix)]
fn exec_command(mut cmd: Command) -> ! {
    use std::os::unix::process::CommandExt;
    let err = cmd.exec();
    panic!("exec failed: {err}");
}

#[cfg(windows)]
fn exec_command(mut cmd: Command) -> ! {
    let status = cmd.status().expect("failed to spawn");
    std::process::exit(status.code().unwrap_or(1));
}
```

Both binaries call `exec_command` instead of `.exec()` directly.

### 2. Replace `std::os::unix::ffi::OsStrExt` byte access

**Affected:** `edit/src/main.rs`, `jump/src/expansion.rs`, `jump/src/main.rs`

Use `as_encoded_bytes()` (stable since Rust 1.74, works on all platforms)
instead of the Unix-only `as_bytes()`.

For reconstructing an `OsStr` from sliced bytes, two options:

- **`OsStr::from_encoded_bytes_unchecked`**: Requires `unsafe`, but is
  trivially sound when slicing ASCII prefixes from valid WTF-8. Prefer
  this when the result must remain an `OsStr` (e.g., passing to
  `Command::args`).
- **`Path::strip_prefix`**: Fully safe, no platform imports. Prefer this
  when the operation maps naturally to path manipulation (stripping
  `a/`, `b/`, or `~` prefixes).

Specific changes:

| File | Current code | Replacement |
|------|-------------|-------------|
| `edit/src/main.rs` | `OsStr::from_bytes(&bytes[2..])` to strip `a/`/`b/` | `Path::new(s).strip_prefix("a/")` or `strip_prefix("b/")` |
| `jump/src/expansion.rs` | `OsStr::from_bytes(&bytes[1..])` to strip `~` | `as_encoded_bytes()` + `from_encoded_bytes_unchecked` (tilde isn't a path prefix in the `Path::strip_prefix` sense) |
| `jump/src/main.rs` | `path.as_os_str().as_bytes()` for stdout write | `path.as_os_str().as_encoded_bytes()` (drop-in replacement) |

### 3. Replace `tput cols` in `jeff-alias`

**Affected:** `jeff-alias/src/main.rs`

Used to pass `--width` to `glow`. Options:

- Use the `terminal_size` crate (small, no dependencies).
- On Windows, skip the `--width` flag entirely (glow handles it).
- `#[cfg]`-gate: use `terminal_size` on all platforms, fall back to a
  default if detection fails.

### 4. Replace `:` PATH separator in `jump`

**Affected:** `jump/src/lib.rs`

`JUMP_PREFIXES` is split on `:`. Use `std::env::split_paths` which
handles `:` on Unix and `;` on Windows. Or split on both `:` and `;`.

### 5. Replace forward-slash absolute path detection

**Affected:** `jeff-login/src/main.rs`, `jump/src/expansion.rs`

`starts_with('/')` to detect absolute paths. Use `Path::is_absolute()`
which handles both `/` (Unix) and `C:\` (Windows).

### 6. Replace `sh -c` in `sync`

**Affected:** `sync/src/jj.rs`

Read `platform.shell.invoke` from the platform crate instead of
hardcoding `["sh", "-c"]`.

### 7. ANSI escape codes in `sync/src/output.rs`

Windows Terminal supports VT100 sequences. Classic `cmd.exe` does not,
but can be enabled via `SetConsoleMode`. Options:

- Do nothing (Windows Terminal is the target; it works).
- Use `anstream` crate for automatic detection.
- `#[cfg(windows)]` call `SetConsoleMode` at startup to enable VT
  processing, which covers both Windows Terminal and `cmd.exe`.

Recommendation: do nothing for now; revisit if `cmd.exe` support matters.

## Config file changes

### `etc/zellij/config.kdl`

Replace hardcoded absolute paths:
```
default_shell "/Users/jeff/.local/bin/xonsh"
scrollback_editor "/Users/jeff/conf/src/edit.zsh"
```

These can't easily read TOML at parse time. Options:
- Use `~` expansion (Zellij may support it).
- Template the file from `install-dotfiles` using platform config values.
- Use environment variables if Zellij supports `$SHELL` or similar.

### `etc/claude/lsp.json`

Contains hardcoded `/Users/jeff/` paths for language servers. Same
templating approach, or use `~` if Claude Code supports it.

### `etc/wezterm/wezterm.lua`

Already has fallback logic for Zellij path. Extend to check
platform-appropriate locations, or read the platform TOML from Lua
(wezterm has `wezterm.read_dir` and `io.open`).

### `etc/xonsh/rc.d/config.py`

Replace hardcoded `/opt/homebrew/bin/yazi` with a `conf_platform.tool()`
call once PyO3 bindings exist.

### `etc/nushell/login.nu`

Delete along with Nushell migration.

### `etc/pythonrc.py`

Hardcodes `/opt/homebrew/lib` for Ghostscript. Replace with platform
crate lookup or `ctypes.util.find_library`.

## Shell environment generation

`jeff-login` currently generates `env.sh` (POSIX `export` syntax) and
`env.json`. Add a PowerShell output mode:

```powershell
# env.ps1 (generated)
$env:EDITOR = "$HOME\.cargo\bin\hx.exe"
$env:VISUAL = "$HOME\.cargo\bin\hx.exe"
$env:LESS = "-FRX -j5"
$env:PATH = "C:\Users\jeff\conf\bin;C:\Users\jeff\.cargo\bin;..."
```

The format is selected by `platform.shell.env_format`. Xonsh on Windows
continues to read `env.json` directly via `rc.d/config.py`.

## Install procedure differences

These are handled by the Python install scripts (see `script-migration.md`)
but noted here for completeness:

| Tool | macOS | Windows |
|------|-------|---------|
| AWS CLI v2 | `installer -pkg` | MSI installer or `msiexec` |
| AWS Session Manager | macOS `.pkg` | Windows `.exe` installer |
| Homebrew packages | `brew install` | `choco install` or `winget install` |
| System updates | `softwareupdate --list` | `winget upgrade --all` |
| Xonsh | `uv tool install xonsh[full]` | Same (Python is cross-platform) |
| Helix | `cargo install` + `xcrun -f lldb-dap` | `cargo install` + skip `xcrun` |
| Neovim | `brew install neovim` | `choco install neovim` |

## Symlink layout (install-dotfiles)

macOS config locations vs Windows equivalents:

| App | macOS | Windows |
|-----|-------|---------|
| VS Code | `~/Library/Application Support/Code/User/settings.json` | `%APPDATA%/Code/User/settings.json` |
| Rustfmt | `~/Library/Application Support/rustfmt/rustfmt.toml` | `%APPDATA%/rustfmt/rustfmt.toml` |
| Viddy | `~/Library/Application Support/viddy/viddy.toml` | `%APPDATA%/viddy/viddy.toml` |
| Helix | `~/.config/helix/` | `%APPDATA%/helix/` |
| Git | `~/.gitconfig` | `~/.gitconfig` (same) |
| Bat | `~/.config/bat/` | `%APPDATA%/bat/` |

The platform crate's `paths.config_home` drives these. The Python
`install-dotfiles` script maps each app's config to the right location.

## Phased rollout

### Phase 1: Rust API portability (one weekend)

Changes that don't require the platform crate:

1. `exec_command` abstraction in `edit` and `jeff-alias`.
2. `as_encoded_bytes()` / `Path::strip_prefix` in `edit` and `jump`.
3. `terminal_size` crate in `jeff-alias`.
4. `std::env::split_paths` in `jump`.
5. `Path::is_absolute()` in `jeff-login` and `jump`.

After this phase, all Rust code compiles on Windows (though it won't
have correct paths yet).

### Phase 2: Platform crate integration (one weekend)

1. Build `conf-platform` crate with TOML loading.
2. Integrate into `jeff-login` (PATH, env vars, shell output).
3. Integrate into `jeff-alias` (tool paths).
4. Integrate into `upgrade` (task commands).
5. Integrate into `sync` (shell invocation).
6. Write `macos.toml` with current hardcoded values.
7. Write `windows.toml` with best-guess values.

### Phase 3: PyO3 bindings and config migration (one weekend)

1. Add PyO3 feature to `conf-platform`.
2. Migrate `rc.d/config.py` to use platform crate.
3. Migrate `install-dotfiles` (now Python) to use platform crate.
4. Template or fix `zellij/config.kdl`, `claude/lsp.json`.

### Phase 4: Windows testing and polish (one weekend)

1. Set up a Windows dev environment.
2. Run `cargo build` on the workspace — fix any remaining issues.
3. Write `windows.toml` with tested values.
4. Run `install-dotfiles` on Windows — fix symlink layout.
5. Test Xonsh startup with `env.json`.
6. Write a `site/` TOML for the Windows machine if needed.

## macOS-only tools (no Windows port needed)

- `watch-windows.zsh` (macOS debugging tool)
- `app/on-file-click.app` (Automator workflow)
- `app/` directory generally
- `itco-edit` (iTerm2 coprocess — works anywhere, but only useful on macOS)

## Open questions

- Should the Rust workspace use `#[cfg]` gates inline, or factor
  platform-specific code into separate modules (`platform/unix.rs`,
  `platform/windows.rs`)? For the small number of differences,
  inline `#[cfg]` is probably fine.
- Is `cmd.exe` compatibility worth pursuing, or is Windows Terminal
  the only target? (Recommendation: Windows Terminal only.)
- Should `jeff-login` detect the platform at runtime (for cross-compiled
  binaries) or at compile time (`#[cfg]`)? Compile time is simpler and
  these are personal tools, not distributed binaries.
- On Windows, should symlinks be used (requires developer mode or admin),
  or should configs be copied? Junctions are another option for directories.
