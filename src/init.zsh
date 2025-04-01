#!/usr/bin/env -S zsh -euo pipefail
#
# This script configures the local host to my liking.

~/conf/src/install-brew.zsh
~/conf/src/install-dotfiles.zsh
~/conf/src/install-omz.zsh
~/conf/src/install-zsh-syntax-highlighting.zsh
~/conf/src/install-nvim.zsh
~/conf/src/install-rust.zsh

# The 'expect' package includes unbuffer, which is used by bin/upgrade.zsh.
brew install entr expect tmux jq pyenv
cargo install bat fd-find ripgrep sd

for p in cl log-profile jump; do
    cargo install --path ~/conf/prj/$p
done
