#!/usr/bin/env -S zsh -euo pipefail

# Install the Rust Language Server.
# rustup component add rust-analyzer

# Install the Debugger Adapter Protocol implementation for macOS.
# xcrun -f lldb-dap

# This works, but I'm trying to install it from source to debug some weirdness
# in Rust projects:
#
# * The global symbol picker never shows me any symbols at all
# * Sometimes files are not recognized as belonging to a project.  This comes
#   and goes for any particular file; re-opening sometimes fixes it, and
#   sometimes doesn't.
#
# My SWAG is that both of these issues are related to rust-analyzer integration.
#
# brew install helix

# <https://docs.helix-editor.com/building-from-source.html>
git clone git@github.com:helix-editor/helix ~/pkg/helix
cd ~/pkg/helix
if [[ -f ~/.cargo/bin/hx ]]; then cargo uninstall helix-term; fi
export HELIX_DEFAULT_RUNTIME=~/pkg/helix/runtime
cargo install --path helix-term --locked
