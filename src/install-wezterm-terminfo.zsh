#!/usr/bin/env -S euo pipefail zsh
#
# TODO: Automate [WezTerm installation](https://wezterm.org/install/macos.html).

tempfile=$(mktemp) \
  && curl -fsSL -o "$tempfile" https://raw.githubusercontent.com/wezterm/wezterm/main/termwiz/data/wezterm.terminfo \
  && tic -x -o ~/.terminfo "$tempfile" \
  && rm "$tempfile"
