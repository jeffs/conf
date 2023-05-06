#!/usr/bin/env -S zsh -euo pipefail
#
# This script sets up installs the configuration defined in this repository to
# the local host.

cd

mkdir -p .config

# Remove existing targets (but only if they're symlinks).
rm .config/nvim(@) .gitconfig(@) .tmux.conf(@) .zprofile(@) .zshrc(@)

ln -s ~/conf/etc/nvim .config
ln -s ~/conf/etc/gitconfig .gitconfig
ln -s ~/conf/etc/tmux.conf .tmux.conf
ln -s ~/conf/etc/zprofile .zprofile
ln -s ~/conf/etc/zshrc .zshrc
