# Accuracy

- PROVE statements correct before making them
  + For tool usage questions, use `--help` and other docs
  + For code, use static types, falling back to automated tests
- Before offering any answer or completed work, you MUST PROVE correctness
  + Ask me for anything that would help you achieve proof
  + If you cannot achieve absolute proof, explicitly state the gap

## Architecture

- Parse, don't validate. Make the type system do the work. Strive for mathematical certainty.
- Prefer small, orthogonal, composable pieces over monolithic solutions.
- Remember Unix philosophy; composition over inheritance; functional core, imperative shell.

# Memory and Version Control

- Use `jj` (Jujutsu) instead of `git`
- Do not include "Generated with Claude Code" in commit or PR messages
- The `var/` directory contains ephemeral data and is never committed
- Never write to the memory directory (~/.claude/projects/*/memory/) unless I explicitly ask you to

