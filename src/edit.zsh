#!/usr/bin/env -S zsh -euo pipefail
#
# Invoke my preferred editing environment from outside an existing Nushell.
# Mainly for use as my Zellij `scrollback_editor`.

HELIX_RUNTIME=~/usr/src/helix/runtime ~/.cargo/bin/hx "$@"
