#!/usr/bin/env -S zsh -euo pipefail
#
# TODO
#
# * [] Automate upgrades of forked packages, such as evcxr_repl
# * [] Automate upgrades of apps from installers: Docker, Firefox, Steam

# Upgrade packages installing using Homebrew.
echo '* Homebrew'
brew upgrade --quiet

# Upgrade Rust and its tooling. The `rustup update` command is occasionally
# bemused by some kind of cache corruption.  Should that occur, run
# `update-rust.zsh`.
echo '* Rustup'
rustup update

# Update all crates that were not installed from local filesystem paths.  The
# output of `cargo install --list` includes such paths in parentheses, so it's
# not hard to grep them out.
echo '* Cargo'
cargo install $(cargo install --list |awk '/^[^ ][^(]*$/ { print $1 }')

# Passing --install --all rather than --list would cause sudo to prompt for
# access, but this script is not meant to be interactive.
echo '* Software Update Tool'
unbuffer softwareupdate --list \
  | rg -v '^(Software Update Tool|Finding available software|No new software available\.|)$'
