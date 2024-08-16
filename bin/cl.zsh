#!/usr/bin/env -S zsh -euo pipefail
#
# Open log project for today (or --yesterday) in VSCode, creating it if it
# doesn't already exist.
#
# TODO: Accept dates as arguments.

if [[ $# -eq 0 ]]; then
    readonly dir=~/log/$(date +%Y/%m/%d)
elif [[ $1 = "--yesterday" ]]; then
    readonly dir=~/log/$(python -m yesterday)
else
    echo "$1: bad option" >&2
    exit 2
fi

mkdir -p $dir
cd $dir

if [ ! -d .git ]; then
    git init
    git commit -m 'Initial commit'
fi

# This flag is a terrible hack.  See also:
# https://stackoverflow.com/questions/76987792/when-starting-vs-code-from-the-cli-can-i-make-the-workspace-trusted-without-dis
code --disable-workspace-trust .
