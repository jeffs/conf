#!/usr/bin/env -S zsh -euo pipefail
#
# (Re-)installs the uv package manager for Python. See also:
# <https://docs.astral.sh/uv/getting-started/installation/#upgrading-uv>
#
# # Alternatives
# 
# * `uv self update` doesn't work unless `uv` was installed via the "manager"
#   (shell script) on the website.
#
# * You can install `uv` directly from GitHub, but cargo doesn't seem to cache
#   build artifacts between runs
#   - `cargo install --git https://github.com/astral-sh/uv uv`
#   - This still doesn't support `self update`
#
# # Requirements
#
# Git and Rust/Cargo must already be installed, and in PATH.
#
# # TODO
#
# Replace this script with a Rust program.

if [ ! -d ~/pkg/uv ]; then
    git clone git@github.com:astral-sh/uv ~/pkg/uv
    cd ~/pkg/uv
else
    cd ~/pkg/uv
    git pull
fi

cargo install --path crates/uv
