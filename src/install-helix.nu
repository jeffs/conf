#!/usr/bin/env nu

# Install the Rust Language Server.
rustup component add rust-analyzer

# Install the Debugger Adapter Protocol implementation for macOS.
xcrun -f lldb-dap

# Install language servers for CSS, etc.
npm i -g vscode-langservers-extracted

# Install a TOML language server.
cargo install taplo-cli

# Based on: 
# <https://docs.helix-editor.com/building-from-source.html>
#
# Alternatively:
# ```nu
# brew install helix
# ```
def install-helix-from-source [] {
  # Clone or fetch the Helix source code.
  if not ('~/pkg/helix' | path exists) {
    git clone git@github.com:helix-editor/helix ~/pkg/helix
    cd ~/pkg/helix
  } else {
    cd ~/pkg/helix
    git fetch
    git merge origin/master
  }
  # Make sure Helix builds before uninstalling any existing command.
  cargo build --release
  if ('~/.cargo/bin/hx' | path exists) {
    cargo uninstall helix-term
  }
  # Install Helix, setting the default runtime directory to the working copy.
  $env.HELIX_DEFAULT_RUNTIME = '~/pkg/helix/runtime' | path expand
  cargo install --path helix-term --locked
}

install-helix-from-source
