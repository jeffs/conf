#!/usr/bin/env -S zsh -euo pipefail
#
# Install the EValuation ConteXt for Rust.

cargo install evcxr_repl

# The config directory varies by platform.
readonly config_dir=$(evcxr <<EOF | tail -1
:dep dirs
println!("{}", dirs::config_dir().unwrap().display());
EOF
)/evcxr

# https://github.com/evcxr/evcxr/blob/main/COMMON.md
mkdir -p "$config_dir"
ln -s ~/conf/etc/init.evcxr "$config_dir"
ln -s ~/conf/etc/prelude.rs "$config_dir"
