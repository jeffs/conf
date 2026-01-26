#!/usr/bin/env -S zsh -euo pipefail


# On some machines, I use a fork of Nushell; on others, I use Homebrew.
# I symlink whichever I'm using so there's a single, portable path to find it.
brew install nushell
mkdir -p ~/usr/bin
ln -s /opt/homebrew/bin/nu ~/usr/bin

if [ ! -d ~/pkg/nu_scripts ]; then
  git clone --depth=1 https://github.com/nushell/nu_scripts ~/pkg/nu_scripts
else
  cd ~/pkg/nu_scripts
  git pull
fi

# As of 2025-05-04, nufmt is strictly a placeholder project. If you actually run
# it, it only garbles your code in ways that make it invalid. Not exaggerating:
# It doesn't work at all.
#
#  cargo install --git https://github.com/nushell/nufmt
