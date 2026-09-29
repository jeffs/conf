#!/bin/sh

# If Claude Code realizes it's running under Zellij, it disables title updates
# while working, so the window (Zellij pane) title always begins with `*`,
# regardless of whether the session is idle; whereas, I want it to update the
# title just as it would outside of Zellij, so I can see which sessions are
# idle.
#
# Sadly, unsetting Zellij breaks scrolling in the Claude Code "fullscreen"
# TUI in Zellij in WezTerm specifically, for reasons beyond my ken. I filed a
# `/bug`. In the meantime, I've gone back to Claude Code's "default" TUI.
unset ZELLIJ

# This makes Claude Code start and exit much faster.
export CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1

exec ~/.local/bin/claude "$@"
