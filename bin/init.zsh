#!/usr/bin/env -S zsh -euo pipefail
#
# This script configures the local host to my liking.

~/conf/bin/install-brew.zsh
~/conf/bin/install-dotfiles.zsh
~/conf/bin/install-omz.zsh
~/conf/bin/install-zsh-syntax-highlighting.zsh
~/conf/bin/install-nvim.zsh
~/conf/bin/install-rust.zsh

brew install entr tmux jq
cargo install bat fd-find
