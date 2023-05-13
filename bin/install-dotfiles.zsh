#!/usr/bin/env -S zsh -euo pipefail

cd

# Create cache and history directories for Neovim.
for d in back swap undo view; do
    mkdir -p var/nvim/$d
done
chmod -R 700 var/nvim

# Move existing config files out of the way.
for f in .config/nvim .gitconfig .tmux.conf .zprofile .zshrc; do
    if [ -e $f ]; then
        mkdir -p orig
        mv $f orig
    fi
done

mkdir -p .config
ln -s ~/conf/etc/nvim .config
ln -s ~/conf/etc/gitconfig .gitconfig
ln -s ~/conf/etc/tmux.conf .tmux.conf
ln -s ~/conf/etc/zprofile .zprofile
ln -s ~/conf/etc/zshrc .zshrc
