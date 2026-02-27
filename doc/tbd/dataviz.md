# Shell data visualization: Nushell tables alternatives in Xonsh

## miller (`mlr`)

Single binary for CSV/JSON/TSV. Awk-like transformations with formatted table
output, fits into pipelines:

    mlr --icsv --opprint sort-by name then head -n 10 data.csv
    mlr --ijson --opprint group-by status then count input.json

Covers ~80% of the Nushell table workflow with zero setup.

## VisiData (`vd`)

Interactive TUI spreadsheet. Opens CSV, JSON, SQLite, Parquet, etc. Good for
exploration when you don't know the shape of the data yet.

## Polars in Xonsh

No context switch since Xonsh is already Python. Polars is the stronger default
over Pandas now: Rust backend, Apache Arrow memory format, lazy evaluation,
automatic parallelization. The expression-based API (`pl.col("x").sort()`) is
more functional/composable than Pandas' mutable-state style. Stricter about
types and nulls. Built-in table formatting when printing DataFrames — no
`tabulate` dependency needed. Hit 1.0 in 2024; ecosystem integrations that
expect Pandas are bridged via `.to_pandas()`. Reach for Pandas only when a
downstream library demands it.

## tidy-viewer (`tv`)

Lightweight Rust tool that renders CSVs as nice tables. Pipe-friendly, zero
config.

## DuckDB

Powerful but adds a conceptual layer that Pandas doesn't in the Xonsh context.
Better suited if the workload is genuinely SQL-shaped or involves large data.
