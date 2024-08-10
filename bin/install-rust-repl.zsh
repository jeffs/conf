#!/usr/bin/env -S zsh -euo pipefail
#
# Install the EValuation ConteXt for Rust.

cargo install evcxr_repl
readonly config_dir=$(evcxr <<EOF | tail -1
:dep dirs
println!("{}", dirs::config_dir().unwrap().display());
EOF
)/evcxr
ln -s ~/conf/etc/prelude.rs $config_dir
