#!/usr/bin/env -S zsh -euo pipefail
#
# https://wezterm.org/config/lua/config/term.html

tempfile=$(mktemp) \
  && curl -o $tempfile https://raw.githubusercontent.com/wezterm/wezterm/main/termwiz/data/wezterm.terminfo \
  && tic -x -o ~/.terminfo $tempfile \
  && rm $tempfile

