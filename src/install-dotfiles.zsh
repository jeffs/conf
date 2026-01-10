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
mkdir -p ~/.claude
ln -s ~/conf/etc/claude/agents ~/.claude/agents
ln -s ~/conf/etc/claude/skills ~/.claude/skills
ln -s ~/conf/src/claude-notify.sh ~/.claude/claude-notify.sh
ln -s ~/conf/etc/claude/settings.json ~/.claude/settings.json
ln -s ~/conf/etc/gitconfig .gitconfig
ln -s ~/conf/etc/nvim .config
ln -s ~/conf/etc/tmux.conf .tmux.conf
ln -s ~/conf/etc/zprofile .zprofile
ln -s ~/conf/etc/zshrc .zshrc

# Symlinking CLAUDE.md is fine on a machine that requires no further
# user-level config. However, on a machine that wants to augment it at all,
# you're better off creating a separate ~/.claude/CLAUDE.md file with a
# @~/conf/etc/claude/CLAUDE.md line.
#
# ln -s ~/conf/etc/claude/CLAUDE.md ~/.claude/CLAUDE.md

# TODO: These paths are specific to macOS. On Linux et al, the path should be
#  $XDG_CONFIG, defaulting to ~/.config.
readonly config_home='Library/Application Support'

mkdir -p "$config_home/rustfmt"
ln -s ~/conf/etc/rustfmt.toml "$config_home/rustfmt"

mkdir -p "$config_home/Code/User"
ln -sf ~/conf/etc/vscode/settings.json "$config_home/Code/User"
