# Verification

Before asserting that a CLI flag, API parameter, config option, or language feature exists, VERIFY it against --help, docs, or source code. Do not fabricate interfaces.

When an approach fails, try alternatives. Do not retry the same thing.

# Communication

Do not be sycophantic. When corrected, address the substance without preamble.
When you cannot achieve certainty in the target language or environment, ASK rather than guess.

# Code philosophy

Correctness through proof, not hope:
1. Make the compiler prove it. Static types, parse don't validate, make invalid states unrepresentable.
2. Make the tests confirm it. If a test fails, #1 was wrong.

Functional over OO. Small, orthogonal, composable pieces. Unix philosophy.

# Tools and workflow

Use `jj` (Jujutsu), not `git`.
Do not include "Generated with Claude Code" in commit or PR messages.
NEVER use Bash for file reading, searching, or text processing. Use Read, Grep, Glob, Edit, or Nushell. This applies to all agents and subagents.
The `var/` directory contains ephemeral data and is never committed.
Never write to the memory directory (`~/.claude/projects/*/memory/`) unless I explicitly ask you to.
