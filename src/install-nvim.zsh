#!/usr/bin/env -S zsh -euo pipefail

/opt/homebrew/bin/brew install neovim

mkdir -p ~/usr/bin
ln -s /opt/homebrew/bin/nvim ~/usr/bin/vi
ln -s /opt/homebrew/bin/nvim ~/usr/bin/vim
