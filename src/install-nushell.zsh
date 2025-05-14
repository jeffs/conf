#!/usr/bin/env -S zsh -euo pipefail

cargo install nu
git clone --depth=1 https://github.com/nushell/nu_scripts ~/pkg/nu_scripts

# As of 2025-05-04, nufmt is strictly a placeholder project. If you actually run
# it, it only garbles your code in ways that make it invalid. Not exaggerating:
# It doesn't work at all.
#
#  cargo install --git https://github.com/nushell/nufmt
