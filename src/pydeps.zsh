#!/usr/bin/env -S zsh -euo pipefail

pydeps --reverse --rankdir=BT "$@"
