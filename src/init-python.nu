#! /usr/bin/env nu
#
# Initializes a Python project using uv; see `install-uv.zsh`.
#
# Whenever you begin work on a project, you'll need to add the virtual
# environment to your PATH:
#
# ```nushell
# use std/util "path add"
# path add (pwd | path join .venv/bin)
# ```

let name = "my-project"

# https://nathanielknight.ca/articles/helix_for_python.html
uv init $name
cd $name
uv venv
uv pip install python-lsp-server python-lsp-ruff python-lsp-black

mkdir .helix
cp ~/conf/etc/helix/python.toml .helix/languages.toml
