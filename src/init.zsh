#!/usr/bin/env -S zsh -euo pipefail
#
# This script configures the local host to my liking.
#
# TODO: Add a test suite to detect forward references. The last time I ran this,
#  it tripped on several missing dependencies requiring manual correction.

# ----------------------
# INSTALL UPSTREAM STUFF
# ----------------------

~/conf/src/install-brew.zsh
~/conf/src/install-bat.zsh
~/conf/src/install-rust.zsh

# Technically, you don't need fd or rg; but you'll want them. Get ripgrep via
# brew rather than binstall because opencode needs the homebrew version, anyway.
#
# Later, you can switch to `~/usr/src` forks of nu and zellij.
brew install nu zellij git-delta ripgrep
cargo binstall --strategies crate-meta-data fd-find jj-cli

# --------------------
# INSTALL MY OWN STUFF
# (and custom forks)
# --------------------

# Some of my own tools.
cargo install --path ~/conf/prj/jump

# Symlinked from `~/conf/bin`.
(cd ~/conf/prj/edit && cargo build --release)
(cd ~/conf/prj/alias && cargo build --release)

# My fork of Helix, and more of my own tools.
(cd ~/conf/prj/rebase && cargo run -- -r helix -r rust-kart)

# ---------------
# CONFIGURE STUFF
# ---------------

# Generate `~/conf/var/env.{json,sh}` for login shells.
(cd ~/conf/prj/mkenv && cargo run)

# Symlink `~/conf/etc` items, mostly into `~/.config`.
~/conf/src/install-dotfiles.zsh
