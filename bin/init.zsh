#!/usr/bin/env -S zsh -euo pipefail
#
# This script configures the local host to my liking.

~/conf/bin/install-brew.zsh
~/conf/bin/install-dotfiles.zsh
~/conf/bin/install-omz.zsh
~/conf/bin/install-zsh-syntax-highlighting.zsh
~/conf/bin/install-nvim.zsh
~/conf/bin/install-rust.zsh

# The 'expect' package includes unbuffer, which is used by bin/upgrade.zsh.
brew install entr expect tmux jq pyenv
cargo install bat fd-find ripgrep sd
