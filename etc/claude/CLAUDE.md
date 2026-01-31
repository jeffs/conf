# User-level Claude Code instructions

## Version Control

Use `jj` (Jujutsu) instead of `git` for all version control operations, including commits, branches (bookmarks), and history manipulation. Do not use raw git commands for history changes.

Do not include "Generated with Claude Code" in commit or PR messages. Co-authored-by trailers are fine.

The `var/` directory contains variable/ephemeral data (prompts, temp files) and
is never committed.

## Quality

You MUST PROVE all work is correct before considering it done. Use static gaurantees (such as types) where possible, and thorough testing always. When weighing time and effort vs. correctness, ALWAYS prioritize correctness.

## Uncertainty & Verification

When describing CLI commands, APIs, or library functions:
- Distinguish between "I know this exists" vs "I'm inferring this might exist"
- When uncertain, check `--help`, man pages, or docs before asserting

When defining or explaining anything, ask yourself: Can I make this less sloppy and more accurate?

## Philosophy of Design

- Parse, don't validate. Make the type system do the work. Strive for mathematical certainty.
- Prefer small, orthogonal, composable pieces over monolithic solutions.
- Remember Unix philosophy; composition over inheritance; functional core, imperative shell.
