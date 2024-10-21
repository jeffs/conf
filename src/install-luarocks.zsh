#!/usr/bin/env -S zsh -euo pipefail
#
# Installs [LuaRocks][], the package manager for Lua modules, which is a
# prerequisite of [lazy.nvim][].
#
# [LuaRocks]: https://luarocks.org/#quick-start
# [lazy.nvim]: https://github.com/folke/lazy.nvim?tab=readme-ov-file

mkdir -p ~/opt/luarocks ~/pkg/luarocks
cd ~/pkg/luarocks
curl -LSsO https://luarocks.org/releases/luarocks-3.11.1.tar.gz
tar zxpf luarocks-3.11.1.tar.gz
cd luarocks-3.11.1
./configure --prefix=$HOME/opt/luarocks
make
make install
luarocks install luasocket
# lua
# Lua 5.3.5 Copyright (C) 1994-2018 Lua.org, PUC-Rio
# > require "socket"
