#!/usr/bin/env nu
#
# SQL tooling: DuckDB (query engine), sqls (language server for Helix).
# Requires: brew, go

# Install DuckDB.
brew install duckdb

# Install the SQL language server used by Helix.
# https://github.com/sqls-server/sqls
go install github.com/sqls-server/sqls@latest
