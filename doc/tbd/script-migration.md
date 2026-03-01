# Script Migration: Replace .nu/.zsh with Rust or Python

## Goal

Eliminate Nushell (`.nu`) and Zsh (`.zsh`) scripts from `~/conf/src/` and
`~/conf/prj/`, replacing them with Python or Rust as appropriate. Purge
configs and automation for tools no longer in use.

## Purge list

Delete outright:

| File | Reason |
|------|--------|
| `src/install-poetry.zsh` | Replaced by `uv` |
| `src/install-nushell.zsh` | No longer primary shell |
| `src/install-nvm.zsh` | Redundant with `fnm` |
| `prj/cl/cl.zsh` | Unused; Cursor dependency removed |
| `prj/cl/src/lib.rs` reference to `/usr/local/bin/cursor` | Dead code |
| `app/cl.app/`, `app/cl.command`, `app/cy.command` | Unused Cursor launchers |
| `etc/nushell/` | Migrating away from Nushell |
| `src/coverage.zsh` | Trivial wrapper; run `cargo tarpaulin` directly |
| `src/dig-dirt.zsh` | Trivial wrapper |
| `src/pycharm.zsh` | Trivial `open -a` wrapper |
| `src/pydeps.zsh` | One-line alias; put in Xonsh config |

Also audit `doc/tbd/nushell.md` — may be obsolete.

## Rewrite in Python

These scripts have logic worth preserving and should become Python modules
or standalone scripts importable from Xonsh. They should use the shared
platform crate (via PyO3) for paths and package manager commands once it
exists; until then, they can hardcode macOS values with `TODO` comments.

| File | Lines | Notes |
|------|-------|-------|
| `src/upgrade.zsh` | 57 | Main upgrade orchestrator. Parallel tasks: brew, rustup, cargo, uv, system update. The Rust `upgrade` TUI may supersede this. |
| `src/install-dotfiles.zsh` | 59 | Symlink layout. Must handle `Library/Application Support` vs `XDG_CONFIG_HOME` vs `%APPDATA%`. Critical for platform crate integration. |
| `src/install-awscliv2.zsh` | 44 | Platform-specific install methods (macOS `installer -pkg` vs Windows MSI). |
| `src/install-aws-session-manager.zsh` | 26 | Same: macOS `.pkg` installer. |
| `src/install-helix.nu` | 43 | Complex: git clone, cargo build, LSP installs, `xcrun` for lldb-dap. |
| `src/install-rust.zsh` | 22 | Multi-tool: rustup, sccache, cargo-binstall, etc. |
| `src/install-uv.zsh` | 33 | Critical tool; install/update logic. |
| `src/install-xonsh.zsh` | 11 | Bootstrap: `uv tool install xonsh[full]`. |
| `src/init.zsh` | 42 | Top-level bootstrap. Calls other installers, builds Rust projects. |
| `src/init-python.nu` | 23 | Project init: `uv init`, venv, LSP tools. |
| `src/install-spec-kit.nu` | 21 | `uv tool install` with extras. |
| `src/install-sql.nu` | 12 | DuckDB + sqls. |
| `src/install-yazi.nu` | 7 | Yazi + deps. |
| `src/install-karts.zsh` | 25 | Multi-step Rust builds from `~/pkg/rust-kart`. |
| `src/install-nvim-plugins.zsh` | 35 | Git clone/pull for Neovim plugins. |
| `src/rebase.zsh` | 15 | Git rebase workflow. Consider whether `jj` makes this obsolete. |
| `src/watch-windows.zsh` | 36 | macOS-only debug tool; keep macOS-only. |
| `src/zed.zsh` | 25 | Editor launcher with venv handling. |
| `src/gable.zsh` | 6 | Pretty git log. |
| `src/loccount.nu` | 10 | LOC delta on branch. |
| `src/on-click-file.nu` | 9 | macOS file-click handler; could also be inlined into the Automator workflow or rewritten in Rust. |

## Rewrite in Rust (or inline)

| File | Lines | Notes |
|------|-------|-------|
| `src/edit.zsh` | 7 | Helix scrollback editor launcher. Could be a mode of the existing `edit` Rust binary, or deleted if Zellij config can point directly at `hx`. |
| `src/on-click-file.nu` | 9 | Alternative: rewrite in Rust and compile to a small binary, since the Automator workflow just needs to exec something. |
| `prj/jump/install.zsh` | 5 | `cargo install` + symlink. Could be handled by `sync` or `init`. |
| `src/init-term.zsh` | 10 | Downloads wezterm terminfo via `curl` + `tic`. Could be a Python function. |

## Trivial installers to fold into a single install script

These are one or two `brew install` calls with no real logic. Rather than
individual scripts, fold them into a package list in the platform TOML
(or a `packages.toml`) and have the Python installer iterate it.

| File | Packages |
|------|----------|
| `src/install-bat.zsh` | `bat` + Nushell syntax (syntax install may still need a script) |
| `src/install-brew.zsh` | Homebrew itself |
| `src/install-fnm.zsh` | `fnm` + Node LTS |
| `src/install-fzf.nu` | `fzf` (clone + symlink; could use brew instead) |
| `src/install-gemini.nu` | `@google/gemini-cli` via npm |
| `src/install-gh.zsh` | GitHub CLI (builds from source; could use brew) |
| `src/install-ghostscript.zsh` | `ghostscript`, `tcl-tk` |
| `src/install-nvim.zsh` | `neovim` + symlinks |
| `src/install-psql.zsh` | `libpq` |
| `src/install-claude-code.nu` | Claude Code CLI |
| `src/install-dioxus.zsh` | WASM target + `dioxus-cli` |

## Migration order

1. Purge dead files (no dependencies, safe to delete now).
2. Build the shared platform crate (see `platform-crate.md`).
3. Rewrite `install-dotfiles` in Python (it defines the symlink layout
   everything else depends on).
4. Rewrite `init.zsh` in Python as the top-level entry point.
5. Migrate individual install scripts, starting with the ones you use
   most often (`install-rust`, `install-uv`, `install-helix`).
6. Fold trivial installers into a package list.
7. Migrate remaining utility scripts (`upgrade`, `gable`, `loccount`, etc.).

## Open questions

- Should `upgrade.zsh` be replaced by the Rust `upgrade` TUI, or by a
  Python script, or both (TUI for interactive, Python for scripted)?
- Is `rebase.zsh` still needed given `jj`?
- Should `on-click-file` be Rust (tiny binary) or Python (simpler to
  change)? Could it be inlined into the Automator workflow?
- Should trivial installers become a declarative package list in TOML
  rather than imperative scripts?
