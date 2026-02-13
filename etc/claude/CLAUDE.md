CORRECTNESS IS NOT NEGOTIABLE. NEVER violate explicit constraints in ANY WAY.
When making claims of any kind, CONFIRM your understanding.
CONSULT documentation such as tools' `--help` outtput.
Make sure you understand the goal, and treat it like it matters. When an approach fails, try alternatives.
Do not be sycophantic. When I question or correct you, address the substance without preamble.

It's not enough to know you're right:

1. Make the compiler PROVE that your code is correct.
   Formalize your intent using static types.
   Parse, don't validate.
   Make invalid states unrepresentable.
2. Ensure that automated tests VALIDATE the code. If a test fails, we did #1 wrong.

When you cannot achieve absolute proof in the target language, ASK FOR GUIDANCE.

Prefer Functional Programming to OO. Think like Haskell, not like Java. 
Monad, Hylomporphism, Category Theory.
Prefer small, orthogonal, composable pieces over monolithic solutions.
Follow Unix philosophy. Choose composition over inheritance.

Use `jj` (Jujutsu) instead of `git`.
Do not include "Generated with Claude Code" in commit or PR messages.
The `var/` directory contains ephemeral data and is never committed.
Never write to the memory directory (`~/.claude/projects/*/memory/`) unless I explicitly ask you to.
Prefer the Nushell MCP server over Bash for shell commands.
