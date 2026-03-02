# Script Migration: Replace .nu/.zsh with Rust

## Goal

Eliminate Nushell (`.nu`) and Zsh (`.zsh`) scripts from `~/conf/src/` and
`~/conf/prj/`, replacing them with Rust. Purge configs and automation for
tools no longer in use.

Python and Xonsh are leaf dependencies of this system, not foundational.
The entire install and config infrastructure is Rust. `uv` is just another
tool that `init` installs; it in turn installs Python, then Xonsh. If
Xonsh is dropped later, nothing else breaks.

## Purge list

Already deleted:

| File | Reason |
|------|--------|
| `src/install-poetry.zsh` | Replaced by `uv` |
| `src/install-nushell.zsh` | No longer primary shell |
| `src/install-nvm.zsh` | Redundant with `fnm` |
| `prj/cl/cl.zsh` | Unused; Cursor dependency removed |
| `prj/cl/src/lib.rs` reference to `/usr/local/bin/cursor` | Dead code |
| `app/cl.app/`, `app/cl.command`, `app/cy.command` | Unused Cursor launchers |
| `src/coverage.zsh` | Trivial wrapper; run `cargo tarpaulin` directly |
| `src/dig-dirt.zsh` | Trivial wrapper |
| `src/pycharm.zsh` | Trivial `open -a` wrapper |
| `src/pydeps.zsh` | One-line alias; put in Xonsh config |

`etc/nushell/` is kept for now.

## The `init` binary

A Rust binary in the workspace that replaces `src/init.zsh` as the
top-level bootstrap entry point. It uses the `platform` crate to
read platform/site TOML for all paths and commands.

`init` is idempotent: running it on a fully set-up machine either no-ops
or converges to the desired state. Each step checks before acting (e.g.,
skip download if `uv` is present, skip symlink if it points to the right
target, skip package if already installed). This means `init` doubles as
both first-time setup and "repair/update my environment."

Responsibilities:
1. Install the platform's package manager if needed (Homebrew on macOS).
2. Install packages from a declarative list (see "Package list" below).
3. Install Rust toolchain (rustup, cargo-binstall, sccache, etc.).
4. Build and install workspace tools (`cargo install --path`).
5. Run `sync` to build forked packages.
6. Create the symlink/dotfile layout (replaces `install-dotfiles.zsh`).
7. Install `uv` (just another tool).
8. Run `uv python install` + `uv tool install xonsh[full]` (just another
   step, like installing any other tool).
9. Generate `env.json` / `env.sh` / `env.ps1` (or delegate to `jeff-login`).

The `init` binary is the only thing you need on a fresh machine. No
Python, no shell scripts, no package manager preinstalled.

## Rewrite in Rust

These scripts have logic worth preserving. They become either part of
the `init` binary, subcommands of a workspace tool, or standalone
binaries.

| File | Lines | Notes |
|------|-------|-------|
| `src/init.zsh` | 42 | Becomes the `init` binary itself. |
| `src/install-dotfiles.zsh` | 59 | Symlink layout. Becomes a module/function in `init`. Uses `platform.paths.config_home` for cross-platform paths. |
| `src/upgrade.zsh` | 57 | The Rust `upgrade` TUI already exists. Extend it to cover what the shell script does. |
| `src/install-rust.zsh` | 22 | Multi-tool: rustup, sccache, cargo-binstall. Becomes a function in `init`. |
| `src/install-uv.zsh` | 33 | Download and run the uv installer. Becomes a function in `init`. |
| `src/install-xonsh.zsh` | 11 | `uv tool install xonsh[full]`. Becomes a function in `init`. |
| `src/install-helix.nu` | 43 | Complex: git clone, cargo build, LSP installs, `xcrun` for lldb-dap. Could be a function in `init` or handled by `sync`. |
| `src/install-awscliv2.zsh` | 44 | Platform-specific install methods (macOS `installer -pkg` vs Windows MSI). Function in `init` with platform branching. |
| `src/install-aws-session-manager.zsh` | 26 | Same: macOS `.pkg` installer. Function in `init`. |
| `src/install-karts.zsh` | 25 | Multi-step Rust builds from `~/pkg/rust-kart`. Function in `init` or handled by `sync`. |
| `src/install-nvim-plugins.zsh` | 35 | Git clone/pull for Neovim plugins. Function in `init`. |
| `src/init-python.nu` | 23 | Project init: `uv init`, venv, LSP tools. Standalone Rust binary or subcommand. |
| `src/install-spec-kit.nu` | 21 | `uv tool install` with extras. Function in `init`. |
| `src/install-sql.nu` | 12 | DuckDB + sqls. Function in `init` or package list. |
| `src/install-yazi.nu` | 7 | Yazi + deps. Package list. |
| `src/edit.zsh` | 7 | Helix scrollback editor launcher. Mode of the existing `edit` binary, or inline in Zellij config. |
| `src/on-click-file.nu` | 9 | macOS file-click handler. Small standalone Rust binary, or inline into the Automator workflow. |
| `src/init-term.zsh` | 10 | Downloads wezterm terminfo via `curl` + `tic`. Function in `init`. |
| `src/zed.zsh` | 25 | Editor launcher with venv handling. Standalone Rust binary or `jeff-alias` subcommand. |
| `src/gable.zsh` | 6 | Pretty git log. `jeff-alias` subcommand or standalone. |
| `src/loccount.nu` | 10 | LOC delta on branch. `jeff-alias` subcommand or standalone. |
| `src/rebase.zsh` | 15 | Git rebase workflow. May be obsolete given `jj`. |
| `src/watch-windows.zsh` | 36 | macOS-only debug tool. Standalone Rust binary. |
| `prj/jump/install.zsh` | 5 | `cargo install` + symlink. Handled by `init`. |

## Package list

Trivial installers (one or two `brew install` calls with no real logic)
become entries in a declarative package list rather than individual
scripts. This list lives in the platform TOML or a separate
`packages.toml`, and `init` iterates it using `platform.package_manager`.

| Former script | Packages |
|---------------|----------|
| `src/install-bat.zsh` | `bat` (syntax install may still need custom logic) |
| `src/install-brew.zsh` | Homebrew itself (bootstrap, not a package) |
| `src/install-fnm.zsh` | `fnm` + Node LTS |
| `src/install-fzf.nu` | `fzf` |
| `src/install-gemini.nu` | `@google/gemini-cli` via npm |
| `src/install-gh.zsh` | `gh` |
| `src/install-ghostscript.zsh` | `ghostscript`, `tcl-tk` |
| `src/install-nvim.zsh` | `neovim` + symlinks |
| `src/install-psql.zsh` | `libpq` |
| `src/install-claude-code.nu` | Claude Code CLI |
| `src/install-dioxus.zsh` | WASM target + `dioxus-cli` |

## Migration order

1. ~~Purge dead files (no dependencies, safe to delete now).~~ Done.
2. ~~Build the shared platform crate (see `platform-crate.md`).~~ Done.
   ~~Migrate `jeff-login` to use it.~~ Done.
3. Build the `init` binary with core bootstrap logic: package manager,
   Rust toolchain, workspace builds, symlink layout.
4. Add `uv` / Python / Xonsh installation as a step in `init`.
5. Move trivial installers to the package list.
6. Migrate remaining complex installers into `init` functions.
7. Migrate utility scripts (`gable`, `loccount`, `zed`, etc.) into
   `jeff-alias` subcommands or standalone binaries.
8. Delete all `.zsh` and `.nu` files from `src/`.

## Open questions

- Should `init` be one binary with subcommands (`init install-dotfiles`,
  `init install-rust`, etc.) or a monolithic bootstrap that runs
  everything in sequence? Subcommands are more flexible for re-running
  individual steps.
- Is `rebase.zsh` still needed given `jj`?
- Should `on-click-file` be a standalone Rust binary or inlined into
  the Automator workflow?
- Should the package list live in the platform TOML (`[packages]`
  table) or a separate `packages.toml`? Separate file keeps platform
  config focused on "how" and packages focused on "what."
- How should `init` handle packages that need different install methods
  per platform (e.g., `gh`: `brew install gh` on macOS, `winget install
  GitHub.cli` on Windows)? Possibly a per-platform packages table.
