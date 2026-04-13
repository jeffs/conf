#!/usr/bin/env -S zsh -euo pipefail
#
# This script configures the local host to my liking.

# ----------------------
# INSTALL UPSTREAM STUFF
# ----------------------

~/conf/src/install-brew.zsh
~/conf/src/install-bat.zsh
~/conf/src/install-rust.zsh

# Technically, you don't need fd or rg; but you'll want them.
brew install nu zellij # Later, you can switch to `~/usr/src` forks.
cargo binstall --strategies crate-meta-data fd-find git-delta jj-cli ripgrep

# --------------------
# INSTALL MY OWN STUFF
# (and custom forks)
# --------------------

# Some of my own tools.
cargo install --path ~/conf/prj/jump
cargo install --path ~/conf/prj/upgrade

# Symlinked from `~/conf/bin`.
(cd ~/conf/prj/edit && cargo build --release)
(cd ~/conf/prj/jeff-alias && cargo build --release)

# My fork of Helix, and more of my own tools.
(cd ~/conf/prj/sync && cargo run -- -r helix -r rust-kart)

# ---------------
# CONFIGURE STUFF
# ---------------

# Generate `~/conf/var/env.{json,sh}` for login shells.
(cd ~/conf/prj/jeff-login && cargo run)

# Symlink `~/conf/etc` items, mostly into `~/.config`.
~/conf/src/install-dotfiles.zsh
