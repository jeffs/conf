# User-level Claude Code instructions

## Version Control

Use `jj` (Jujutsu) instead of `git` for all version control operations,
including commits, branches (bookmarks), and history manipulation. Do not use
raw git commands for history changes.

Do not include "Generated with Claude Code" in commit or PR messages.
Co-authored-by trailers are fine.

## Markdown Conventions

Stick to common, pandoc-compatible conventions. Include blank lines after headers, and before lists or code blocks.

## Quality

You MUST PROVE all work is correct before considering it done. Use static gaurantees (such as types) where possible, and thorough testing always.
When weighing time and effort vs. correctness, ALWAYS prioritize correctness.
