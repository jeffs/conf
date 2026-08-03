#!/usr/bin/env -S zsh -euo pipefail

# Build with cargo, but run the binary directly: a child of `cargo run` would
# inherit RUSTUP_TOOLCHAIN from the rustup shim, overriding each repo's own
# toolchain resolution in the builds rebase spawns.
cd ~/conf/prj
cargo build -p rebase
exec target/debug/rebase "$@"
