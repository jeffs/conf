#!/usr/bin/env -S zsh -euo pipefail
#
# Install the EValuation ConteXt for Rust.

mkdir -p ~/pkg
cd ~/pkg
git clone git@github.com:jeffs/evcxr
cd evcxr/evcxr_repl
cargo install --path .

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
