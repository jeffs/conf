#!/usr/bin/env -S zsh -euo pipefail
#
# This script configures the local host to my liking.

~/conf/src/install-brew.zsh

# TODO: Move dotfile linking to `install-*.zsh` scripts.
~/conf/src/install-dotfiles.zsh

# TODO: Don't install Zsh niceties by default, since I've moved to Nushell as my
#  daily driver. I'm leaving these here until I can make sure my Zsh config
#  works properly without them.
~/conf/src/install-omz.zsh
~/conf/src/install-zsh-syntax-highlighting.zsh

~/conf/src/install-bat.zsh
~/conf/src/install-rust.zsh

brew install difftastic entr helix nu tmux jq
cargo install fd-find ripgrep sd

for p in cl edit jump upgrade; do
    cargo install --path ~/conf/prj/$p
done

(cd ~/conf/prj/log-profile && cargo build --release)
