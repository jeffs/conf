#!/usr/bin/env -S zsh -euo pipefail
#
# Copied from:
# https://github.com/fireflyprotocol/event-dispatcher-v3/blob/b7790c9f53ac2780dda9ec93c566a98dad22840f/.github/workflows/pr_checks.yaml

cargo tarpaulin --out Xml --engine llvm --output-dir ./target --exclude-files 'gen/*' --exclude-files 'submodules/*' --exclude-files 'src/boundary/zmq/*' --exclude-files 'src/main.rs'  --exclude-files 'src/app.rs' --exclude-files 'src/test_utils.rs'
