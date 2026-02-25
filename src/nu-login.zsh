#!/usr/bin/env -S zsh -euo pipefail
#
# Zsh is at /bin/zsh at macOS but /usr/bin/zsh on Unbuntu.
# Can't we all just get along?

# Clean up Zsh-specific variables that are meaningless to Nushell.
unset PROMPT_COMMAND PS1 PS2 PS3 PS4

export XDG_CONFIG_HOME=~/.config

exec ~/usr/bin/nu --login "$@"
