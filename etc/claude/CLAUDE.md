# User-level Claude Code instructions

## Version Control

Use `jj` (Jujutsu) instead of `git` for all version control operations, including commits, branches (bookmarks), and history manipulation. Do not use raw git commands for history changes.

Do not include "Generated with Claude Code" in commit or PR messages. Co-authored-by trailers are fine.

The `var/` directory contains variable/ephemeral data (prompts, temp files) and
is never committed.

## Quality

You MUST PROVE all work is correct before considering it done. Use static gaurantees (such as types) where possible, and thorough testing always. When weighing time and effort vs. correctness, ALWAYS prioritize correctness.

## Uncertainty & Verification

Never suggest a command, flag, or API to the user unless you have verified it works in this session. If you haven't verified it, say so explicitly.

When defining or explaining anything, ask yourself: Can I make this less sloppy and more accurate?

## Memory

Never write to the memory directory (~/.claude/projects/*/memory/) unless I explicitly ask you to.

## Philosophy of Design

- Parse, don't validate. Make the type system do the work. Strive for mathematical certainty.
- Prefer small, orthogonal, composable pieces over monolithic solutions.
- Remember Unix philosophy; composition over inheritance; functional core, imperative shell.
