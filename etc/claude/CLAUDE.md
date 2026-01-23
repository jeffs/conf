# User-level Claude Code instructions

## Version Control

Use `jj` (Jujutsu) instead of `git` for all version control operations,
including commits, branches (bookmarks), and history manipulation. Do not use
raw git commands for history changes.

Do not include "Generated with Claude Code" in commit or PR messages.
Co-authored-by trailers are fine.

The `var/` directory contains variable/ephemeral data (prompts, temp files) and
is never committed.

## Quality

You MUST PROVE all work is correct before considering it done. Use static gaurantees (such as types) where possible, and thorough testing always.
When weighing time and effort vs. correctness, ALWAYS prioritize correctness.

Parse, don't validate. Make the type system do the work. Strive for mathematical certainty.

## Uncertainty & Verification

When describing CLI commands, APIs, or library functions:
- Distinguish between "I know this exists" vs "I'm inferring this might exist"
- When uncertain, check `--help`, man pages, or docs before asserting

## Philosophy of Design

- Prefer small, orthogonal, composable pieces over monolithic solutions.
- Remember Unix philosophy; composition over inheritance; functional core, imperative shell.

## Process Management

Use the MCP task server for background processes, not raw bash backgrounding
or pkill. This includes dev servers, watch processes, and any long-running
commands.

- `task_ensure` to start (idempotent - safe to call if already running)
- `task_list` to see what's running
- `task_logs` to check output
- `task_stop` to stop cleanly

Never use `pkill`, `kill`, or `&` backgrounding for process management.
