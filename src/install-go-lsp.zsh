#!/usr/bin/env -S zsh -euo pipefail
#
# Install Go language servers and tools for Helix.
#
# Prerequisites: go, golangci-lint (brew install golangci-lint)

go install golang.org/x/tools/gopls@latest
go install github.com/nametake/golangci-lint-langserver@latest
go install github.com/go-delve/delve/cmd/dlv@latest
