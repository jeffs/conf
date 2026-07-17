#!/usr/bin/env nu
#
# Useful for debugging the language server integration:
# <https://github.com/helix-editor/helix/discussions/7203#discussioncomment-7819383>
#
#   hx -v FILES...
#   :log-open
#
#   tail -f ~/.cache/helix/helix.log

# Install the Debugger Adapter Protocol implementation for macOS.
xcrun -f lldb-dap

# Install language servers for CSS, etc.
npm i -g vscode-langservers-extracted

# Install a TOML language server.
cargo install taplo-cli

# Install Obsidian-aware Markdown LSP. Binstall is also available.
cargo install --locked --git https://github.com/Feel-ix-343/markdown-oxide.git markdown-oxide

# Clone or update the Helix fork.
let helix_dir = $"($env.HOME)/usr/src/helix"
if ($helix_dir | path exists) {
    print "Updating Helix fork..."
    git -C $helix_dir pull --ff-only
} else {
    print "Cloning Helix fork..."
    mkdir ($helix_dir | path dirname)
    git clone git@github.com:jeffs/helix.git $helix_dir
}

cargo install --path $"($helix_dir)/helix-term"
