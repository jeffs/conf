#!/usr/bin/env -S zsh -euo pipefail

# Upgrade packages installing using Homebrew.
echo '* Homebrew'
brew upgrade --quiet

# Upgrade Rust and its tooling, then any third-party packages installed using
# Cargo.  As a hack, consider a package "third-party" if its name doesn't
# include my name; otherwise, cargo would try to check crates.io for packages
# that have the same name as the ones I installed from local source code.
#
# The `rustup update` command is occasionally bemused by some kind of cache
# corruption.  Should that occur, run `update-rust.zsh`.
echo '* Rustup'
rustup update

echo '* Cargo'
# Update cratest that were not installed from local filesystem paths.  The
# output of `cargo install --list` includes paths in parentheses, so it's not
# hard to grep them out.
cargo install --quiet $(cargo install --list |awk '/^[^ ][^(]*$/ { print $1 }')

# Pass --List rahter than --all so I don't get prompted by sudo.
echo '* Software Update Tool'
unbuffer softwareupdate --list |rg -v '^(Software Update Tool|Finding available software|No new software available\.|)$'
