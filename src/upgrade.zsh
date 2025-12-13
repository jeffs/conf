#!/usr/bin/env -S zsh -euo pipefail
#
# TODO
#
# - [] Upgrade:
#   * forked packages, such as evcxr_repl
#   * apps from installers: Docker, Firefox, Slack, Steam
#   * Docker images

# Start SSH agent and prompt for passphrase up front, so this script doesn't
# pause for a passphrase half way through.
eval "$(ssh-agent -s)"
ssh-add

# Upgrade Homebrew packages.
echo '* Homebrew'
echo '** brew upgrade --quiet'
brew upgrade --quiet

# Upgrade Rust and its tooling. The `rustup update` command is occasionally
# bemused by some kind of cache corruption.  Should that occur, run
# `update-rust.zsh`.
echo '* Rustup'
echo '** rustup update'
rustup update

# Update all crates that were not installed from local filesystem paths.  The
# output of `cargo install --list` includes such paths in parentheses, so it's
# not hard to grep them out.
echo '* Cargo'
readonly crates=$(cargo install --list | awk '/^[^ ][^(]*$/ { print $1 }' | tr '\n' ' ')
echo "** cargo install $crates"
cargo install ${(s: :)crates}

~/conf/src/install-nushell.zsh
~/conf/src/install-helix.nu

# Run this occasionally to avoid undue bitrot.
# ~/conf/src/install-uv.zsh

# Passing --install --all rather than --list would cause sudo to prompt for
# access, but this script is not meant to be interactive.
echo '* Software Update Tool'
echo '** softwareupdate --list'
unbuffer softwareupdate --list \
  | rg -v '^(Software Update Tool|Finding available software|No new software available\.|)$'
