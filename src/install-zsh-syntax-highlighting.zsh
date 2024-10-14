#!/usr/bin/env -S zsh -euo pipefail
#
# Install a syntax highlighting plugin for Zsh.
#
# To install a particular version: git clone --branch=0.7.1 ...

git clone --depth=1 \
    https://github.com/zsh-users/zsh-syntax-highlighting.git \
    ~/.oh-my-zsh/plugins/zsh-syntax-highlighting
