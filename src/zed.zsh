#!/usr/bin/env -S zsh -euo pipefail

readonly zed_prefix=/Applications/Zed.app/Contents/MacOS

export PATH="$zed_prefix:$PATH"

# For whatever reason, the Zed CLI doesn't recognize an exported Python venv.
# export ZED_FORCE_CLI_MODE=true

# If we're already in a Zed terminal, launch Zed's own CLI.
if [[ -v ZED_TERM ]]; then
    exec $zed_prefix/cli "$@"
fi

#"How bizarre." ---OMC
#
# If you redirect stdout and stderr:
#
# * Zed doesn't recognize exported venv.
# * Zed crashes the parent terminal window; at least in iTerm.
# * Zed doesn't steal focus when launched.
#
# If you redirect only stderr, Zed has none of these three problems.
exec zed "$@" 2>/dev/null &
