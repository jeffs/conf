---
description: Answer a question about this codebase
argument-hint: [question]
allowed-tools: Agent, Read, Bash
---

Question: $ARGUMENTS

Do not search or read files in this thread. Delegate that to a subagent via
the Agent tool: pass the question verbatim, and require it to return the
answer plus the `path:line` locations supporting each part of it.

Then answer here, under these rules:

- The first sentence answers the question. Nothing precedes it.
- At most three sentences before any elaboration, and elaborate only if the
  answer is wrong or misleading without it.
- Every claim about the code carries a `path:line`.
- Use only identifiers that appear literally in the codebase. If you need a
  term the code does not define, mark it: `the retry path (my term)`.
- If the subagent did not find enough to answer, say what is missing. Do not
  infer behavior from naming conventions.
- Do not list related code I did not ask about.
