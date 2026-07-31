#!/usr/bin/env -S euo pipefail zsh
#
# TODO: Automate [WezTerm installation](https://wezterm.org/install/macos.html).

tempfile=$(mktemp) \
  && curl -fsSL -o "$tempfile" https://raw.githubusercontent.com/wezterm/wezterm/main/termwiz/data/wezterm.terminfo \
  && tic -x -o ~/.terminfo "$tempfile" \
  && rm "$tempfile"

# WezTerm appends debug-overlay REPL history to a file in this directory, but
# silently skips persistence if the directory itself doesn't exist.
mkdir -p ~/Library/Application\ Support/wezterm
