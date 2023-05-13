#!/usr/bin/env -S zsh -euo pipefail

# The default Python version on macOS as of this writing is 3.9, which isn't
# good enough for Poetry.
brew install pyenv
pyenv global 3.11

curl -sSL https://install.python-poetry.org | python3 -

mkdir ${ZSH_CUSTOM:=$ZSH/custom}/plugins/poetry
poetry completions zsh > $ZSH_CUSTOM/plugins/poetry/_poetry
