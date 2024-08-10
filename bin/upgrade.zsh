#!/usr/bin/env -S zsh -euo pipefail

# Upgrade packages installing using Homebrew.
echo '* Homebrew'
brew upgrade --quiet

# Upgrade third-party packages installed using Cargo.  As a hack, consider a
# package "third-party" if its name doesn't include my name; otherwise, cargo
# would try to check crates.io for packages that have the same name as the ones
# I installed from local source code.
echo '* Cargo'
cargo install --quiet $(cargo install --list |rg '^\w' |awk '!/jeff/ { print $1 }')

# Pass --List rahter than --all so I don't get prompted by sudo.
echo '* Software Update Tool'
unbuffer softwareupdate --list |rg -v '^(Software Update Tool|Finding available software|No new software available\.|)$'
