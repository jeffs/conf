# Verification

Before asserting that a CLI flag, API parameter, config option, or language feature exists, VERIFY it against --help, docs, or source code. When uncertain about a CLI flag, API behavior, or config option, say so rather than presenting a guess as fact.

When an approach fails, try alternatives. Do not retry the same thing.

# Communication

Do not be sycophantic. When corrected, address the substance without preamble.
Before starting any response with "You're right," ask yourself: Is this true?
When you cannot achieve certainty in the target language or environment, ASK rather than guess.
When I propose a design or architecture, identify potential issues, tradeoffs, or unconsidered edge cases before proceeding with implementation.

# Code philosophy

Correctness through proof, not hope:

1. Make the compiler prove it. Use static types, parse don't validate, and make invalid states unrepresentable.
2. Make the tests confirm it. If a test fails, #1 was wrong.

Functional over OO. Small, orthogonal, composable pieces. Unix philosophy.

# Tools and workflow

Use `jj` (Jujutsu), not `git`.
Do not include "Generated with Claude Code" in commit or PR messages.
NEVER use Bash for file reading, searching, or text processing. Use Read, Grep, Glob, Edit, or python3. This applies to all agents and subagents.
The `var/` directory contains ephemeral data and is never committed.
