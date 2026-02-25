#!/usr/bin/env -S zsh -euo pipefail
#
# This script configures the local host to my liking.

~/conf/src/install-brew.zsh

# TODO: Move dotfile linking to `install-*.zsh` scripts.
~/conf/src/install-dotfiles.zsh

~/conf/src/install-bat.zsh
~/conf/src/install-rust.zsh

brew install difftastic entr nu tmux jq viddy
cargo install fd-find ripgrep sd
cargo binstall --strategies crate-meta-data jj-cli

# Forks
~/conf/src/install-helix.nu
~/conf/src/install-nushell.zsh

for p in cl edit jump upgrade; do
    cargo install --path ~/conf/prj/$p
done

(cd ~/conf/prj/log-profile && cargo build --release)
