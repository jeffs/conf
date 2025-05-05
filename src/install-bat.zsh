#!/usr/bin/env -S zsh -euo pipefail
#
# # Bugs
#
# The Nushell Sublime syntax (or maybe syntect?) seems a bit buggy. As an
# alternative, use `nu-hightlight`. For comparison:
#
#   config nu --doc | bat -pl nu
#   config nu --doc | nu-highlight | less -R
#
# # See also
#
# * <https://github.com/sharkdp/bat/blob/master/README.md#adding-new-syntaxes--language-definitions>
# * <https://packagecontrol.io/packages/Nushell>

cargo install bat

cd  $(dirname "$0")/..
ln -s "$PWD/etc/bat" "$(bat --config-dir)"
git clone https://github.com/stevenxxiu/sublime_text_nushell etc/bat/syntaxes/Nushell
bat cache --build
