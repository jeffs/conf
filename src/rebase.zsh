#!/usr/bin/env -S zsh -euo pipefail

cd ~/conf/prj
cargo run -p rebase -- "$@"
