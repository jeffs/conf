#!/usr/bin/env nu
#
# <https://github.github.com/spec-kit/upgrade.html>
#
# # TODO
#
# Automatically update `/speckit.*` commands:
#
#   specify init --here --force --ai claude
#
# There are a few obstacles:
#
# * The command scripts are per-project, not systemic.
# * <https://github.github.com/spec-kit/upgrade.html#1-constitution-file-will-be-overwritten>
#
#   > Known issue: `specify init --here --force` currently overwrites
#   > `.specify/memory/constitution.md` with the default template, erasing any
#   > customizations you made.

uv tool install specify-cli --force --from git+https://github.com/github/spec-kit.git
