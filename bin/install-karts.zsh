#!/usr/bin/env -S zsh -euo pipefail

mkdir -p ~/git
cd ~/git

git clone git@github.com:jeffs/rust-kart
cd rust-kart
cargo install --path tmux-send
