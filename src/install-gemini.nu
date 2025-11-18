#!/usr/bin/env nu
#
# Installs the Google Gemini CLI, an agentic AI with a TUI. The "CLI" in the
# name is misleading.
#
# If `npm` is not found, run `install-fnm.zsh` first.

npm install --global @google/gemini-cli
gemini -v
