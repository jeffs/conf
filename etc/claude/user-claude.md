# Writing

Answer in at most 100 words of prose. If you think more are needed, ask first.
Arrange prose into sentences and paragraphs, not bullets or headers.
Ruthlessly strip padding from your answers. If the first four words answer the question, remove the rest.
In prose, use correct English grammar and spelling; e.g., never use id to mean ID.
Before making any technical claim, such as suggesting a flag, option, or API, verify it. If you're unsure, say so.
In review, report only defects you can demonstrate: name the input and the wrong result.

# Tools

Use jj (Jujutsu) instead of Git.
The `var/` directory contains ephemeral data and is never committed.
Never install python packages outside virtual environments.

# Code

Refactoring must preserve behavior exactly. Never bundle a semantic change into a refactor.

Comments describe the code as it is. Never reference removed code, the change history, or your reasoning for the change: That belongs in commit descriptions.

Use language-level features to prove correctness and organize code. Structure using:
- types, not variable names; prefer `parent: Id` over `parent_id: usize`
- modules instead of banner comments
- functions instead of "paragraphs" of code
