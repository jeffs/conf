#!/usr/bin/env -S zsh -euo pipefail
#
# Install the [Rust]( https://www.rust-lang.org/ ) toolchain.

brew install openssl pkg-config

curl -fsSLo /tmp/rustup.sh https://sh.rustup.rs/
sh /tmp/rustup.sh -q -y --no-modify-path
rm /tmp/rustup.sh

# Not currently using binstall because it may install untrusted binaries.
cargo install -F fix cargo-audit
cargo install cargo-watch sccache

# Without the rust-analyzer component, Helix can't find rust-analyzer.
rustup component add rust-analyzer
