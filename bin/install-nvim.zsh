#!/usr/bin/env -S zsh -euo pipefail

/opt/homebrew/bin/brew install neovim

mkdir -p ~/usr/bin
ln -fs /opt/homebrew/bin/nvim ~/usr/bin/vi
ln -fs /opt/homebrew/bin/nvim ~/usr/bin/vim

# Initialize files per:
# https://lazy.folke.io/installation
