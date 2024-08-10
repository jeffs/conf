#!/usr/bin/env -S zsh -eu
#
# This script is meant to be the entrypoint launched by standalone terminals
# like iTerm.  Its chief value is launching a tmux session: Each new terminal
# gets its own session called Local N, where N is the number of live sesions
# (including the new one).
#
# On macOS, the following should be set as the terminal's command. PATH needs
# /opt/homebrew/bin for tmux, /usr/bin for grep and wc, and /bin for zsh:
#
#   /usr/bin/env PATH=/opt/homebrew/bin:/usr/bin:/bin:$PATH ~/conf/bin/main.zsh
#
# On macOS specifically, we don't have to muck with SSH_AUTH_SOCKET, which is
# set automatically by launchd.

typeset -i n=$(tmux list-sessions | wc -l)
for i in $(seq $(expr $n + 1)); do
    declare name="Local $i"
    if ! tmux ls | grep --quiet "^$name:"; then
        exec tmux new -As "$name"
    fi
done

echo "~/conf/bin/main.zsh: error: couldn't find free session number ($n='$n')" > /tmp/main.err
