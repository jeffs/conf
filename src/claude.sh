#!/bin/sh
#
# Launch Claude with the ZELLIJ environment variable unset. If Claude Code
# realizes it's running under Zellij, it disables title updates while working,
# so the window (Zellij pane) title always begins with `*`, regardless of
# whether the session is idle.

exec env -u ZELLIJ ~/.local/bin/claude "$@"
