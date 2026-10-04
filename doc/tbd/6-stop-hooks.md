# Add stop hooks for Claude Code

Force Claude to comply with basic restrictions on:

- Reply length -- Progress: `../../etc/claude/hooks/terse.py`
- Use of bullets, headers, or other non-prose
- Use of "id" to mean "ID"
- Direct calls to Git rather than JJ
- Use of banned tools (if any; awk, sed?)
- Banner comments
- Code paragraphs

## Updates

### 2026-10-04

Stop hooks on reply length are working; the rest of these will be Whac-a-Mole.
