#!/usr/bin/env -S zsh -euo pipefail
#
# Accepts paths to .git directories, and prints the paths to whichever of those
# directories' parent repos have uncommitted changes.

for arg in "$@"; do
    cd "$(dirname "$arg")"
    git diff --quiet || pwd
    # git remote -v |rg :jeffs || true
    # if [ "$(git remote -v |rg :jeffs)" ]; then pwd; fi
    cd -
done
