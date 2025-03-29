#!/usr/bin/env -S zsh -euo pipefail

cargo install --path .
ln -s ~/conf/etc/jump ~/.config/
