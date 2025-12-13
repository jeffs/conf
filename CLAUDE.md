# conf Development Guidelines

Auto-generated from all feature plans. Last updated: 2025-12-12

## Active Technologies
- CSV files (`~/.config/jump/targets.csv` or `$JUMP_PREFIXES`) (002-generalize-jump-targets)

- Rust 2024 edition (nightly features) + chrono 0.4.40 (retained for date expansion in paths) (001-remove-jump-command-mode)

## Project Structure

```text
src/
tests/
```

## Commands

cargo test [ONLY COMMANDS FOR ACTIVE TECHNOLOGIES][ONLY COMMANDS FOR ACTIVE TECHNOLOGIES] cargo clippy

## Code Style

Rust 2024 edition (nightly features): Follow standard conventions

## Recent Changes
- 002-generalize-jump-targets: Added Rust 2024 edition (nightly features) + chrono 0.4.40 (retained for date expansion in paths)

- 001-remove-jump-command-mode: Added Rust 2024 edition (nightly features) + chrono 0.4.40 (retained for date expansion in paths)

<!-- MANUAL ADDITIONS START -->

## Stylistic Guidance

### Functional Naming
Name methods as English instructions, not OOP message-passing:
- `Expand::path` ("expand path") not `Expand::expand_path`
- `Expand::target` ("expand target") not `Expand::resolve_target`

### Avoid Trivial Helpers
Don't create single-use helper functions for simple logic. Inline it:
```rust
// Prefer: inline simple checks
if value.starts_with(['/', '~', '$', '%']) { ... }

// Avoid: separate helper for one-time use
fn is_path_target(s: &str) -> bool { ... }
```

This applies to CLI helpers too:
```rust
// Prefer: inline the message
eprintln!("usage: jump TARGET");

// Avoid: single-use function
fn usage() { eprintln!("usage: jump TARGET"); }
usage();
```

### Error Message Style
- Use `error:` prefix, not the program name
- Lowercase for usage: `usage: prog ARG`
- ALLCAPS for metavariables: `TARGET` not `<target>`

```rust
eprintln!("error: {err}");
eprintln!("usage: jump TARGET");
```

### Use Array Patterns
```rust
// Prefer
s.starts_with(['/', '~', '$', '%'])

// Avoid
s.starts_with('/') || s.starts_with('~') || s.starts_with('$') || s.starts_with('%')
```

### Type Locality
Keep types near their primary producer/consumer. Re-export from lib.rs if needed:
```rust
// In expansion.rs: define Target alongside Expand
pub enum Target { ... }

// In lib.rs: re-export for public API
pub use expansion::{Expand, Target};
```

### Simplify Detection Logic
Let values fall through naturally instead of explicit checks:
- URLs don't start with `/`, `~`, `$`, `%` → handled by else branch
- No need for separate `is_url_target()` check

<!-- MANUAL ADDITIONS END -->
