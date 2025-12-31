#!/usr/bin/env -S zsh -euo pipefail

# shellcheck is a recommended dependency of bash-language-server.
brew install helix shellcheck

# Install the Rust Language Server.
rustup component add rust-analyzer

# Install the Debugger Adapter Protocol implementation for macOS.
xcrun -f lldb-dap

# Install language servers for CSS, etc.
npm i -g vscode-langservers-extracted bash-language-server

# Install a TOML language server.
cargo install taplo-cli

# <https://docs.helix-editor.com/building-from-source.html>
# git clone git@github.com:helix-editor/helix ~/pkg/helix
# cd ~/pkg/helix
# if [[ -f ~/.cargo/bin/hx ]]; then cargo uninstall helix-term; fi
# export HELIX_DEFAULT_RUNTIME=~/pkg/helix/runtime
# cargo install --path helix-term --locked
