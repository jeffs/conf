#!/usr/bin/env -S zsh -euo pipefail
#
# # Prerequisites
#
# * Xcode  

rustup target add wasm32-unknown-unknown

# This specific binary is recommended by the Dioxus docs.
cargo binstall dioxus-cli
