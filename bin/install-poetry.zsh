#!/usr/bin/env -S zsh -euo pipefail

curl -sSL https://install.python-poetry.org | python3 -

mkdir $ZSH_CUSTOM/plugins/poetry
poetry completions zsh > $ZSH_CUSTOM/plugins/poetry/_poetry
