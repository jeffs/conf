#!/usr/bin/env -S zsh -euo pipefail
#
# The name of this script refers to "karts," which are repositories of code
# I've written, arranged by programming language: go-kart, py-kart, rust-kart.
# In practice, only rust-kart is still in use, partly because I prefer the Rust
# language, but also because Go and Python package management have proven
# painful whereas Rust package management has been straightforward.
#
# This script also installs fewer commands than it once did, as I've moved
# toward tools supporting subcommands, sometimes wrapped in shell aliases.
 
readonly parent=~/pkg

mkdir -p $parent
cd $parent

if [[ ! -d $parent/rust-kart ]]; then
  git clone git@github.com:jeffs/rust-kart $parent/rust-kart
fi

cargo install --path $parent/rust-kart/crates/tmux-send
cargo install --path $parent/rust-kart/crates/vimod
