# Implementation Notes: Generalize Jump Targets

Post-implementation cleanup (commit 242dffe3) captured key stylistic improvements.

## Lessons Learned

### 1. Functional Naming

Read method names as English instructions, not OOP message-passing:

- ✅ `Expand::path` → "expand path"
- ✅ `Expand::target` → "expand target"
- ❌ `Expand::expand_path` → redundant OOP style

### 2. Eliminate Trivial Helpers

Don't create single-use helper functions for simple logic:

```rust
// ❌ Before: separate helper functions
pub fn is_url_target(s: &str) -> bool {
    s.starts_with("http://") || s.starts_with("https://")
}
pub fn is_path_target(s: &str) -> bool {
    s.starts_with('/') || s.starts_with('~') || ...
}

// ✅ After: inline the logic
if value.starts_with(['/', '~', '$', '%']) {
    Ok(Target::Path(...))
} else {
    Ok(Target::String(...))
}
```

### 3. Simplify Detection Logic

URLs don't need explicit detection if they naturally fall through:

- URLs don't start with `/`, `~`, `$`, or `%`
- Therefore, they're handled by the `else` branch as strings
- Removed unnecessary URL-specific check

### 4. Use Array Patterns

```rust
// ❌ Verbose
s.starts_with('/') || s.starts_with('~') || s.starts_with('$') || s.starts_with('%')

// ✅ Concise
value.starts_with(['/', '~', '$', '%'])
```

### 5. Locality of Types

Keep related types together:

- `Target` enum moved from `lib.rs` to `expansion.rs`
- Co-located with `Expand` which produces it
- Re-exported via `pub use expansion::{Expand, Target}`

### 6. Doc Comments

- Reference related methods with `Self::method`
- Be precise about error conditions
- Document panics separately from errors
