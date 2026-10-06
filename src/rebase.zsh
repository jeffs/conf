#!/usr/bin/env -S zsh -euo pipefail

# Don't wait for rebase to fail for lack of keys to access GitHub repos.
if ! ssh-add -l >/dev/null; then
    echo "$@" 'Run ssh-add.' >&2
    exit 1
fi

# Build with cargo, but run the binary directly: a child of `cargo run` would
# inherit RUSTUP_TOOLCHAIN from the rustup shim, overriding each repo's own
# toolchain resolution in the builds rebase spawns.
cd ~/conf/prj
cargo build --quiet --release --package rebase
exec target/release/rebase "$@"
