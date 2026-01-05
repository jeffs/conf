---
name: refresh-rust-expert
description: Update the rust-expert agent with the latest Rust idioms, features, and best practices from official sources. Run this periodically to keep Rust advice current.
allowed-tools: WebSearch, WebFetch, Read, Write, Glob
user-invocable: true
---

# Refresh Rust Expert

This skill updates the `~/.claude/agents/rust-expert.md` agent with current Rust knowledge.

## Your Task

When invoked, perform these steps:

### 1. Fetch Current Rust Information

Search for and fetch the latest information from official sources:

```
WebFetch: https://blog.rust-lang.org/releases/  (get the LATEST version number first!)
WebFetch: https://blog.rust-lang.org/YYYY/MM/DD/Rust-X.YZ.0/ (for each recent release)
WebSearch: "rust edition guide site:doc.rust-lang.org {current_year}"
```

**IMPORTANT**: Always fetch the releases index first to identify the current stable version. Do not assume you know the latest version - verify it. Fetch release notes for ALL versions from the last known version in the agent file up to current.

Focus on:
- **Recent stable releases** - New features, stabilizations, deprecations
- **Current idioms** - Modern formatting (e.g., `println!("{value}")` not `println!("{}", value)`)
- **Error handling patterns** - Current best practices for Result/Option
- **Async/await updates** - Latest async patterns and stabilizations
- **Popular crates** - Current recommendations for common tasks

### 2. Read the Current Agent

Read `~/.claude/agents/rust-expert.md` to understand its current state.

### 3. Update the Agent

Update the rust-expert.md file with a new section called `## Current Rust Knowledge (Auto-Updated)` that includes:

- **Last updated**: Today's date
- **Latest stable Rust version**: From release notes
- **Recent notable features**: Last 2-3 releases worth of important changes
- **Current idioms**: Modern syntax preferences with examples
- **Deprecated patterns**: Things to avoid in modern Rust

Also fix any outdated examples in the existing content (e.g., update `println!("{}", x)` to `println!("{x}")`).

### 4. Verify No Regressions

After updating, review the file to ensure:
- YAML frontmatter is intact
- No syntax errors in code examples
- Examples compile (no borrowing mistakes like `&String::from(...)`)

### 5. Report Changes

Summarize what was updated and any notable new Rust features the agent should now know about.

## Example Output

After running, the rust-expert.md should have updated examples and a section like:

```markdown
## Current Rust Knowledge (Auto-Updated)

**Last updated**: 2025-01-05
**Latest stable**: Rust 1.84.0

### Recent Features
- 1.84: Feature X stabilized
- 1.83: Feature Y added
- 1.82: Feature Z improved

### Modern Idioms
- Use `println!("{value}")` not `println!("{}", value)` (format string captures)
- Use `let else` for early returns: `let Some(x) = opt else { return }`
- Prefer `std::array::from_fn` over collect for fixed arrays

### Patterns to Avoid
- `&String::from("...")` in `unwrap_or` - creates dangling reference
- Raw `Arc<Mutex<...>>` without considering ownership redesign
```
