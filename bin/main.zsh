#!/usr/bin/env -S zsh -euo pipefail
#
# This script is meant to be the entrypoint launched by my terminal.

if [[ ! -v SSH_AUTH_SOCK ]]; then
    if tmux showenv -g SSH_AUTH_SOCK >& /dev/null; then
        export $(tmux showenv -g SSH_AUTH_SOCK)
    else
        eval `ssh-agent`
        tmux start
        tmux setenv -g SSH_AUTH_SOCK $SSH_AUTH_SOCK
    fi
fi

exec tmux new -As Local
