#!/usr/bin/env -S zsh -euo pipefail
#
# Install the [Rust]( https://www.rust-lang.org/ ) toolchain.

# Installating sccache from source would be slow. See:
# <https://github.com/mozilla/sccache?tab=readme-ov-file#via-cargo>
brew install openssl pkg-config sccache

curl -fsSLo /tmp/rustup.sh https://sh.rustup.rs/
sh /tmp/rustup.sh -q -y --no-modify-path
rm /tmp/rustup.sh

# Not currently using binstall because it may install untrusted binaries.
cargo install -F fix cargo-audit
cargo install cargo-update # Used by `~/conf/prj/upgrade`.
cargo install cargo-watch

# You may also have to run this command in projects where you want the LSP
# server to run, so that rustup installs RA for the correct Rust version; but
# this at least prepares a baseline for `cargo new` projects.
rustup component add rust-analyzer
