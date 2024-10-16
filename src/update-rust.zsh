#!/usr/bin/env -S zsh -euo pipefail

# Uninstall and reinstall to avoid issues with corrupted component files.  I'm
# not clear on what a component file is or how it becomes corrupted, but it bit
# me by causing rustup update to fail thus:
#
#   error: could not rename component file from
#   '/Users/jeff/.rustup/tmp/hl9f8chal9xk6s3x_dir/bk' to
#   '/Users/jeff/.rustup/toolchains/nightly-aarch64-apple-darwin/etc'
#
#
# https://github.com/rust-lang/rustup/issues/2729
for toolchain in nightly stable; do
    rustup toolchain uninstall $toolchain
    rustup toolchain install $toolchain
done

rustup update
