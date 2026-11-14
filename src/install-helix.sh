#!/bin/sh
#
# Useful for debugging the language server integration:
# <https://github.com/helix-editor/helix/discussions/7203#discussioncomment-7819383>
#
#   hx -v FILES...
#   :log-open
#
#   tail -f ~/.cache/helix/helix.log

set -eu

# Install the Debugger Adapter Protocol implementation for macOS.
xcrun -f lldb-dap

# Install language servers for CSS, etc.
npm i -g vscode-langservers-extracted

# Install a TOML language server.
cargo install taplo-cli

# Install Obsidian-aware Markdown LSP. Binstall is also available.
cargo install --locked --git https://github.com/Feel-ix-343/markdown-oxide.git markdown-oxide

# Install the snippet/word completion server configured as scls.
cargo install --locked --git https://github.com/estin/simple-completion-language-server.git

# Clone or update the Helix fork.
helix_dir="$HOME/usr/src/helix"
if [ -e "$helix_dir" ]; then
    echo "Updating Helix fork..."
    jj -R "$helix_dir" git fetch --remote origin
    jj -R "$helix_dir" new custom
else
    echo "Cloning Helix fork..."
    mkdir -p "$(dirname "$helix_dir")"
    jj git clone git@github.com:jeffs/helix.git "$helix_dir"
fi

cargo install --locked --path "$helix_dir/helix-term"
