#!/usr/bin/env -S zsh -euo pipefail
#
# Installs the Xonsh command shell.

if ! test -x ~/.local/bin/xonsh; then
  brew install pipx
  pipx install xonsh
else
  pipx upgrade xonsh
fi
