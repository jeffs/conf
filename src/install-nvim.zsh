#!/usr/bin/env -S zsh -euo pipefail
#
# TODO: Automate [lazy.vim setup](https://lazy.folke.io/):
#
# * Ensure Neovim and Git versions
# * Install a Nerd Font
# * Install luarocks


/opt/homebrew/bin/brew install neovim

mkdir -p ~/usr/bin
ln -s /opt/homebrew/bin/nvim ~/usr/bin/vi
ln -s /opt/homebrew/bin/nvim ~/usr/bin/vim
