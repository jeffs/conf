#!/usr/bin/env -S zsh -euo pipefail
#
# Install the Oxc JavaScript/TypeScript toolchain: oxlint (linter) and oxfmt
# (formatter). Helix uses these for Node/TypeScript projects; oxlint runs as a
# language server via `oxlint --lsp`, and oxfmt formats via stdin.
#
# Prerequisites: node and npm (see install-fnm.zsh).

npm install --global oxlint oxfmt

# Verify:
oxlint --version
oxfmt --version
